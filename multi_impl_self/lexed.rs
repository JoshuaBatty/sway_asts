Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe08f6888f0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe08f6888f0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
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
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 24,
                                        as_str(): "MyStruct",
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
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 28,
                                        as_str(): "{\n}",
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
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 30,
                                        end: 34,
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
                                                    src (ptr): 0x00007fe08f6888f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                    ),
                                                    start: 35,
                                                    end: 43,
                                                    as_str(): "MyStruct",
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
                                                    visibility: Some(
                                                        PubToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f6888f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                ),
                                                                start: 50,
                                                                end: 53,
                                                                as_str(): "pub",
                                                            },
                                                        },
                                                    ),
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f6888f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                            ),
                                                            start: 54,
                                                            end: 56,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f6888f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 63,
                                                            as_str(): "my_fun",
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
                                                            src (ptr): 0x00007fe08f6888f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                            ),
                                                            start: 63,
                                                            end: 65,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f6888f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 68,
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
                                                                                src (ptr): 0x00007fe08f6888f0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                                ),
                                                                                start: 69,
                                                                                end: 72,
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
                                                            FuncApp {
                                                                func: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08f6888f0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                                    ),
                                                                                    start: 83,
                                                                                    end: 86,
                                                                                    as_str(): "fun",
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
                                                                        final_value_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6888f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                        ),
                                                                        start: 86,
                                                                        end: 88,
                                                                        as_str(): "()",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f6888f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                        ),
                                                        start: 73,
                                                        end: 94,
                                                        as_str(): "{\n        fun()\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 44,
                                        end: 96,
                                        as_str(): "{\n    pub fn my_fun() -> u64 {\n        fun()\n    }\n}",
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
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 98,
                                        end: 102,
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
                                                    src (ptr): 0x00007fe08f6888f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                    ),
                                                    start: 103,
                                                    end: 111,
                                                    as_str(): "MyStruct",
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
                                    inner: [],
                                    span: Span {
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 112,
                                        end: 115,
                                        as_str(): "{\n}",
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
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 119,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 120,
                                            end: 123,
                                            as_str(): "fun",
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
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 123,
                                            end: 125,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe08f6888f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                    ),
                                                    start: 126,
                                                    end: 128,
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
                                                                src (ptr): 0x00007fe08f6888f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                ),
                                                                start: 129,
                                                                end: 132,
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
                                                            src (ptr): 0x00007fe08f6888f0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                            ),
                                                            start: 139,
                                                            end: 141,
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
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 133,
                                        end: 143,
                                        as_str(): "{\n    42\n}",
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
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 145,
                                            end: 147,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 148,
                                            end: 152,
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
                                            src (ptr): 0x00007fe08f6888f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                            ),
                                            start: 152,
                                            end: 154,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe08f6888f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                    ),
                                                    start: 155,
                                                    end: 157,
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
                                                                src (ptr): 0x00007fe08f6888f0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                ),
                                                                start: 158,
                                                                end: 161,
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
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f6888f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 168,
                                                                    end: 176,
                                                                    as_str(): "MyStruct",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [
                                                            (
                                                                DoubleColonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6888f0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                        ),
                                                                        start: 176,
                                                                        end: 178,
                                                                        as_str(): "::",
                                                                    },
                                                                },
                                                                PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f6888f0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                                            ),
                                                                            start: 178,
                                                                            end: 184,
                                                                            as_str(): "my_fun",
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
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f6888f0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                                        ),
                                                        start: 184,
                                                        end: 186,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe08f6888f0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIony7M/multi_impl_self/src/main.sw",
                                        ),
                                        start: 162,
                                        end: 188,
                                        as_str(): "{\n    MyStruct::my_fun()\n}",
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
