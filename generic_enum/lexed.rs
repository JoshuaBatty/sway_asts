Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0fc4e9d60,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0fc4e9d60,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
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
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 8,
                                        end: 11,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 16,
                                            as_str(): "core",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
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
                                                src (ptr): 0x00007fe0fc4e9d60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 21,
                                                as_str(): "ops",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fc4e9d60,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                ),
                                                start: 21,
                                                end: 23,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 23,
                                                    end: 26,
                                                    as_str(): "Ord",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 27,
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
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 29,
                                            end: 31,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 32,
                                            end: 36,
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
                                            src (ptr): 0x00007fe0fc4e9d60,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                            ),
                                            start: 36,
                                            end: 38,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 39,
                                                    end: 41,
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
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 42,
                                                                end: 46,
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
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 53,
                                                            end: 56,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 57,
                                                                end: 58,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 60,
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
                                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                            ),
                                                                            start: 61,
                                                                            end: 67,
                                                                            as_str(): "Option",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                                ),
                                                                                start: 67,
                                                                                end: 69,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 69,
                                                                                    end: 73,
                                                                                    as_str(): "Some",
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
                                                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 74,
                                                                                    end: 76,
                                                                                    as_str(): "10",
                                                                                },
                                                                                parsed: 10,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 77,
                                                                as_str(): "(10)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 77,
                                                            end: 78,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 84,
                                                            end: 87,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 88,
                                                                end: 89,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 91,
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
                                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                            ),
                                                                            start: 92,
                                                                            end: 98,
                                                                            as_str(): "Option",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                                ),
                                                                                start: 98,
                                                                                end: 100,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 100,
                                                                                    end: 104,
                                                                                    as_str(): "Some",
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
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 105,
                                                                                    end: 109,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 110,
                                                                as_str(): "(true)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 110,
                                                            end: 111,
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
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 145,
                                                            end: 149,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 47,
                                        end: 151,
                                        as_str(): "{\n    let x = Option::Some(10); \n    let y = Option::Some(true); \n\n //   x == Option::Some(10)\n   true\n}",
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
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 153,
                                        end: 157,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 164,
                                        as_str(): "Option",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 164,
                                                    end: 165,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 165,
                                                            end: 166,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc4e9d60,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                    ),
                                                    start: 166,
                                                    end: 167,
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
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 174,
                                                                end: 178,
                                                                as_str(): "Some",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 178,
                                                                end: 179,
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
                                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                            ),
                                                                            start: 180,
                                                                            end: 181,
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
                                                        src (ptr): 0x00007fe0fc4e9d60,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                        ),
                                                        start: 181,
                                                        end: 182,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: Some(
                                            Annotated {
                                                attribute_list: [],
                                                value: TypeField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 187,
                                                            end: 191,
                                                            as_str(): "None",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc4e9d60,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                            ),
                                                            start: 191,
                                                            end: 192,
                                                            as_str(): ":",
                                                        },
                                                    },
                                                    ty: Tuple(
                                                        Parens {
                                                            inner: Nil,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc4e9d60,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                                                ),
                                                                start: 193,
                                                                end: 195,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc4e9d60,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbC6z6d/generic_enum/src/main.sw",
                                        ),
                                        start: 168,
                                        end: 197,
                                        as_str(): "{\n    Some: T,\n    None: ()\n}",
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
