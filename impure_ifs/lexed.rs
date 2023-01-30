Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0d2fa5ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0d2fa5ff0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
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
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 19,
                                                                    end: 25,
                                                                    as_str(): "assert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 25,
                                                                    end: 27,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 27,
                                                                        end: 33,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 33,
                                                                end: 34,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 42,
                                                                as_str(): "logging",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 42,
                                                                end: 44,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 44,
                                                                    end: 47,
                                                                    as_str(): "log",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                ),
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 48,
                                                as_str(): "{assert::assert, logging::log}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 49,
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
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 51,
                                        end: 55,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 56,
                                        end: 60,
                                        as_str(): "Bool",
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 67,
                                                                end: 71,
                                                                as_str(): "True",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 71,
                                                                end: 72,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 73,
                                                                    end: 75,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 75,
                                                        end: 76,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 81,
                                                                end: 86,
                                                                as_str(): "False",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 86,
                                                                end: 87,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 88,
                                                                    end: 90,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                        ),
                                                        start: 90,
                                                        end: 91,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 61,
                                        end: 93,
                                        as_str(): "{\n    True: (),\n    False: (),\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 95,
                                            end: 97,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 101,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 102,
                                                                    end: 103,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 103,
                                                                end: 104,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 105,
                                                                            end: 109,
                                                                            as_str(): "bool",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 101,
                                            end: 110,
                                            as_str(): "(b: bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 111,
                                                    end: 113,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 114,
                                                                end: 117,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 124,
                                                            end: 126,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 127,
                                                                            end: 128,
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
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 139,
                                                                                end: 142,
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
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 129,
                                                            end: 148,
                                                            as_str(): "{\n        101\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 149,
                                                                    end: 153,
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 164,
                                                                                            end: 167,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 154,
                                                                        end: 173,
                                                                        as_str(): "{\n        102\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 118,
                                        end: 175,
                                        as_str(): "{\n    if b {\n        101\n    } else {\n        102\n    }\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 177,
                                            end: 179,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 180,
                                            end: 183,
                                            as_str(): "bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 184,
                                                                    end: 185,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 185,
                                                                end: 186,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 187,
                                                                            end: 191,
                                                                            as_str(): "bool",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 183,
                                            end: 192,
                                            as_str(): "(b: bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 193,
                                                    end: 195,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 199,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 206,
                                                            end: 208,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 209,
                                                                            end: 210,
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
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: Return {
                                                                        return_token: ReturnToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 221,
                                                                                end: 227,
                                                                                as_str(): "return",
                                                                            },
                                                                        },
                                                                        expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 228,
                                                                                            end: 231,
                                                                                            as_str(): "101",
                                                                                        },
                                                                                        parsed: 101,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 231,
                                                                                end: 232,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 211,
                                                            end: 238,
                                                            as_str(): "{\n        return 101;\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 239,
                                                                    end: 243,
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 254,
                                                                                            end: 260,
                                                                                            as_str(): "return",
                                                                                        },
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 261,
                                                                                                        end: 264,
                                                                                                        as_str(): "102",
                                                                                                    },
                                                                                                    parsed: 102,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                semicolon_token_opt: Some(
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 264,
                                                                                            end: 265,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ],
                                                                        final_expr_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 244,
                                                                        end: 271,
                                                                        as_str(): "{\n        return 102;\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 200,
                                        end: 273,
                                        as_str(): "{\n    if b {\n        return 101;\n    } else {\n        return 102;\n    }\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 275,
                                            end: 277,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 278,
                                            end: 282,
                                            as_str(): "bell",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 283,
                                                                    end: 284,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 284,
                                                                end: 285,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 286,
                                                                            end: 290,
                                                                            as_str(): "bool",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 282,
                                            end: 291,
                                            as_str(): "(b: bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 292,
                                                    end: 294,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 295,
                                                                end: 298,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 305,
                                                            end: 307,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 308,
                                                                            end: 309,
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
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: Return {
                                                                        return_token: ReturnToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 320,
                                                                                end: 326,
                                                                                as_str(): "return",
                                                                            },
                                                                        },
                                                                        expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 327,
                                                                                            end: 330,
                                                                                            as_str(): "101",
                                                                                        },
                                                                                        parsed: 101,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 330,
                                                                                end: 331,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 310,
                                                            end: 337,
                                                            as_str(): "{\n        return 101;\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 338,
                                                                    end: 342,
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 353,
                                                                                            end: 356,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 343,
                                                                        end: 362,
                                                                        as_str(): "{\n        102\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 299,
                                        end: 364,
                                        as_str(): "{\n    if b {\n        return 101;\n    } else {\n        102\n    }\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 366,
                                            end: 368,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 369,
                                            end: 372,
                                            as_str(): "moo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 373,
                                                                    end: 374,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 374,
                                                                end: 375,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 376,
                                                                            end: 380,
                                                                            as_str(): "bool",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 372,
                                            end: 381,
                                            as_str(): "(b: bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 382,
                                                    end: 384,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 385,
                                                                end: 388,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 395,
                                                            end: 397,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 398,
                                                                            end: 399,
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
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 410,
                                                                                end: 413,
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
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 400,
                                                            end: 419,
                                                            as_str(): "{\n        101\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 420,
                                                                    end: 424,
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 435,
                                                                                            end: 441,
                                                                                            as_str(): "return",
                                                                                        },
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 442,
                                                                                                        end: 445,
                                                                                                        as_str(): "102",
                                                                                                    },
                                                                                                    parsed: 102,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                semicolon_token_opt: Some(
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 445,
                                                                                            end: 446,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ],
                                                                        final_expr_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 425,
                                                                        end: 452,
                                                                        as_str(): "{\n        return 102;\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 389,
                                        end: 454,
                                        as_str(): "{\n    if b {\n        101\n    } else {\n        return 102;\n    }\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 456,
                                            end: 458,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 459,
                                            end: 462,
                                            as_str(): "poo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 463,
                                                                    end: 464,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 464,
                                                                end: 465,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 466,
                                                                            end: 470,
                                                                            as_str(): "Bool",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 462,
                                            end: 471,
                                            as_str(): "(b: Bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 472,
                                                    end: 474,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 475,
                                                                end: 478,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 485,
                                                            end: 487,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Let {
                                                        let_token: LetToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 488,
                                                                end: 491,
                                                                as_str(): "let",
                                                            },
                                                        },
                                                        lhs: Constant(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 492,
                                                                            end: 496,
                                                                            as_str(): "Bool",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 496,
                                                                                end: 498,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 498,
                                                                                    end: 502,
                                                                                    as_str(): "True",
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
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 503,
                                                                end: 504,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 505,
                                                                            end: 506,
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
                                                    },
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 517,
                                                                                end: 520,
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
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 507,
                                                            end: 526,
                                                            as_str(): "{\n        101\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 527,
                                                                    end: 531,
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 542,
                                                                                            end: 548,
                                                                                            as_str(): "return",
                                                                                        },
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 549,
                                                                                                        end: 552,
                                                                                                        as_str(): "102",
                                                                                                    },
                                                                                                    parsed: 102,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                semicolon_token_opt: Some(
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 552,
                                                                                            end: 553,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ],
                                                                        final_expr_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 532,
                                                                        end: 559,
                                                                        as_str(): "{\n        return 102;\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 479,
                                        end: 561,
                                        as_str(): "{\n    if let Bool::True = b {\n        101\n    } else {\n        return 102;\n    }\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 563,
                                            end: 565,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 566,
                                            end: 582,
                                            as_str(): "ran_out_of_names",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 583,
                                                                    end: 584,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 584,
                                                                end: 585,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 586,
                                                                            end: 590,
                                                                            as_str(): "Bool",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 582,
                                            end: 591,
                                            as_str(): "(b: Bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 592,
                                                    end: 594,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 595,
                                                                end: 598,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 605,
                                                            end: 607,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Let {
                                                        let_token: LetToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 608,
                                                                end: 611,
                                                                as_str(): "let",
                                                            },
                                                        },
                                                        lhs: Constant(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 612,
                                                                            end: 616,
                                                                            as_str(): "Bool",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 616,
                                                                                end: 618,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 618,
                                                                                    end: 622,
                                                                                    as_str(): "True",
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
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 623,
                                                                end: 624,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 625,
                                                                            end: 626,
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
                                                    },
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: Return {
                                                                        return_token: ReturnToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 637,
                                                                                end: 643,
                                                                                as_str(): "return",
                                                                            },
                                                                        },
                                                                        expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 644,
                                                                                            end: 647,
                                                                                            as_str(): "101",
                                                                                        },
                                                                                        parsed: 101,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 647,
                                                                                end: 648,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 627,
                                                            end: 654,
                                                            as_str(): "{\n        return 101;\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 655,
                                                                    end: 659,
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 670,
                                                                                            end: 676,
                                                                                            as_str(): "return",
                                                                                        },
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 677,
                                                                                                        end: 680,
                                                                                                        as_str(): "102",
                                                                                                    },
                                                                                                    parsed: 102,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                semicolon_token_opt: Some(
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 680,
                                                                                            end: 681,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ],
                                                                        final_expr_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 660,
                                                                        end: 687,
                                                                        as_str(): "{\n        return 102;\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 599,
                                        end: 689,
                                        as_str(): "{\n    if let Bool::True = b {\n        return 101;\n    } else {\n        return 102;\n    }\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 691,
                                            end: 693,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 694,
                                            end: 704,
                                            as_str(): "another_fn",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 705,
                                                                    end: 706,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 706,
                                                                end: 707,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 708,
                                                                            end: 712,
                                                                            as_str(): "Bool",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 704,
                                            end: 713,
                                            as_str(): "(b: Bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 714,
                                                    end: 716,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 717,
                                                                end: 720,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 727,
                                                            end: 729,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Let {
                                                        let_token: LetToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 730,
                                                                end: 733,
                                                                as_str(): "let",
                                                            },
                                                        },
                                                        lhs: Constant(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 734,
                                                                            end: 738,
                                                                            as_str(): "Bool",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 738,
                                                                                end: 740,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 740,
                                                                                    end: 744,
                                                                                    as_str(): "True",
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
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 745,
                                                                end: 746,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 747,
                                                                            end: 748,
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
                                                    },
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [
                                                                Expr {
                                                                    expr: Return {
                                                                        return_token: ReturnToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 759,
                                                                                end: 765,
                                                                                as_str(): "return",
                                                                            },
                                                                        },
                                                                        expr_opt: Some(
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 766,
                                                                                            end: 769,
                                                                                            as_str(): "101",
                                                                                        },
                                                                                        parsed: 101,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    semicolon_token_opt: Some(
                                                                        SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 769,
                                                                                end: 770,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                            final_expr_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 749,
                                                            end: 776,
                                                            as_str(): "{\n        return 101;\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 777,
                                                                    end: 781,
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 792,
                                                                                            end: 795,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 782,
                                                                        end: 801,
                                                                        as_str(): "{\n        102\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 721,
                                        end: 803,
                                        as_str(): "{\n    if let Bool::True = b {\n        return 101;\n    } else {\n        102\n    }\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 805,
                                            end: 807,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 808,
                                            end: 817,
                                            as_str(): "thats_all",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
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
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 818,
                                                                    end: 819,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 819,
                                                                end: 820,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 821,
                                                                            end: 825,
                                                                            as_str(): "Bool",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 817,
                                            end: 826,
                                            as_str(): "(b: Bool)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 827,
                                                    end: 829,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 830,
                                                                end: 833,
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
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 840,
                                                            end: 842,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Let {
                                                        let_token: LetToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 843,
                                                                end: 846,
                                                                as_str(): "let",
                                                            },
                                                        },
                                                        lhs: Constant(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 847,
                                                                            end: 851,
                                                                            as_str(): "Bool",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 851,
                                                                                end: 853,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                    ),
                                                                                    start: 853,
                                                                                    end: 857,
                                                                                    as_str(): "True",
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
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 858,
                                                                end: 859,
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
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 860,
                                                                            end: 861,
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
                                                    },
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 872,
                                                                                end: 875,
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
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 862,
                                                            end: 881,
                                                            as_str(): "{\n        101\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                    ),
                                                                    start: 882,
                                                                    end: 886,
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 897,
                                                                                            end: 900,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 887,
                                                                        end: 906,
                                                                        as_str(): "{\n        102\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 834,
                                        end: 908,
                                        as_str(): "{\n    if let Bool::True = b {\n        101\n    } else {\n        102\n    }\n}",
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 910,
                                            end: 912,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 913,
                                            end: 917,
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
                                            src (ptr): 0x00007fe0d2fa5ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                            ),
                                            start: 917,
                                            end: 919,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                    ),
                                                    start: 920,
                                                    end: 922,
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
                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                ),
                                                                start: 923,
                                                                end: 926,
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
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 933,
                                                                        end: 939,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 940,
                                                                                            end: 943,
                                                                                            as_str(): "foo",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 944,
                                                                                                    end: 948,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 943,
                                                                                end: 949,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 950,
                                                                            end: 952,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 953,
                                                                                            end: 956,
                                                                                            as_str(): "bar",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 957,
                                                                                                    end: 961,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 956,
                                                                                end: 962,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 939,
                                                            end: 963,
                                                            as_str(): "(foo(true) == bar(true))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 963,
                                                            end: 964,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 969,
                                                                        end: 975,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 976,
                                                                                            end: 979,
                                                                                            as_str(): "foo",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 980,
                                                                                                    end: 985,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 979,
                                                                                end: 986,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 987,
                                                                            end: 989,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 990,
                                                                                            end: 993,
                                                                                            as_str(): "bar",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 994,
                                                                                                    end: 999,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 993,
                                                                                end: 1000,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 975,
                                                            end: 1001,
                                                            as_str(): "(foo(false) == bar(false))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1001,
                                                            end: 1002,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1007,
                                                                        end: 1013,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1014,
                                                                                            end: 1017,
                                                                                            as_str(): "foo",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1018,
                                                                                                    end: 1022,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1017,
                                                                                end: 1023,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1024,
                                                                            end: 1026,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1027,
                                                                                            end: 1031,
                                                                                            as_str(): "bell",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1032,
                                                                                                    end: 1036,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1031,
                                                                                end: 1037,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1013,
                                                            end: 1038,
                                                            as_str(): "(foo(true) == bell(true))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1038,
                                                            end: 1039,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1044,
                                                                        end: 1050,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1051,
                                                                                            end: 1054,
                                                                                            as_str(): "foo",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1055,
                                                                                                    end: 1060,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1054,
                                                                                end: 1061,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1062,
                                                                            end: 1064,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1065,
                                                                                            end: 1069,
                                                                                            as_str(): "bell",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1070,
                                                                                                    end: 1075,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1069,
                                                                                end: 1076,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1050,
                                                            end: 1077,
                                                            as_str(): "(foo(false) == bell(false))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1077,
                                                            end: 1078,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1083,
                                                                        end: 1089,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1090,
                                                                                            end: 1093,
                                                                                            as_str(): "foo",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1094,
                                                                                                    end: 1098,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1093,
                                                                                end: 1099,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1100,
                                                                            end: 1102,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1103,
                                                                                            end: 1106,
                                                                                            as_str(): "moo",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1107,
                                                                                                    end: 1111,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1106,
                                                                                end: 1112,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1089,
                                                            end: 1113,
                                                            as_str(): "(foo(true) == moo(true))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1113,
                                                            end: 1114,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1119,
                                                                        end: 1125,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1126,
                                                                                            end: 1129,
                                                                                            as_str(): "foo",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1130,
                                                                                                    end: 1135,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1129,
                                                                                end: 1136,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1137,
                                                                            end: 1139,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1140,
                                                                                            end: 1143,
                                                                                            as_str(): "moo",
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
                                                                                                    src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1144,
                                                                                                    end: 1149,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1143,
                                                                                end: 1150,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1125,
                                                            end: 1151,
                                                            as_str(): "(foo(false) == moo(false))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1151,
                                                            end: 1152,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1158,
                                                                        end: 1164,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1165,
                                                                                            end: 1174,
                                                                                            as_str(): "thats_all",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1175,
                                                                                                        end: 1179,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1179,
                                                                                                            end: 1181,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1181,
                                                                                                                end: 1185,
                                                                                                                as_str(): "True",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1174,
                                                                                end: 1186,
                                                                                as_str(): "(Bool::True)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1187,
                                                                            end: 1189,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1190,
                                                                                            end: 1193,
                                                                                            as_str(): "poo",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1194,
                                                                                                        end: 1198,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1198,
                                                                                                            end: 1200,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1200,
                                                                                                                end: 1204,
                                                                                                                as_str(): "True",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1193,
                                                                                end: 1205,
                                                                                as_str(): "(Bool::True)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1164,
                                                            end: 1206,
                                                            as_str(): "(thats_all(Bool::True) == poo(Bool::True))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1206,
                                                            end: 1207,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1212,
                                                                        end: 1218,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1219,
                                                                                            end: 1228,
                                                                                            as_str(): "thats_all",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1229,
                                                                                                        end: 1233,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1233,
                                                                                                            end: 1235,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1235,
                                                                                                                end: 1240,
                                                                                                                as_str(): "False",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1228,
                                                                                end: 1241,
                                                                                as_str(): "(Bool::False)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1242,
                                                                            end: 1244,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1245,
                                                                                            end: 1248,
                                                                                            as_str(): "poo",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1249,
                                                                                                        end: 1253,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1253,
                                                                                                            end: 1255,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1255,
                                                                                                                end: 1260,
                                                                                                                as_str(): "False",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1248,
                                                                                end: 1261,
                                                                                as_str(): "(Bool::False)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1218,
                                                            end: 1262,
                                                            as_str(): "(thats_all(Bool::False) == poo(Bool::False))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1262,
                                                            end: 1263,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1268,
                                                                        end: 1274,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1275,
                                                                                            end: 1284,
                                                                                            as_str(): "thats_all",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1285,
                                                                                                        end: 1289,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1289,
                                                                                                            end: 1291,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1291,
                                                                                                                end: 1295,
                                                                                                                as_str(): "True",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1284,
                                                                                end: 1296,
                                                                                as_str(): "(Bool::True)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1297,
                                                                            end: 1299,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1300,
                                                                                            end: 1316,
                                                                                            as_str(): "ran_out_of_names",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1317,
                                                                                                        end: 1321,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1321,
                                                                                                            end: 1323,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1323,
                                                                                                                end: 1327,
                                                                                                                as_str(): "True",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1316,
                                                                                end: 1328,
                                                                                as_str(): "(Bool::True)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1274,
                                                            end: 1329,
                                                            as_str(): "(thats_all(Bool::True) == ran_out_of_names(Bool::True))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1329,
                                                            end: 1330,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1335,
                                                                        end: 1341,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1342,
                                                                                            end: 1351,
                                                                                            as_str(): "thats_all",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1352,
                                                                                                        end: 1356,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1356,
                                                                                                            end: 1358,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1358,
                                                                                                                end: 1363,
                                                                                                                as_str(): "False",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1351,
                                                                                end: 1364,
                                                                                as_str(): "(Bool::False)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1365,
                                                                            end: 1367,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1368,
                                                                                            end: 1384,
                                                                                            as_str(): "ran_out_of_names",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1385,
                                                                                                        end: 1389,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1389,
                                                                                                            end: 1391,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1391,
                                                                                                                end: 1396,
                                                                                                                as_str(): "False",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1384,
                                                                                end: 1397,
                                                                                as_str(): "(Bool::False)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1341,
                                                            end: 1398,
                                                            as_str(): "(thats_all(Bool::False) == ran_out_of_names(Bool::False))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1398,
                                                            end: 1399,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1404,
                                                                        end: 1410,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1411,
                                                                                            end: 1420,
                                                                                            as_str(): "thats_all",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1421,
                                                                                                        end: 1425,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1425,
                                                                                                            end: 1427,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1427,
                                                                                                                end: 1431,
                                                                                                                as_str(): "True",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1420,
                                                                                end: 1432,
                                                                                as_str(): "(Bool::True)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1433,
                                                                            end: 1435,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1436,
                                                                                            end: 1446,
                                                                                            as_str(): "another_fn",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1447,
                                                                                                        end: 1451,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1451,
                                                                                                            end: 1453,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1453,
                                                                                                                end: 1457,
                                                                                                                as_str(): "True",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1446,
                                                                                end: 1458,
                                                                                as_str(): "(Bool::True)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1410,
                                                            end: 1459,
                                                            as_str(): "(thats_all(Bool::True) == another_fn(Bool::True))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1459,
                                                            end: 1460,
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
                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                        ),
                                                                        start: 1465,
                                                                        end: 1471,
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
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1472,
                                                                                            end: 1481,
                                                                                            as_str(): "thats_all",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1482,
                                                                                                        end: 1486,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1486,
                                                                                                            end: 1488,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1488,
                                                                                                                end: 1493,
                                                                                                                as_str(): "False",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1481,
                                                                                end: 1494,
                                                                                as_str(): "(Bool::False)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                            ),
                                                                            start: 1495,
                                                                            end: 1497,
                                                                            as_str(): "==",
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
                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                            ),
                                                                                            start: 1498,
                                                                                            end: 1508,
                                                                                            as_str(): "another_fn",
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
                                                                                                        src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1509,
                                                                                                        end: 1513,
                                                                                                        as_str(): "Bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1513,
                                                                                                            end: 1515,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1515,
                                                                                                                end: 1520,
                                                                                                                as_str(): "False",
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
                                                                                src (ptr): 0x00007fe0d2fa5ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                                                ),
                                                                                start: 1508,
                                                                                end: 1521,
                                                                                as_str(): "(Bool::False)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1471,
                                                            end: 1522,
                                                            as_str(): "(thats_all(Bool::False) == another_fn(Bool::False))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1522,
                                                            end: 1523,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d2fa5ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                                            ),
                                                            start: 1529,
                                                            end: 1530,
                                                            as_str(): "2",
                                                        },
                                                        parsed: 2,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0d2fa5ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6Jhagq/impure_ifs/src/main.sw",
                                        ),
                                        start: 927,
                                        end: 1532,
                                        as_str(): "{\n    assert(foo(true) == bar(true));\n    assert(foo(false) == bar(false));\n    assert(foo(true) == bell(true));\n    assert(foo(false) == bell(false));\n    assert(foo(true) == moo(true));\n    assert(foo(false) == moo(false));\n\n    assert(thats_all(Bool::True) == poo(Bool::True));\n    assert(thats_all(Bool::False) == poo(Bool::False));\n    assert(thats_all(Bool::True) == ran_out_of_names(Bool::True));\n    assert(thats_all(Bool::False) == ran_out_of_names(Bool::False));\n    assert(thats_all(Bool::True) == another_fn(Bool::True));\n    assert(thats_all(Bool::False) == another_fn(Bool::False));\n\n    2\n}",
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
