Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe08f5aad80,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe08f5aad80,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe08f5aad80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                            ),
                                            start: 47,
                                            end: 49,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f5aad80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
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
                                            src (ptr): 0x00007fe08f5aad80,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
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
                                                    src (ptr): 0x00007fe08f5aad80,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
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
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 60,
                                                                end: 63,
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
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 70,
                                                            end: 73,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 74,
                                                                end: 89,
                                                                as_str(): "tuple_in_struct",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 91,
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
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 92,
                                                                        end: 105,
                                                                        as_str(): "TupleInStruct",
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
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 116,
                                                                                    end: 128,
                                                                                    as_str(): "nested_tuple",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 128,
                                                                                            end: 129,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Tuple(
                                                                                        Parens {
                                                                                            inner: Cons {
                                                                                                head: Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 131,
                                                                                                                end: 133,
                                                                                                                as_str(): "42",
                                                                                                            },
                                                                                                            parsed: 42,
                                                                                                            ty_opt: Some(
                                                                                                                (
                                                                                                                    U64,
                                                                                                                    Span {
                                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 133,
                                                                                                                        end: 136,
                                                                                                                        as_str(): "u64",
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                                comma_token: CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 136,
                                                                                                        end: 137,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                                tail: Punctuated {
                                                                                                    value_separator_pairs: [],
                                                                                                    final_value_opt: Some(
                                                                                                        Tuple(
                                                                                                            Parens {
                                                                                                                inner: Cons {
                                                                                                                    head: Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 139,
                                                                                                                                    end: 141,
                                                                                                                                    as_str(): "42",
                                                                                                                                },
                                                                                                                                parsed: 42,
                                                                                                                                ty_opt: Some(
                                                                                                                                    (
                                                                                                                                        U32,
                                                                                                                                        Span {
                                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 141,
                                                                                                                                            end: 144,
                                                                                                                                            as_str(): "u32",
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                ),
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    comma_token: CommaToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 144,
                                                                                                                            end: 145,
                                                                                                                            as_str(): ",",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    tail: Punctuated {
                                                                                                                        value_separator_pairs: [],
                                                                                                                        final_value_opt: Some(
                                                                                                                            Tuple(
                                                                                                                                Parens {
                                                                                                                                    inner: Cons {
                                                                                                                                        head: Literal(
                                                                                                                                            Bool(
                                                                                                                                                LitBool {
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 147,
                                                                                                                                                        end: 151,
                                                                                                                                                        as_str(): "true",
                                                                                                                                                    },
                                                                                                                                                    kind: True,
                                                                                                                                                },
                                                                                                                                            ),
                                                                                                                                        ),
                                                                                                                                        comma_token: CommaToken {
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 151,
                                                                                                                                                end: 152,
                                                                                                                                                as_str(): ",",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                        tail: Punctuated {
                                                                                                                                            value_separator_pairs: [],
                                                                                                                                            final_value_opt: Some(
                                                                                                                                                Literal(
                                                                                                                                                    String(
                                                                                                                                                        LitString {
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 153,
                                                                                                                                                                end: 157,
                                                                                                                                                                as_str(): "\"ok\"",
                                                                                                                                                            },
                                                                                                                                                            parsed: "ok",
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                ),
                                                                                                                                            ),
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 146,
                                                                                                                                        end: 158,
                                                                                                                                        as_str(): "(true, \"ok\")",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    },
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 138,
                                                                                                                    end: 160,
                                                                                                                    as_str(): "(42u32, (true, \"ok\") )",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 130,
                                                                                                end: 162,
                                                                                                as_str(): "(42u64, (42u32, (true, \"ok\") ) )",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 162,
                                                                                end: 163,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 106,
                                                                end: 169,
                                                                as_str(): "{\n        nested_tuple: (42u64, (42u32, (true, \"ok\") ) ),\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 170,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 175,
                                                            end: 178,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 179,
                                                                        end: 192,
                                                                        as_str(): "TupleInStruct",
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
                                                                    Field {
                                                                        field_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 195,
                                                                                end: 207,
                                                                                as_str(): "nested_tuple",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        pattern_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 207,
                                                                                        end: 208,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Tuple(
                                                                                    Parens {
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [
                                                                                                (
                                                                                                    Var {
                                                                                                        reference: None,
                                                                                                        mutable: None,
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 210,
                                                                                                                end: 211,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 211,
                                                                                                            end: 212,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: Some(
                                                                                                Tuple(
                                                                                                    Parens {
                                                                                                        inner: Punctuated {
                                                                                                            value_separator_pairs: [
                                                                                                                (
                                                                                                                    Var {
                                                                                                                        reference: None,
                                                                                                                        mutable: None,
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 214,
                                                                                                                                end: 215,
                                                                                                                                as_str(): "b",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    CommaToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 215,
                                                                                                                            end: 216,
                                                                                                                            as_str(): ",",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                            final_value_opt: Some(
                                                                                                                Tuple(
                                                                                                                    Parens {
                                                                                                                        inner: Punctuated {
                                                                                                                            value_separator_pairs: [
                                                                                                                                (
                                                                                                                                    Var {
                                                                                                                                        reference: None,
                                                                                                                                        mutable: None,
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 218,
                                                                                                                                                end: 219,
                                                                                                                                                as_str(): "c",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    CommaToken {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 219,
                                                                                                                                            end: 220,
                                                                                                                                            as_str(): ",",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ],
                                                                                                                            final_value_opt: Some(
                                                                                                                                Var {
                                                                                                                                    reference: None,
                                                                                                                                    mutable: None,
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 221,
                                                                                                                                            end: 222,
                                                                                                                                            as_str(): "d",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 217,
                                                                                                                            end: 223,
                                                                                                                            as_str(): "(c, d)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 213,
                                                                                                            end: 225,
                                                                                                            as_str(): "(b, (c, d) )",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 209,
                                                                                            end: 227,
                                                                                            as_str(): "(a, (b, (c, d) ) )",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 193,
                                                                end: 229,
                                                                as_str(): "{ nested_tuple: (a, (b, (c, d) ) ) }",
                                                            },
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 230,
                                                            end: 231,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 232,
                                                                        end: 247,
                                                                        as_str(): "tuple_in_struct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 247,
                                                            end: 248,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 254,
                                                            end: 257,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 258,
                                                                end: 273,
                                                                as_str(): "struct_in_tuple",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 274,
                                                            end: 275,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Struct {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 277,
                                                                                    end: 282,
                                                                                    as_str(): "Point",
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
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 285,
                                                                                                end: 286,
                                                                                                as_str(): "x",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        expr_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 286,
                                                                                                        end: 287,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 288,
                                                                                                                end: 289,
                                                                                                                as_str(): "2",
                                                                                                            },
                                                                                                            parsed: 2,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 289,
                                                                                            end: 290,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    ExprStructField {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 291,
                                                                                                end: 292,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        expr_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 292,
                                                                                                        end: 293,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 294,
                                                                                                                end: 295,
                                                                                                                as_str(): "4",
                                                                                                            },
                                                                                                            parsed: 4,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 295,
                                                                                            end: 296,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 283,
                                                                            end: 298,
                                                                            as_str(): "{ x: 2, y: 4, }",
                                                                        },
                                                                    },
                                                                },
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 298,
                                                                        end: 299,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Struct {
                                                                            path: PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 300,
                                                                                            end: 305,
                                                                                            as_str(): "Point",
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
                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 308,
                                                                                                        end: 309,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                expr_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 309,
                                                                                                                end: 310,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 311,
                                                                                                                        end: 312,
                                                                                                                        as_str(): "3",
                                                                                                                    },
                                                                                                                    parsed: 3,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                            CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 312,
                                                                                                    end: 313,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    final_value_opt: Some(
                                                                                        ExprStructField {
                                                                                            field_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 314,
                                                                                                    end: 315,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 315,
                                                                                                            end: 316,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 317,
                                                                                                                    end: 318,
                                                                                                                    as_str(): "6",
                                                                                                                },
                                                                                                                parsed: 6,
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
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 306,
                                                                                    end: 320,
                                                                                    as_str(): "{ x: 3, y: 6 }",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 276,
                                                                end: 321,
                                                                as_str(): "(Point { x: 2, y: 4, }, Point { x: 3, y: 6 })",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 321,
                                                            end: 322,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 327,
                                                            end: 330,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Tuple(
                                                        Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        Struct {
                                                                            path: PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 332,
                                                                                            end: 337,
                                                                                            as_str(): "Point",
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
                                                                                            Field {
                                                                                                field_name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 340,
                                                                                                        end: 341,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                pattern_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 341,
                                                                                                                end: 342,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Var {
                                                                                                            reference: None,
                                                                                                            mutable: None,
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 343,
                                                                                                                    end: 345,
                                                                                                                    as_str(): "x0",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                            CommaToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 345,
                                                                                                    end: 346,
                                                                                                    as_str(): ",",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    final_value_opt: Some(
                                                                                        Field {
                                                                                            field_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 347,
                                                                                                    end: 348,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            pattern_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 348,
                                                                                                            end: 349,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Var {
                                                                                                        reference: None,
                                                                                                        mutable: None,
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 350,
                                                                                                                end: 352,
                                                                                                                as_str(): "y0",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 338,
                                                                                    end: 354,
                                                                                    as_str(): "{ x: x0, y: y0 }",
                                                                                },
                                                                            },
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 354,
                                                                                end: 355,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    Struct {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 356,
                                                                                        end: 361,
                                                                                        as_str(): "Point",
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
                                                                                        Field {
                                                                                            field_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 364,
                                                                                                    end: 365,
                                                                                                    as_str(): "x",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            pattern_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 365,
                                                                                                            end: 366,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Var {
                                                                                                        reference: None,
                                                                                                        mutable: None,
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 367,
                                                                                                                end: 369,
                                                                                                                as_str(): "x1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 369,
                                                                                                end: 370,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    Field {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 371,
                                                                                                end: 372,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        pattern_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 372,
                                                                                                        end: 373,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Var {
                                                                                                    reference: None,
                                                                                                    mutable: None,
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 374,
                                                                                                            end: 376,
                                                                                                            as_str(): "y1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 362,
                                                                                end: 378,
                                                                                as_str(): "{ x: x1, y: y1 }",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 331,
                                                                end: 379,
                                                                as_str(): "(Point { x: x0, y: y0 }, Point { x: x1, y: y1 })",
                                                            },
                                                        },
                                                    ),
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 380,
                                                            end: 381,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 382,
                                                                        end: 397,
                                                                        as_str(): "struct_in_tuple",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 397,
                                                            end: 398,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 404,
                                                            end: 407,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 408,
                                                                end: 414,
                                                                as_str(): "point1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 415,
                                                            end: 416,
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
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 417,
                                                                        end: 422,
                                                                        as_str(): "Point",
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
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 425,
                                                                                    end: 426,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 426,
                                                                                            end: 427,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 428,
                                                                                                    end: 429,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 429,
                                                                                end: 430,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    ExprStructField {
                                                                        field_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 431,
                                                                                end: 432,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 432,
                                                                                        end: 433,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 434,
                                                                                                end: 435,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                            parsed: 0,
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
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 423,
                                                                end: 437,
                                                                as_str(): "{ x: 0, y: 0 }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 437,
                                                            end: 438,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 443,
                                                            end: 446,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 447,
                                                                end: 453,
                                                                as_str(): "point2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 454,
                                                            end: 455,
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
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 456,
                                                                        end: 461,
                                                                        as_str(): "Point",
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
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 464,
                                                                                    end: 465,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 465,
                                                                                            end: 466,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 467,
                                                                                                    end: 468,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 468,
                                                                                end: 469,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    ExprStructField {
                                                                        field_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 470,
                                                                                end: 471,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 471,
                                                                                        end: 472,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 473,
                                                                                                end: 474,
                                                                                                as_str(): "1",
                                                                                            },
                                                                                            parsed: 1,
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
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 462,
                                                                end: 476,
                                                                as_str(): "{ x: 1, y: 1 }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 476,
                                                            end: 477,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 482,
                                                            end: 485,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 486,
                                                                end: 490,
                                                                as_str(): "line",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 491,
                                                            end: 492,
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
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 493,
                                                                        end: 497,
                                                                        as_str(): "Line",
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
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 500,
                                                                                    end: 502,
                                                                                    as_str(): "p1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 502,
                                                                                            end: 503,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 504,
                                                                                                        end: 510,
                                                                                                        as_str(): "point1",
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
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 510,
                                                                                end: 511,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    ExprStructField {
                                                                        field_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 512,
                                                                                end: 514,
                                                                                as_str(): "p2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 514,
                                                                                        end: 515,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 516,
                                                                                                    end: 522,
                                                                                                    as_str(): "point2",
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
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 498,
                                                                end: 524,
                                                                as_str(): "{ p1: point1, p2: point2 }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 524,
                                                            end: 525,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 530,
                                                            end: 533,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 534,
                                                                        end: 538,
                                                                        as_str(): "Line",
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
                                                                        Field {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                    ),
                                                                                    start: 541,
                                                                                    end: 543,
                                                                                    as_str(): "p1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            pattern_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 543,
                                                                                            end: 544,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Struct {
                                                                                        path: PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                        ),
                                                                                                        start: 545,
                                                                                                        end: 550,
                                                                                                        as_str(): "Point",
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
                                                                                                        Field {
                                                                                                            field_name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 553,
                                                                                                                    end: 554,
                                                                                                                    as_str(): "x",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            pattern_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 554,
                                                                                                                            end: 555,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Var {
                                                                                                                        reference: None,
                                                                                                                        mutable: None,
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 556,
                                                                                                                                end: 558,
                                                                                                                                as_str(): "x2",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 558,
                                                                                                                end: 559,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: Some(
                                                                                                    Field {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 560,
                                                                                                                end: 561,
                                                                                                                as_str(): "y",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        pattern_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 561,
                                                                                                                        end: 562,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Var {
                                                                                                                    reference: None,
                                                                                                                    mutable: None,
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 563,
                                                                                                                            end: 565,
                                                                                                                            as_str(): "y2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 551,
                                                                                                end: 567,
                                                                                                as_str(): "{ x: x2, y: y2 }",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 567,
                                                                                end: 568,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: Some(
                                                                    Field {
                                                                        field_name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                ),
                                                                                start: 569,
                                                                                end: 571,
                                                                                as_str(): "p2",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        pattern_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 571,
                                                                                        end: 572,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Struct {
                                                                                    path: PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                    ),
                                                                                                    start: 573,
                                                                                                    end: 578,
                                                                                                    as_str(): "Point",
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
                                                                                                    Field {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                ),
                                                                                                                start: 581,
                                                                                                                end: 582,
                                                                                                                as_str(): "x",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        pattern_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 582,
                                                                                                                        end: 583,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Var {
                                                                                                                    reference: None,
                                                                                                                    mutable: None,
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 584,
                                                                                                                            end: 586,
                                                                                                                            as_str(): "x3",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 586,
                                                                                                            end: 587,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: Some(
                                                                                                Field {
                                                                                                    field_name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 588,
                                                                                                            end: 589,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    pattern_opt: Some(
                                                                                                        (
                                                                                                            ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 589,
                                                                                                                    end: 590,
                                                                                                                    as_str(): ":",
                                                                                                                },
                                                                                                            },
                                                                                                            Var {
                                                                                                                reference: None,
                                                                                                                mutable: None,
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 591,
                                                                                                                        end: 593,
                                                                                                                        as_str(): "y3",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                            ),
                                                                                            start: 579,
                                                                                            end: 594,
                                                                                            as_str(): "{ x: x3, y: y3}",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 539,
                                                                end: 596,
                                                                as_str(): "{ p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} }",
                                                            },
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 597,
                                                            end: 598,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                        ),
                                                                        start: 599,
                                                                        end: 603,
                                                                        as_str(): "line",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f5aad80,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                            ),
                                                            start: 603,
                                                            end: 604,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 609,
                                                                end: 611,
                                                                as_str(): "x2",
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
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 64,
                                        end: 613,
                                        as_str(): "{\n    let tuple_in_struct = TupleInStruct {\n        nested_tuple: (42u64, (42u32, (true, \"ok\") ) ),\n    };\n    let TupleInStruct { nested_tuple: (a, (b, (c, d) ) ) } = tuple_in_struct;\n\n    let struct_in_tuple = (Point { x: 2, y: 4, }, Point { x: 3, y: 6 });\n    let (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = struct_in_tuple;\n\n    let point1 = Point { x: 0, y: 0 };\n    let point2 = Point { x: 1, y: 1 };\n    let line = Line { p1: point1, p2: point2 };\n    let Line { p1: Point { x: x2, y: y2 }, p2: Point { x: x3, y: y3} } = line;\n    x2\n}",
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
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 615,
                                        end: 621,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 622,
                                        end: 627,
                                        as_str(): "Point",
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
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 634,
                                                                end: 635,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 635,
                                                                end: 636,
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
                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 637,
                                                                            end: 640,
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
                                                        src (ptr): 0x00007fe08f5aad80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 640,
                                                        end: 641,
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
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 646,
                                                                end: 647,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 647,
                                                                end: 648,
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
                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 649,
                                                                            end: 652,
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
                                                        src (ptr): 0x00007fe08f5aad80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 652,
                                                        end: 653,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 628,
                                        end: 655,
                                        as_str(): "{\n    x: u64,\n    y: u64,\n}",
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
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 657,
                                        end: 663,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 664,
                                        end: 668,
                                        as_str(): "Line",
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
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 675,
                                                                end: 677,
                                                                as_str(): "p1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 677,
                                                                end: 678,
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
                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 679,
                                                                            end: 684,
                                                                            as_str(): "Point",
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
                                                        src (ptr): 0x00007fe08f5aad80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 684,
                                                        end: 685,
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
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 690,
                                                                end: 692,
                                                                as_str(): "p2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 692,
                                                                end: 693,
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
                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 694,
                                                                            end: 699,
                                                                            as_str(): "Point",
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
                                                        src (ptr): 0x00007fe08f5aad80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 699,
                                                        end: 700,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 669,
                                        end: 702,
                                        as_str(): "{\n    p1: Point,\n    p2: Point,\n}",
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
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 704,
                                        end: 710,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 711,
                                        end: 724,
                                        as_str(): "TupleInStruct",
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
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 731,
                                                                end: 743,
                                                                as_str(): "nested_tuple",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f5aad80,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                ),
                                                                start: 743,
                                                                end: 744,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Cons {
                                                                    head: Path(
                                                                        PathType {
                                                                            root_opt: None,
                                                                            prefix: PathTypeSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 746,
                                                                                        end: 749,
                                                                                        as_str(): "u64",
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
                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                            ),
                                                                            start: 749,
                                                                            end: 750,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                    tail: Punctuated {
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
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 752,
                                                                                                            end: 755,
                                                                                                            as_str(): "u32",
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
                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                ),
                                                                                                start: 755,
                                                                                                end: 756,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                        tail: Punctuated {
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
                                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 758,
                                                                                                                                end: 762,
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
                                                                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 762,
                                                                                                                    end: 763,
                                                                                                                    as_str(): ",",
                                                                                                                },
                                                                                                            },
                                                                                                            tail: Punctuated {
                                                                                                                value_separator_pairs: [],
                                                                                                                final_value_opt: Some(
                                                                                                                    Str {
                                                                                                                        str_token: StrToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 764,
                                                                                                                                end: 767,
                                                                                                                                as_str(): "str",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        length: SquareBrackets {
                                                                                                                            inner: Literal(
                                                                                                                                Int(
                                                                                                                                    LitInt {
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 768,
                                                                                                                                            end: 769,
                                                                                                                                            as_str(): "2",
                                                                                                                                        },
                                                                                                                                        parsed: 2,
                                                                                                                                        ty_opt: None,
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe08f5aad80,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 767,
                                                                                                                                end: 770,
                                                                                                                                as_str(): "[2]",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            },
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08f5aad80,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                                            ),
                                                                                                            start: 757,
                                                                                                            end: 771,
                                                                                                            as_str(): "(bool, str[2])",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f5aad80,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                                        ),
                                                                                        start: 751,
                                                                                        end: 773,
                                                                                        as_str(): "(u32, (bool, str[2]) )",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f5aad80,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                                    ),
                                                                    start: 745,
                                                                    end: 775,
                                                                    as_str(): "(u64, (u32, (bool, str[2]) ) )",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f5aad80,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                                        ),
                                                        start: 775,
                                                        end: 776,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe08f5aad80,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRkQ5Ov8/nested_struct_destructuring/src/main.sw",
                                        ),
                                        start: 725,
                                        end: 778,
                                        as_str(): "{\n    nested_tuple: (u64, (u32, (bool, str[2]) ) ),\n}",
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
