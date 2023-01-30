Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb13db7f560,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb13db7f560,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Dependency(
                            Dependency {
                                dep_token: DepToken {
                                    span: Span {
                                        src (ptr): 0x00007fb13db7f560,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                        ),
                                        start: 46,
                                        end: 49,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13db7f560,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                            ),
                                            start: 50,
                                            end: 62,
                                            as_str(): "a_dependency",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb13db7f560,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                        ),
                                        start: 62,
                                        end: 63,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fb13db7f560,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                        ),
                                        start: 65,
                                        end: 68,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13db7f560,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                            ),
                                            start: 69,
                                            end: 72,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb13db7f560,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                            ),
                                            start: 72,
                                            end: 74,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Rename {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb13db7f560,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                ),
                                                start: 74,
                                                end: 77,
                                                as_str(): "Foo",
                                            },
                                            is_raw_ident: false,
                                        },
                                        as_token: AsToken {
                                            span: Span {
                                                src (ptr): 0x00007fb13db7f560,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                ),
                                                start: 78,
                                                end: 80,
                                                as_str(): "as",
                                            },
                                        },
                                        alias: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb13db7f560,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                ),
                                                start: 81,
                                                end: 86,
                                                as_str(): "MyFoo",
                                            },
                                            is_raw_ident: false,
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb13db7f560,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                        ),
                                        start: 86,
                                        end: 87,
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
                                            src (ptr): 0x00007fb13db7f560,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                            ),
                                            start: 89,
                                            end: 91,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13db7f560,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                            ),
                                            start: 92,
                                            end: 96,
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
                                            src (ptr): 0x00007fb13db7f560,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                            ),
                                            start: 96,
                                            end: 98,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb13db7f560,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                    ),
                                                    start: 99,
                                                    end: 101,
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
                                                                src (ptr): 0x00007fb13db7f560,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                ),
                                                                start: 102,
                                                                end: 105,
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
                                                            src (ptr): 0x00007fb13db7f560,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 115,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb13db7f560,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                ),
                                                                start: 116,
                                                                end: 119,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13db7f560,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                            ),
                                                            start: 120,
                                                            end: 121,
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
                                                                        src (ptr): 0x00007fb13db7f560,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                        ),
                                                                        start: 122,
                                                                        end: 127,
                                                                        as_str(): "MyFoo",
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
                                                                                    src (ptr): 0x00007fb13db7f560,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                                    ),
                                                                                    start: 138,
                                                                                    end: 141,
                                                                                    as_str(): "foo",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb13db7f560,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                                            ),
                                                                                            start: 141,
                                                                                            end: 142,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb13db7f560,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                                                    ),
                                                                                                    start: 143,
                                                                                                    end: 145,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13db7f560,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                                ),
                                                                                start: 145,
                                                                                end: 146,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb13db7f560,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                ),
                                                                start: 128,
                                                                end: 152,
                                                                as_str(): "{\n        foo: 42,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13db7f560,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                            ),
                                                            start: 152,
                                                            end: 153,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            FieldProjection {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb13db7f560,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                                    ),
                                                                    start: 158,
                                                                    end: 161,
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
                                                dot_token: DotToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb13db7f560,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                        ),
                                                        start: 161,
                                                        end: 162,
                                                        as_str(): ".",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb13db7f560,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                        ),
                                                        start: 162,
                                                        end: 165,
                                                        as_str(): "foo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13db7f560,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                        ),
                                        start: 106,
                                        end: 167,
                                        as_str(): "{\n    let foo = MyFoo {\n        foo: 42,\n    };\n    foo.foo\n}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb13c982ff0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                            ),
                            start: 8,
                            end: 11,
                            as_str(): "foo",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb13c982ff0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                ),
                                start: 8,
                                end: 11,
                                as_str(): "foo",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fb13c982ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13c982ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                            ),
                                            start: 8,
                                            end: 11,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb13c982ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                        ),
                                        start: 11,
                                        end: 12,
                                        as_str(): ";",
                                    },
                                },
                                items: [
                                    Annotated {
                                        attribute_list: [],
                                        value: Struct(
                                            ItemStruct {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb13c982ff0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                                            ),
                                                            start: 14,
                                                            end: 17,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb13c982ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                                        ),
                                                        start: 18,
                                                        end: 24,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb13c982ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                                        ),
                                                        start: 25,
                                                        end: 28,
                                                        as_str(): "Foo",
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
                                                                                src (ptr): 0x00007fb13c982ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                                                                ),
                                                                                start: 35,
                                                                                end: 38,
                                                                                as_str(): "foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb13c982ff0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                                                                ),
                                                                                start: 38,
                                                                                end: 39,
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
                                                                                            src (ptr): 0x00007fb13c982ff0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                                                                            ),
                                                                                            start: 40,
                                                                                            end: 43,
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
                                                                        src (ptr): 0x00007fb13c982ff0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                                                        ),
                                                                        start: 43,
                                                                        end: 44,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb13c982ff0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                                        ),
                                                        start: 29,
                                                        end: 46,
                                                        as_str(): "{\n    foo: u64,\n}",
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
                ),
            ],
        },
    },
)
