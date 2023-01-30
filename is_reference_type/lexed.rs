Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0c08fd1d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
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
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0c08fd1d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0c08fd1d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                ),
                                                start: 24,
                                                end: 26,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 26,
                                                    end: 32,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 32,
                                        end: 33,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 41,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 42,
                                        end: 43,
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 50,
                                                                end: 51,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 52,
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 53,
                                                                            end: 56,
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
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 56,
                                                        end: 57,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 44,
                                        end: 59,
                                        as_str(): "{\n    a: u64,\n}",
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 65,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 66,
                                        end: 67,
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 74,
                                                                end: 81,
                                                                as_str(): "Variant",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 81,
                                                                end: 82,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 83,
                                                                    end: 85,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 85,
                                                        end: 86,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 68,
                                        end: 88,
                                        as_str(): "{\n    Variant: (),\n}",
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
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 90,
                                            end: 92,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 93,
                                            end: 109,
                                            as_str(): "arg_is_reference",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 109,
                                                        end: 110,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 111,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 111,
                                                        end: 112,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
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
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 113,
                                                                    end: 114,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 114,
                                                                end: 115,
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 116,
                                                                            end: 117,
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 112,
                                            end: 118,
                                            as_str(): "(a: T)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 121,
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 126,
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
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                    ),
                                                                    start: 133,
                                                                    end: 152,
                                                                    as_str(): "__is_reference_type",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: Some(
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 152,
                                                                            end: 154,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    GenericArgs {
                                                                        parameters: AngleBrackets {
                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 154,
                                                                                    end: 155,
                                                                                    as_str(): "<",
                                                                                },
                                                                            },
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
                                                                                    Path(
                                                                                        PathType {
                                                                                            root_opt: None,
                                                                                            prefix: PathTypeSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 155,
                                                                                                        end: 156,
                                                                                                        as_str(): "T",
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
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 156,
                                                                                    end: 157,
                                                                                    as_str(): ">",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
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
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 157,
                                                        end: 159,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 127,
                                        end: 161,
                                        as_str(): "{\n    __is_reference_type::<T>()\n}",
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
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 163,
                                            end: 165,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 166,
                                            end: 170,
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
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
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
                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
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
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 187,
                                                                        end: 193,
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
                                                                Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 194,
                                                                            end: 195,
                                                                            as_str(): "!",
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 195,
                                                                                            end: 214,
                                                                                            as_str(): "__is_reference_type",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: Some(
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 214,
                                                                                                    end: 216,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            GenericArgs {
                                                                                                parameters: AngleBrackets {
                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 216,
                                                                                                            end: 217,
                                                                                                            as_str(): "<",
                                                                                                        },
                                                                                                    },
                                                                                                    inner: Punctuated {
                                                                                                        value_separator_pairs: [],
                                                                                                        final_value_opt: Some(
                                                                                                            Tuple(
                                                                                                                Parens {
                                                                                                                    inner: Nil,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 217,
                                                                                                                        end: 219,
                                                                                                                        as_str(): "()",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 219,
                                                                                                            end: 220,
                                                                                                            as_str(): ">",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
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
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 220,
                                                                                end: 222,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 193,
                                                            end: 223,
                                                            as_str(): "(!__is_reference_type::<()>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 223,
                                                            end: 224,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 259,
                                                                        end: 265,
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
                                                                Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 266,
                                                                            end: 267,
                                                                            as_str(): "!",
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 267,
                                                                                            end: 286,
                                                                                            as_str(): "__is_reference_type",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: Some(
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 286,
                                                                                                    end: 288,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            GenericArgs {
                                                                                                parameters: AngleBrackets {
                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 288,
                                                                                                            end: 289,
                                                                                                            as_str(): "<",
                                                                                                        },
                                                                                                    },
                                                                                                    inner: Punctuated {
                                                                                                        value_separator_pairs: [],
                                                                                                        final_value_opt: Some(
                                                                                                            Path(
                                                                                                                PathType {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathTypeSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 289,
                                                                                                                                end: 293,
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
                                                                                                    },
                                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 293,
                                                                                                            end: 294,
                                                                                                            as_str(): ">",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
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
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 294,
                                                                                end: 296,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 297,
                                                            as_str(): "(!__is_reference_type::<bool>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 297,
                                                            end: 298,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 303,
                                                                        end: 309,
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
                                                                Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 310,
                                                                            end: 311,
                                                                            as_str(): "!",
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 311,
                                                                                            end: 330,
                                                                                            as_str(): "__is_reference_type",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: Some(
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 330,
                                                                                                    end: 332,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            GenericArgs {
                                                                                                parameters: AngleBrackets {
                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 332,
                                                                                                            end: 333,
                                                                                                            as_str(): "<",
                                                                                                        },
                                                                                                    },
                                                                                                    inner: Punctuated {
                                                                                                        value_separator_pairs: [],
                                                                                                        final_value_opt: Some(
                                                                                                            Path(
                                                                                                                PathType {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathTypeSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 333,
                                                                                                                                end: 336,
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
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 336,
                                                                                                            end: 337,
                                                                                                            as_str(): ">",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
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
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 337,
                                                                                end: 339,
                                                                                as_str(): "()",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 309,
                                                            end: 340,
                                                            as_str(): "(!__is_reference_type::<u64>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 340,
                                                            end: 341,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 347,
                                                                        end: 353,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 354,
                                                                                        end: 373,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: Some(
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 373,
                                                                                                end: 375,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        GenericArgs {
                                                                                            parameters: AngleBrackets {
                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 375,
                                                                                                        end: 376,
                                                                                                        as_str(): "<",
                                                                                                    },
                                                                                                },
                                                                                                inner: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Str {
                                                                                                            str_token: StrToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 376,
                                                                                                                    end: 379,
                                                                                                                    as_str(): "str",
                                                                                                                },
                                                                                                            },
                                                                                                            length: SquareBrackets {
                                                                                                                inner: Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 380,
                                                                                                                                end: 381,
                                                                                                                                as_str(): "1",
                                                                                                                            },
                                                                                                                            parsed: 1,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 379,
                                                                                                                    end: 382,
                                                                                                                    as_str(): "[1]",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 382,
                                                                                                        end: 383,
                                                                                                        as_str(): ">",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 383,
                                                                            end: 385,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 353,
                                                            end: 386,
                                                            as_str(): "(__is_reference_type::<str[1]>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 386,
                                                            end: 387,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 392,
                                                                        end: 398,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 399,
                                                                                        end: 418,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: Some(
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 418,
                                                                                                end: 420,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        GenericArgs {
                                                                                            parameters: AngleBrackets {
                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 420,
                                                                                                        end: 421,
                                                                                                        as_str(): "<",
                                                                                                    },
                                                                                                },
                                                                                                inner: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Path(
                                                                                                            PathType {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathTypeSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 421,
                                                                                                                            end: 425,
                                                                                                                            as_str(): "b256",
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
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 425,
                                                                                                        end: 426,
                                                                                                        as_str(): ">",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 426,
                                                                            end: 428,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 398,
                                                            end: 429,
                                                            as_str(): "(__is_reference_type::<b256>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 429,
                                                            end: 430,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 435,
                                                                        end: 441,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 442,
                                                                                        end: 461,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: Some(
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 461,
                                                                                                end: 463,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        GenericArgs {
                                                                                            parameters: AngleBrackets {
                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 463,
                                                                                                        end: 464,
                                                                                                        as_str(): "<",
                                                                                                    },
                                                                                                },
                                                                                                inner: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Path(
                                                                                                            PathType {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathTypeSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 464,
                                                                                                                            end: 465,
                                                                                                                            as_str(): "S",
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
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 465,
                                                                                                        end: 466,
                                                                                                        as_str(): ">",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 466,
                                                                            end: 468,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 441,
                                                            end: 469,
                                                            as_str(): "(__is_reference_type::<S>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 469,
                                                            end: 470,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 475,
                                                                        end: 481,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 482,
                                                                                        end: 501,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: Some(
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 501,
                                                                                                end: 503,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        GenericArgs {
                                                                                            parameters: AngleBrackets {
                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 503,
                                                                                                        end: 504,
                                                                                                        as_str(): "<",
                                                                                                    },
                                                                                                },
                                                                                                inner: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Path(
                                                                                                            PathType {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathTypeSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 504,
                                                                                                                            end: 505,
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
                                                                                                },
                                                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 505,
                                                                                                        end: 506,
                                                                                                        as_str(): ">",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 506,
                                                                            end: 508,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 481,
                                                            end: 509,
                                                            as_str(): "(__is_reference_type::<E>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 509,
                                                            end: 510,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 515,
                                                                        end: 521,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 522,
                                                                                        end: 541,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: Some(
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 541,
                                                                                                end: 543,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        GenericArgs {
                                                                                            parameters: AngleBrackets {
                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 543,
                                                                                                        end: 544,
                                                                                                        as_str(): "<",
                                                                                                    },
                                                                                                },
                                                                                                inner: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Tuple(
                                                                                                            Parens {
                                                                                                                inner: Cons {
                                                                                                                    head: Path(
                                                                                                                        PathType {
                                                                                                                            root_opt: None,
                                                                                                                            prefix: PathTypeSegment {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 545,
                                                                                                                                        end: 549,
                                                                                                                                        as_str(): "bool",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                generics_opt: None,
                                                                                                                            },
                                                                                                                            suffix: [],
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    comma_token: CommaToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 549,
                                                                                                                            end: 550,
                                                                                                                            as_str(): ",",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    tail: Punctuated {
                                                                                                                        value_separator_pairs: [],
                                                                                                                        final_value_opt: Some(
                                                                                                                            Path(
                                                                                                                                PathType {
                                                                                                                                    root_opt: None,
                                                                                                                                    prefix: PathTypeSegment {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 551,
                                                                                                                                                end: 555,
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
                                                                                                                    },
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 544,
                                                                                                                    end: 556,
                                                                                                                    as_str(): "(bool, bool)",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 556,
                                                                                                        end: 557,
                                                                                                        as_str(): ">",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 557,
                                                                            end: 559,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 521,
                                                            end: 560,
                                                            as_str(): "(__is_reference_type::<(bool, bool)>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 560,
                                                            end: 561,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 566,
                                                                        end: 572,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 573,
                                                                                        end: 592,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: Some(
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 592,
                                                                                                end: 594,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        GenericArgs {
                                                                                            parameters: AngleBrackets {
                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 594,
                                                                                                        end: 595,
                                                                                                        as_str(): "<",
                                                                                                    },
                                                                                                },
                                                                                                inner: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
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
                                                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 596,
                                                                                                                                        end: 599,
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
                                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 599,
                                                                                                                            end: 600,
                                                                                                                            as_str(): ";",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    length: Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 601,
                                                                                                                                    end: 602,
                                                                                                                                    as_str(): "2",
                                                                                                                                },
                                                                                                                                parsed: 2,
                                                                                                                                ty_opt: None,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 595,
                                                                                                                    end: 603,
                                                                                                                    as_str(): "[u64; 2]",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 603,
                                                                                                        end: 604,
                                                                                                        as_str(): ">",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 604,
                                                                            end: 606,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 572,
                                                            end: 607,
                                                            as_str(): "(__is_reference_type::<[u64; 2]>())",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 607,
                                                            end: 608,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 614,
                                                                        end: 620,
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
                                                                Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 621,
                                                                            end: 622,
                                                                            as_str(): "!",
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 622,
                                                                                            end: 638,
                                                                                            as_str(): "arg_is_reference",
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
                                                                                    Tuple(
                                                                                        Parens {
                                                                                            inner: Nil,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 639,
                                                                                                end: 641,
                                                                                                as_str(): "()",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 638,
                                                                                end: 642,
                                                                                as_str(): "(())",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 620,
                                                            end: 643,
                                                            as_str(): "(!arg_is_reference(()))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 643,
                                                            end: 644,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 649,
                                                                        end: 655,
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
                                                                Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 656,
                                                                            end: 657,
                                                                            as_str(): "!",
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 657,
                                                                                            end: 673,
                                                                                            as_str(): "arg_is_reference",
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
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 674,
                                                                                                    end: 679,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 673,
                                                                                end: 680,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 655,
                                                            end: 681,
                                                            as_str(): "(!arg_is_reference(false))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 681,
                                                            end: 682,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 687,
                                                                        end: 693,
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
                                                                Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 694,
                                                                            end: 695,
                                                                            as_str(): "!",
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 695,
                                                                                            end: 711,
                                                                                            as_str(): "arg_is_reference",
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
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 712,
                                                                                                    end: 716,
                                                                                                    as_str(): "0x2b",
                                                                                                },
                                                                                                parsed: 43,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 711,
                                                                                end: 717,
                                                                                as_str(): "(0x2b)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 693,
                                                            end: 718,
                                                            as_str(): "(!arg_is_reference(0x2b))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 718,
                                                            end: 719,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 724,
                                                                        end: 730,
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
                                                                Not {
                                                                    bang_token: BangToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 731,
                                                                            end: 732,
                                                                            as_str(): "!",
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 732,
                                                                                            end: 748,
                                                                                            as_str(): "arg_is_reference",
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
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 749,
                                                                                                    end: 750,
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
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 748,
                                                                                end: 751,
                                                                                as_str(): "(0)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 730,
                                                            end: 752,
                                                            as_str(): "(!arg_is_reference(0))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 752,
                                                            end: 753,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 759,
                                                                        end: 765,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 766,
                                                                                        end: 782,
                                                                                        as_str(): "arg_is_reference",
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
                                                                                    String(
                                                                                        LitString {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 783,
                                                                                                end: 794,
                                                                                                as_str(): "\"breakfast\"",
                                                                                            },
                                                                                            parsed: "breakfast",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 782,
                                                                            end: 795,
                                                                            as_str(): "(\"breakfast\")",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 765,
                                                            end: 796,
                                                            as_str(): "(arg_is_reference(\"breakfast\"))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 796,
                                                            end: 797,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 802,
                                                                        end: 808,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 809,
                                                                                        end: 825,
                                                                                        as_str(): "arg_is_reference",
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
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 826,
                                                                                                end: 892,
                                                                                                as_str(): "0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe",
                                                                                            },
                                                                                            parsed: 115338002612856131912106000753751876842080690608128248102047946423568372268798,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 825,
                                                                            end: 893,
                                                                            as_str(): "(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe)",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 808,
                                                            end: 894,
                                                            as_str(): "(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 894,
                                                            end: 895,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 900,
                                                                        end: 906,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 907,
                                                                                        end: 923,
                                                                                        as_str(): "arg_is_reference",
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
                                                                                Struct {
                                                                                    path: PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 924,
                                                                                                    end: 925,
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
                                                                                            value_separator_pairs: [],
                                                                                            final_value_opt: Some(
                                                                                                ExprStructField {
                                                                                                    field_name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 928,
                                                                                                            end: 929,
                                                                                                            as_str(): "a",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    expr_opt: Some(
                                                                                                        (
                                                                                                            ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 929,
                                                                                                                    end: 930,
                                                                                                                    as_str(): ":",
                                                                                                                },
                                                                                                            },
                                                                                                            Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 931,
                                                                                                                            end: 933,
                                                                                                                            as_str(): "42",
                                                                                                                        },
                                                                                                                        parsed: 42,
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 926,
                                                                                            end: 935,
                                                                                            as_str(): "{ a: 42 }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 923,
                                                                            end: 936,
                                                                            as_str(): "(S { a: 42 })",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 906,
                                                            end: 937,
                                                            as_str(): "(arg_is_reference(S { a: 42 }))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 937,
                                                            end: 938,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 943,
                                                                        end: 949,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 950,
                                                                                        end: 966,
                                                                                        as_str(): "arg_is_reference",
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
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 967,
                                                                                                    end: 968,
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
                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                        ),
                                                                                                        start: 968,
                                                                                                        end: 970,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 970,
                                                                                                            end: 977,
                                                                                                            as_str(): "Variant",
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
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 966,
                                                                            end: 978,
                                                                            as_str(): "(E::Variant)",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 949,
                                                            end: 979,
                                                            as_str(): "(arg_is_reference(E::Variant))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 979,
                                                            end: 980,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 985,
                                                                        end: 991,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 992,
                                                                                        end: 1008,
                                                                                        as_str(): "arg_is_reference",
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
                                                                                Tuple(
                                                                                    Parens {
                                                                                        inner: Cons {
                                                                                            head: Literal(
                                                                                                Bool(
                                                                                                    LitBool {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1010,
                                                                                                            end: 1014,
                                                                                                            as_str(): "true",
                                                                                                        },
                                                                                                        kind: True,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                            comma_token: CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1014,
                                                                                                    end: 1015,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                            tail: Punctuated {
                                                                                                value_separator_pairs: [],
                                                                                                final_value_opt: Some(
                                                                                                    Literal(
                                                                                                        Bool(
                                                                                                            LitBool {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1016,
                                                                                                                    end: 1020,
                                                                                                                    as_str(): "true",
                                                                                                                },
                                                                                                                kind: True,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1009,
                                                                                            end: 1021,
                                                                                            as_str(): "(true, true)",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 1008,
                                                                            end: 1022,
                                                                            as_str(): "((true, true))",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 991,
                                                            end: 1023,
                                                            as_str(): "(arg_is_reference((true, true)))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 1023,
                                                            end: 1024,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1029,
                                                                        end: 1035,
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 1036,
                                                                                        end: 1052,
                                                                                        as_str(): "arg_is_reference",
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
                                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1054,
                                                                                                                        end: 1055,
                                                                                                                        as_str(): "5",
                                                                                                                    },
                                                                                                                    parsed: 5,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1055,
                                                                                                                end: 1056,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1057,
                                                                                                                        end: 1058,
                                                                                                                        as_str(): "4",
                                                                                                                    },
                                                                                                                    parsed: 4,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1058,
                                                                                                                end: 1059,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1060,
                                                                                                                        end: 1061,
                                                                                                                        as_str(): "3",
                                                                                                                    },
                                                                                                                    parsed: 3,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1061,
                                                                                                                end: 1062,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1063,
                                                                                                                        end: 1064,
                                                                                                                        as_str(): "2",
                                                                                                                    },
                                                                                                                    parsed: 2,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1064,
                                                                                                                end: 1065,
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
                                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1066,
                                                                                                                    end: 1067,
                                                                                                                    as_str(): "1",
                                                                                                                },
                                                                                                                parsed: 1,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1053,
                                                                                            end: 1068,
                                                                                            as_str(): "[5, 4, 3, 2, 1]",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 1052,
                                                                            end: 1069,
                                                                            as_str(): "([5, 4, 3, 2, 1])",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 1035,
                                                            end: 1070,
                                                            as_str(): "(arg_is_reference([5, 4, 3, 2, 1]))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 1070,
                                                            end: 1071,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 1077,
                                                            end: 1081,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 181,
                                        end: 1083,
                                        as_str(): "{\n    assert(!__is_reference_type::<()>());        // Is Unit ref or not?\n    assert(!__is_reference_type::<bool>());\n    assert(!__is_reference_type::<u64>());\n\n    assert(__is_reference_type::<str[1]>());\n    assert(__is_reference_type::<b256>());\n    assert(__is_reference_type::<S>());\n    assert(__is_reference_type::<E>());\n    assert(__is_reference_type::<(bool, bool)>());\n    assert(__is_reference_type::<[u64; 2]>());\n\n    assert(!arg_is_reference(()));\n    assert(!arg_is_reference(false));\n    assert(!arg_is_reference(0x2b));\n    assert(!arg_is_reference(0));\n\n    assert(arg_is_reference(\"breakfast\"));\n    assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe));\n    assert(arg_is_reference(S { a: 42 }));\n    assert(arg_is_reference(E::Variant));\n    assert(arg_is_reference((true, true)));\n    assert(arg_is_reference([5, 4, 3, 2, 1]));\n\n    true\n}",
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
