Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0934e5bc0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0934e5bc0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
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
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
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
                                        value_separator_pairs: [],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 21,
                                        as_str(): "{ }",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 23,
                                        end: 27,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: None,
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0934e5bc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                    ),
                                                    start: 28,
                                                    end: 29,
                                                    as_str(): "A",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                    },
                                ),
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0934e5bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                            ),
                                                            start: 36,
                                                            end: 38,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0934e5bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                            ),
                                                            start: 39,
                                                            end: 40,
                                                            as_str(): "f",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0934e5bc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                    ),
                                                                    start: 41,
                                                                    end: 45,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0934e5bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                            ),
                                                            start: 40,
                                                            end: 46,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0934e5bc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                    ),
                                                                    start: 47,
                                                                    end: 49,
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
                                                                                src (ptr): 0x00007fe0934e5bc0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                                ),
                                                                                start: 50,
                                                                                end: 53,
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
                                                            Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0934e5bc0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                            ),
                                                                            start: 64,
                                                                            end: 65,
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
                                                        src (ptr): 0x00007fe0934e5bc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                        ),
                                                        start: 54,
                                                        end: 71,
                                                        as_str(): "{\n        1\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 30,
                                        end: 73,
                                        as_str(): "{\n    fn f(self) -> u64 {\n        1\n    }\n}",
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
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 75,
                                            end: 77,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 82,
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
                                            src (ptr): 0x00007fe0934e5bc0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                            ),
                                            start: 82,
                                            end: 84,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0934e5bc0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                    ),
                                                    start: 85,
                                                    end: 87,
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
                                                                src (ptr): 0x00007fe0934e5bc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                ),
                                                                start: 88,
                                                                end: 91,
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
                                                            src (ptr): 0x00007fe0934e5bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 101,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0934e5bc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 103,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0934e5bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 105,
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
                                                                        src (ptr): 0x00007fe0934e5bc0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                        ),
                                                                        start: 106,
                                                                        end: 107,
                                                                        as_str(): "A",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0934e5bc0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                ),
                                                                start: 108,
                                                                end: 111,
                                                                as_str(): "{ }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0934e5bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 112,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            MethodCall {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0934e5bc0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 118,
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
                                                dot_token: DotToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0934e5bc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                        ),
                                                        start: 118,
                                                        end: 119,
                                                        as_str(): ".",
                                                    },
                                                },
                                                path_seg: PathExprSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0934e5bc0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 120,
                                                            as_str(): "f",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                contract_args_opt: None,
                                                args: Parens {
                                                    inner: Punctuated {
                                                        value_separator_pairs: [],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0934e5bc0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                                        ),
                                                        start: 120,
                                                        end: 122,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0934e5bc0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIROKgJW4/method_on_empty_struct/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 124,
                                        as_str(): "{\n    let a = A { };\n    a.f()\n}",
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
