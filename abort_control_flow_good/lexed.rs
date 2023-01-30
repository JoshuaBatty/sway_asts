Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb1443204a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
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
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
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
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
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
                                                src (ptr): 0x00007fb1443204a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "revert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb1443204a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
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
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 26,
                                                    end: 32,
                                                    as_str(): "revert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
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
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 39,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                        ),
                                        start: 40,
                                        end: 46,
                                        as_str(): "Result",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 46,
                                                    end: 47,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 47,
                                                                end: 48,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
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
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 50,
                                                            end: 51,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 51,
                                                    end: 52,
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
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 59,
                                                                end: 61,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 62,
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
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 63,
                                                                            end: 64,
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
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 64,
                                                        end: 65,
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
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 70,
                                                                end: 73,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 74,
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
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 75,
                                                                            end: 76,
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
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 76,
                                                        end: 77,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 79,
                                        as_str(): "{\n    Ok: T,\n    Err: E,\n}",
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
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 83,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 84,
                                            end: 95,
                                            as_str(): "local_panic",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 95,
                                                        end: 96,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 96,
                                                                end: 97,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 97,
                                                        end: 98,
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
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 100,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 101,
                                                    end: 103,
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
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 105,
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
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 112,
                                                                    end: 120,
                                                                    as_str(): "__revert",
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
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 121,
                                                                            end: 123,
                                                                            as_str(): "42",
                                                                        },
                                                                        parsed: 42,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 120,
                                                        end: 124,
                                                        as_str(): "(42)",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                        ),
                                        start: 106,
                                        end: 126,
                                        as_str(): "{\n    __revert(42)\n}",
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
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 130,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 131,
                                            end: 135,
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
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 135,
                                            end: 137,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1443204a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                    ),
                                                    start: 138,
                                                    end: 140,
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
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 141,
                                                                end: 144,
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
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 261,
                                                            end: 264,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 265,
                                                                end: 266,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 267,
                                                            end: 268,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 269,
                                                                    end: 271,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Expr(
                                                                Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 272,
                                                                                end: 276,
                                                                                as_str(): "true",
                                                                            },
                                                                            kind: True,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            then_block: Braces {
                                                                inner: CodeBlockContents {
                                                                    statements: [],
                                                                    final_expr_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 279,
                                                                                        end: 281,
                                                                                        as_str(): "42",
                                                                                    },
                                                                                    parsed: 42,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U64,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 281,
                                                                                                end: 284,
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
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 277,
                                                                    end: 286,
                                                                    as_str(): "{ 42u64 }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 287,
                                                                            end: 291,
                                                                            as_str(): "else",
                                                                        },
                                                                    },
                                                                    Break(
                                                                        Braces {
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
                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                            ),
                                                                                                            start: 294,
                                                                                                            end: 300,
                                                                                                            as_str(): "revert",
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
                                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 301,
                                                                                                                    end: 302,
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
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 300,
                                                                                                end: 303,
                                                                                                as_str(): "(0)",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 292,
                                                                                end: 305,
                                                                                as_str(): "{ revert(0) }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 305,
                                                            end: 306,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 311,
                                                            end: 314,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 315,
                                                                end: 316,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 316,
                                                                    end: 317,
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
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 318,
                                                                                end: 321,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 322,
                                                            end: 323,
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
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 324,
                                                                            end: 335,
                                                                            as_str(): "local_panic",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: Some(
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 335,
                                                                                    end: 337,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            GenericArgs {
                                                                                parameters: AngleBrackets {
                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 337,
                                                                                            end: 338,
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
                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                ),
                                                                                                                start: 338,
                                                                                                                end: 341,
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
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 341,
                                                                                            end: 342,
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
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 342,
                                                                end: 344,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 344,
                                                            end: 345,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 350,
                                                            end: 353,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 354,
                                                                end: 355,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 356,
                                                            end: 357,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 358,
                                                                    end: 360,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Let {
                                                                let_token: LetToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 361,
                                                                        end: 364,
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
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 365,
                                                                                    end: 371,
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
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 371,
                                                                                        end: 373,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 373,
                                                                                            end: 375,
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
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 376,
                                                                                            end: 378,
                                                                                            as_str(): "ok",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 375,
                                                                            end: 379,
                                                                            as_str(): "(ok)",
                                                                        },
                                                                    },
                                                                },
                                                                eq_token: EqToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 380,
                                                                        end: 381,
                                                                        as_str(): "=",
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
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 382,
                                                                                        end: 388,
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
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 388,
                                                                                            end: 390,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 390,
                                                                                                end: 392,
                                                                                                as_str(): "Ok",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: Some(
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                        ),
                                                                                                        start: 392,
                                                                                                        end: 394,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                GenericArgs {
                                                                                                    parameters: AngleBrackets {
                                                                                                        open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                ),
                                                                                                                start: 394,
                                                                                                                end: 395,
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
                                                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 395,
                                                                                                                                        end: 398,
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
                                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 398,
                                                                                                                            end: 399,
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
                                                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 400,
                                                                                                                                    end: 403,
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
                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                ),
                                                                                                                start: 403,
                                                                                                                end: 404,
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
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 405,
                                                                                                end: 406,
                                                                                                as_str(): "5",
                                                                                            },
                                                                                            parsed: 5,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 404,
                                                                            end: 407,
                                                                            as_str(): "(5)",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            then_block: Braces {
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
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 418,
                                                                                            end: 420,
                                                                                            as_str(): "ok",
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
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 408,
                                                                    end: 426,
                                                                    as_str(): "{\n        ok\n    }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 427,
                                                                            end: 431,
                                                                            as_str(): "else",
                                                                        },
                                                                    },
                                                                    Break(
                                                                        Braces {
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
                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                            ),
                                                                                                            start: 442,
                                                                                                            end: 453,
                                                                                                            as_str(): "local_panic",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: Some(
                                                                                                        (
                                                                                                            DoubleColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 453,
                                                                                                                    end: 455,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            GenericArgs {
                                                                                                                parameters: AngleBrackets {
                                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 455,
                                                                                                                            end: 456,
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
                                                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 456,
                                                                                                                                                end: 459,
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
                                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 459,
                                                                                                                            end: 460,
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
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 460,
                                                                                                end: 462,
                                                                                                as_str(): "()",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 432,
                                                                                end: 468,
                                                                                as_str(): "{\n        local_panic::<u64>()\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 468,
                                                            end: 469,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 474,
                                                            end: 477,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 478,
                                                                end: 479,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 480,
                                                            end: 481,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: If(
                                                        IfExpr {
                                                            if_token: IfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 482,
                                                                    end: 484,
                                                                    as_str(): "if",
                                                                },
                                                            },
                                                            condition: Expr(
                                                                Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 485,
                                                                                end: 489,
                                                                                as_str(): "true",
                                                                            },
                                                                            kind: True,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                            then_block: Braces {
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
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 500,
                                                                                                end: 506,
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
                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                    ),
                                                                                                    start: 506,
                                                                                                    end: 508,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                        ),
                                                                                                        start: 508,
                                                                                                        end: 511,
                                                                                                        as_str(): "Err",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: Some(
                                                                                                    (
                                                                                                        DoubleColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                ),
                                                                                                                start: 511,
                                                                                                                end: 513,
                                                                                                                as_str(): "::",
                                                                                                            },
                                                                                                        },
                                                                                                        GenericArgs {
                                                                                                            parameters: AngleBrackets {
                                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 513,
                                                                                                                        end: 514,
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
                                                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 514,
                                                                                                                                                end: 517,
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
                                                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 517,
                                                                                                                                    end: 518,
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
                                                                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 519,
                                                                                                                                            end: 522,
                                                                                                                                            as_str(): "u32",
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
                                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 522,
                                                                                                                        end: 523,
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
                                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                        ),
                                                                                                        start: 524,
                                                                                                        end: 526,
                                                                                                        as_str(): "12",
                                                                                                    },
                                                                                                    parsed: 12,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 523,
                                                                                    end: 527,
                                                                                    as_str(): "(12)",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 490,
                                                                    end: 533,
                                                                    as_str(): "{\n        Result::Err::<u64, u32>(12)\n    }",
                                                                },
                                                            },
                                                            else_opt: Some(
                                                                (
                                                                    ElseToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 534,
                                                                            end: 538,
                                                                            as_str(): "else",
                                                                        },
                                                                    },
                                                                    Break(
                                                                        Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [
                                                                                    Expr {
                                                                                        expr: Return {
                                                                                            return_token: ReturnToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                    ),
                                                                                                    start: 549,
                                                                                                    end: 555,
                                                                                                    as_str(): "return",
                                                                                                },
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                                ),
                                                                                                                start: 556,
                                                                                                                end: 558,
                                                                                                                as_str(): "10",
                                                                                                            },
                                                                                                            parsed: 10,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                    ),
                                                                                                    start: 558,
                                                                                                    end: 559,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ],
                                                                                final_expr_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 539,
                                                                                end: 565,
                                                                                as_str(): "{\n        return 10;\n    }",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 565,
                                                            end: 566,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 571,
                                                            end: 577,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 578,
                                                                        end: 580,
                                                                        as_str(): "42",
                                                                    },
                                                                    parsed: 42,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 580,
                                                            end: 581,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1443204a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                        ),
                                        start: 145,
                                        end: 583,
                                        as_str(): "{\n    // all of these should be okay, since\n    // the branches that would have type errors abort control flow.\n    let x = if true { 42u64 } else { revert(0) };\n    let x: u64 = local_panic::<u64>();\n    let x = if let Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    } else {\n        local_panic::<u64>()\n    };\n    let x = if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    };\n    return 42;\n}",
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
