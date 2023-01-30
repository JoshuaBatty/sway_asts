Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb0f64c4390,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Group {
                                        imports: Braces {
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 19,
                                                                    end: 26,
                                                                    as_str(): "address",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 26,
                                                                    end: 28,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 28,
                                                                        end: 35,
                                                                        as_str(): "Address",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 36,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 37,
                                                                    end: 43,
                                                                    as_str(): "assert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 43,
                                                                    end: 45,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 45,
                                                                        end: 51,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 52,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    Path {
                                                        prefix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 53,
                                                                end: 61,
                                                                as_str(): "identity",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 63,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 71,
                                                                    as_str(): "Identity",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                ),
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 72,
                                                as_str(): "{address::Address, assert::assert, identity::Identity}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 72,
                                        end: 73,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 75,
                                        end: 79,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 80,
                                        end: 86,
                                        as_str(): "Result",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 86,
                                                    end: 87,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 87,
                                                                end: 88,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 88,
                                                                end: 89,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 91,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 91,
                                                    end: 92,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
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
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 99,
                                                                end: 101,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 101,
                                                                end: 102,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 103,
                                                                            end: 104,
                                                                            as_str(): "T",
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
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 104,
                                                        end: 105,
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
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 113,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 113,
                                                                end: 114,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 115,
                                                                            end: 116,
                                                                            as_str(): "E",
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
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 116,
                                                        end: 117,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 93,
                                        end: 119,
                                        as_str(): "{\n    Ok: T,\n    Err: E,\n}",
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
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 121,
                                        end: 126,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 127,
                                        end: 129,
                                        as_str(): "B1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 130,
                                        end: 131,
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
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 132,
                                                    end: 139,
                                                    as_str(): "Address",
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
                                            value_separator_pairs: [],
                                            final_value_opt: Some(
                                                ExprStructField {
                                                    field_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 146,
                                                            end: 151,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    expr_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 151,
                                                                    end: 152,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 153,
                                                                            end: 219,
                                                                            as_str(): "0x0100000000000000000000000000000000000000000000000000000000000010",
                                                                        },
                                                                        parsed: 452312848583266388373324160190187140051835877600158453279131187530910662672,
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
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 140,
                                            end: 221,
                                            as_str(): "{\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n}",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 221,
                                        end: 222,
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
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 224,
                                            end: 226,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 227,
                                            end: 231,
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
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 231,
                                            end: 233,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 234,
                                                    end: 236,
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
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 237,
                                                                end: 240,
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
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 247,
                                                            end: 250,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 251,
                                                                end: 252,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 253,
                                                            end: 254,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 255,
                                                                            end: 261,
                                                                            as_str(): "Result",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 261,
                                                                                end: 263,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 263,
                                                                                    end: 265,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: Some(
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 265,
                                                                                            end: 267,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    GenericArgs {
                                                                                        parameters: AngleBrackets {
                                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 267,
                                                                                                    end: 268,
                                                                                                    as_str(): "<",
                                                                                                },
                                                                                            },
                                                                                            inner: Punctuated {
                                                                                                value_separator_pairs: [
                                                                                                    (
                                                                                                        Path(
                                                                                                            PathType {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathTypeSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 268,
                                                                                                                            end: 271,
                                                                                                                            as_str(): "u64",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                            },
                                                                                                        ),
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 271,
                                                                                                                end: 272,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: Some(
                                                                                                    Path(
                                                                                                        PathType {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathTypeSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 273,
                                                                                                                        end: 276,
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
                                                                                            },
                                                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 276,
                                                                                                    end: 277,
                                                                                                    as_str(): ">",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
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
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 278,
                                                                                    end: 281,
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
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 277,
                                                                end: 282,
                                                                as_str(): "(100)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 282,
                                                            end: 283,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 288,
                                                            end: 291,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 292,
                                                                end: 293,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 294,
                                                            end: 295,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 296,
                                                                    end: 298,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Let {
                                                                let_token: LetToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 299,
                                                                        end: 302,
                                                                        as_str(): "let",
                                                                    },
                                                                },
                                                                lhs: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 303,
                                                                                    end: 309,
                                                                                    as_str(): "Result",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 309,
                                                                                        end: 311,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 311,
                                                                                            end: 313,
                                                                                            as_str(): "Ok",
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
                                                                                Var {
                                                                                    reference: None,
                                                                                    mutable: None,
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 314,
                                                                                            end: 315,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 313,
                                                                            end: 316,
                                                                            as_str(): "(y)",
                                                                        },
                                                                    },
                                                                },
                                                                eq_token: EqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 317,
                                                                        end: 318,
                                                                        as_str(): "=",
                                                                    },
                                                                },
                                                                rhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 319,
                                                                                    end: 320,
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
                                                            },
                                                            then_block: Braces {
                                                                inner: CodeBlockContents {
                                                                    statements: [],
                                                                    final_expr_opt: Some(
                                                                        Add {
                                                                            lhs: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 323,
                                                                                                end: 324,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [],
                                                                                    incomplete_suffix: false,
                                                                                },
                                                                            ),
                                                                            add_token: AddToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 325,
                                                                                    end: 326,
                                                                                    as_str(): "+",
                                                                                },
                                                                            },
                                                                            rhs: Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 327,
                                                                                            end: 329,
                                                                                            as_str(): "10",
                                                                                        },
                                                                                        parsed: 10,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 321,
                                                                    end: 331,
                                                                    as_str(): "{ y + 10 }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 332,
                                                                            end: 336,
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
                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 339,
                                                                                                    end: 340,
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
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 337,
                                                                                end: 342,
                                                                                as_str(): "{ 1 }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 342,
                                                            end: 343,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 348,
                                                                        end: 354,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 355,
                                                                                        end: 356,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 357,
                                                                            end: 359,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 360,
                                                                                    end: 363,
                                                                                    as_str(): "110",
                                                                                },
                                                                                parsed: 110,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 364,
                                                            as_str(): "(b == 110)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 364,
                                                            end: 365,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 371,
                                                            end: 374,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 375,
                                                                end: 381,
                                                                as_str(): "sender",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 382,
                                                            end: 383,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 384,
                                                                            end: 392,
                                                                            as_str(): "Identity",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 392,
                                                                                end: 394,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 401,
                                                                                    as_str(): "Address",
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
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 402,
                                                                                        end: 404,
                                                                                        as_str(): "B1",
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
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 401,
                                                                end: 405,
                                                                as_str(): "(B1)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 405,
                                                            end: 406,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: If(
                                                    IfExpr {
                                                        if_token: IfToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 411,
                                                                end: 413,
                                                                as_str(): "if",
                                                            },
                                                        },
                                                        condition: Let {
                                                            let_token: LetToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 414,
                                                                    end: 417,
                                                                    as_str(): "let",
                                                                },
                                                            },
                                                            lhs: Constructor {
                                                                path: PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 418,
                                                                                end: 426,
                                                                                as_str(): "Identity",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 426,
                                                                                    end: 428,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 428,
                                                                                        end: 435,
                                                                                        as_str(): "Address",
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
                                                                            Var {
                                                                                reference: None,
                                                                                mutable: None,
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 436,
                                                                                        end: 441,
                                                                                        as_str(): "addr1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 435,
                                                                        end: 442,
                                                                        as_str(): "(addr1)",
                                                                    },
                                                                },
                                                            },
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 443,
                                                                    end: 444,
                                                                    as_str(): "=",
                                                                },
                                                            },
                                                            rhs: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 445,
                                                                                end: 451,
                                                                                as_str(): "sender",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                            ),
                                                        },
                                                        then_block: Braces {
                                                            inner: CodeBlockContents {
                                                                statements: [],
                                                                final_expr_opt: Some(
                                                                    Match {
                                                                        match_token: MatchToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 462,
                                                                                end: 467,
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
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 468,
                                                                                            end: 474,
                                                                                            as_str(): "sender",
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
                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 489,
                                                                                                        end: 497,
                                                                                                        as_str(): "Identity",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 497,
                                                                                                            end: 499,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 499,
                                                                                                                end: 506,
                                                                                                                as_str(): "Address",
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
                                                                                                    Var {
                                                                                                        reference: None,
                                                                                                        mutable: None,
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 507,
                                                                                                                end: 512,
                                                                                                                as_str(): "addr2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 506,
                                                                                                end: 513,
                                                                                                as_str(): "(addr2)",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 514,
                                                                                            end: 516,
                                                                                            as_str(): "=>",
                                                                                        },
                                                                                    },
                                                                                    kind: Block {
                                                                                        block: Braces {
                                                                                            inner: CodeBlockContents {
                                                                                                statements: [
                                                                                                    Expr {
                                                                                                        expr: FuncApp {
                                                                                                            func: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 535,
                                                                                                                                end: 541,
                                                                                                                                as_str(): "assert",
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
                                                                                                                        Equal {
                                                                                                                            lhs: Path(
                                                                                                                                PathExpr {
                                                                                                                                    root_opt: None,
                                                                                                                                    prefix: PathExprSegment {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 542,
                                                                                                                                                end: 547,
                                                                                                                                                as_str(): "addr1",
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
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 548,
                                                                                                                                    end: 550,
                                                                                                                                    as_str(): "==",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            rhs: Path(
                                                                                                                                PathExpr {
                                                                                                                                    root_opt: None,
                                                                                                                                    prefix: PathExprSegment {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 551,
                                                                                                                                                end: 556,
                                                                                                                                                as_str(): "addr2",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        generics_opt: None,
                                                                                                                                    },
                                                                                                                                    suffix: [],
                                                                                                                                    incomplete_suffix: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        },
                                                                                                                    ),
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 541,
                                                                                                                    end: 557,
                                                                                                                    as_str(): "(addr1 == addr2)",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                        semicolon_token_opt: Some(
                                                                                                            SemicolonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 557,
                                                                                                                    end: 558,
                                                                                                                    as_str(): ";",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                ],
                                                                                                final_expr_opt: None,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 517,
                                                                                                end: 572,
                                                                                                as_str(): "{\n                assert(addr1 == addr2);\n            }",
                                                                                            },
                                                                                        },
                                                                                        comma_token_opt: None,
                                                                                    },
                                                                                },
                                                                                MatchBranch {
                                                                                    pattern: Wildcard {
                                                                                        underscore_token: UnderscoreToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 585,
                                                                                                end: 586,
                                                                                                as_str(): "_",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                    fat_right_arrow_token: FatRightArrowToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 587,
                                                                                            end: 589,
                                                                                            as_str(): "=>",
                                                                                        },
                                                                                    },
                                                                                    kind: Block {
                                                                                        block: Braces {
                                                                                            inner: CodeBlockContents {
                                                                                                statements: [
                                                                                                    Expr {
                                                                                                        expr: FuncApp {
                                                                                                            func: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 608,
                                                                                                                                end: 614,
                                                                                                                                as_str(): "assert",
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
                                                                                                                        Literal(
                                                                                                                            Bool(
                                                                                                                                LitBool {
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 615,
                                                                                                                                        end: 620,
                                                                                                                                        as_str(): "false",
                                                                                                                                    },
                                                                                                                                    kind: False,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 614,
                                                                                                                    end: 621,
                                                                                                                    as_str(): "(false)",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                        semicolon_token_opt: Some(
                                                                                                            SemicolonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 621,
                                                                                                                    end: 622,
                                                                                                                    as_str(): ";",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                ],
                                                                                                final_expr_opt: None,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 590,
                                                                                                end: 636,
                                                                                                as_str(): "{\n                assert(false);\n            }",
                                                                                            },
                                                                                        },
                                                                                        comma_token_opt: None,
                                                                                    },
                                                                                },
                                                                            ],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 475,
                                                                                end: 646,
                                                                                as_str(): "{\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 452,
                                                                end: 652,
                                                                as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                                            },
                                                        },
                                                        else_opt: None,
                                                    },
                                                ),
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 653,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 659,
                                                            end: 662,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 663,
                                                                end: 664,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 664,
                                                                    end: 665,
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
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 666,
                                                                                end: 672,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: Some(
                                                                            (
                                                                                None,
                                                                                GenericArgs {
                                                                                    parameters: AngleBrackets {
                                                                                        open_angle_bracket_token: OpenAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 672,
                                                                                                end: 673,
                                                                                                as_str(): "<",
                                                                                            },
                                                                                        },
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [
                                                                                                (
                                                                                                    Path(
                                                                                                        PathType {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathTypeSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 673,
                                                                                                                        end: 676,
                                                                                                                        as_str(): "u64",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                generics_opt: None,
                                                                                                            },
                                                                                                            suffix: [],
                                                                                                        },
                                                                                                    ),
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 676,
                                                                                                            end: 677,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: Some(
                                                                                                Path(
                                                                                                    PathType {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathTypeSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 678,
                                                                                                                    end: 681,
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
                                                                                        },
                                                                                        close_angle_bracket_token: CloseAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 681,
                                                                                                end: 682,
                                                                                                as_str(): ">",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 683,
                                                            end: 684,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 685,
                                                                            end: 691,
                                                                            as_str(): "Result",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 691,
                                                                                end: 693,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 693,
                                                                                    end: 695,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: Some(
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 695,
                                                                                            end: 697,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    GenericArgs {
                                                                                        parameters: AngleBrackets {
                                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 697,
                                                                                                    end: 698,
                                                                                                    as_str(): "<",
                                                                                                },
                                                                                            },
                                                                                            inner: Punctuated {
                                                                                                value_separator_pairs: [
                                                                                                    (
                                                                                                        Path(
                                                                                                            PathType {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathTypeSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 698,
                                                                                                                            end: 701,
                                                                                                                            as_str(): "u64",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                            },
                                                                                                        ),
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 701,
                                                                                                                end: 702,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: Some(
                                                                                                    Path(
                                                                                                        PathType {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathTypeSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 703,
                                                                                                                        end: 706,
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
                                                                                            },
                                                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 706,
                                                                                                    end: 707,
                                                                                                    as_str(): ">",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
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
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 708,
                                                                                    end: 709,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 709,
                                                                                            end: 712,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 707,
                                                                end: 713,
                                                                as_str(): "(5u64)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 713,
                                                            end: 714,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 720,
                                                            end: 723,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 724,
                                                                end: 732,
                                                                as_str(): "result_1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 733,
                                                            end: 734,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 735,
                                                                    end: 737,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Let {
                                                                let_token: LetToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 738,
                                                                        end: 741,
                                                                        as_str(): "let",
                                                                    },
                                                                },
                                                                lhs: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 742,
                                                                                    end: 748,
                                                                                    as_str(): "Result",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 748,
                                                                                        end: 750,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 750,
                                                                                            end: 752,
                                                                                            as_str(): "Ok",
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
                                                                                Var {
                                                                                    reference: None,
                                                                                    mutable: None,
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 753,
                                                                                            end: 754,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 752,
                                                                            end: 755,
                                                                            as_str(): "(x)",
                                                                        },
                                                                    },
                                                                },
                                                                eq_token: EqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 756,
                                                                        end: 757,
                                                                        as_str(): "=",
                                                                    },
                                                                },
                                                                rhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 758,
                                                                                    end: 759,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                ),
                                                            },
                                                            then_block: Braces {
                                                                inner: CodeBlockContents {
                                                                    statements: [],
                                                                    final_expr_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 770,
                                                                                        end: 773,
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
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 760,
                                                                    end: 779,
                                                                    as_str(): "{\n        100\n    }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 780,
                                                                            end: 784,
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
                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 795,
                                                                                                    end: 796,
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
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 785,
                                                                                end: 802,
                                                                                as_str(): "{\n        1\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 802,
                                                            end: 803,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 808,
                                                            end: 811,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 812,
                                                                end: 820,
                                                                as_str(): "result_2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 821,
                                                            end: 822,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 823,
                                                                    end: 825,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Let {
                                                                let_token: LetToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 826,
                                                                        end: 829,
                                                                        as_str(): "let",
                                                                    },
                                                                },
                                                                lhs: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 830,
                                                                                    end: 836,
                                                                                    as_str(): "Result",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 836,
                                                                                        end: 838,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 838,
                                                                                            end: 841,
                                                                                            as_str(): "Err",
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
                                                                                Var {
                                                                                    reference: None,
                                                                                    mutable: None,
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 842,
                                                                                            end: 843,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 841,
                                                                            end: 844,
                                                                            as_str(): "(x)",
                                                                        },
                                                                    },
                                                                },
                                                                eq_token: EqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 845,
                                                                        end: 846,
                                                                        as_str(): "=",
                                                                    },
                                                                },
                                                                rhs: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 847,
                                                                                    end: 848,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                ),
                                                            },
                                                            then_block: Braces {
                                                                inner: CodeBlockContents {
                                                                    statements: [],
                                                                    final_expr_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 859,
                                                                                        end: 860,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 849,
                                                                    end: 866,
                                                                    as_str(): "{\n        3\n    }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 867,
                                                                            end: 871,
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
                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 882,
                                                                                                    end: 884,
                                                                                                    as_str(): "43",
                                                                                                },
                                                                                                parsed: 43,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 872,
                                                                                end: 890,
                                                                                as_str(): "{\n        43\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 890,
                                                            end: 891,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Add {
                                                lhs: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 896,
                                                                    end: 904,
                                                                    as_str(): "result_1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                        incomplete_suffix: false,
                                                    },
                                                ),
                                                add_token: AddToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 905,
                                                        end: 906,
                                                        as_str(): "+",
                                                    },
                                                },
                                                rhs: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 907,
                                                                    end: 915,
                                                                    as_str(): "result_2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                        incomplete_suffix: false,
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 241,
                                        end: 917,
                                        as_str(): "{\n    let a = Result::Ok::<u64, u64>(100);\n    let b = if let Result::Ok(y) = a { y + 10 } else { 1 };\n    assert(b == 110);\n\n    let sender = Identity::Address(B1);\n    if let Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    };\n\n    let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    let result_1 = if let Result::Ok(x) = x {\n        100\n    } else {\n        1\n    };\n    let result_2 = if let Result::Err(x) = x {\n        3\n    } else {\n        43\n    };\n    result_1 + result_2\n}",
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
