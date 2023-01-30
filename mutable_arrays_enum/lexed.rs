Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0935f24b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0935f24b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
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
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 17,
                                        as_str(): "X",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Annotated {
                                                attribute_list: [],
                                                value: TypeField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 24,
                                                            end: 29,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 29,
                                                            end: 30,
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
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 31,
                                                                        end: 34,
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
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 36,
                                        as_str(): "{\n    value: u64\n}",
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
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 38,
                                        end: 42,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 46,
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
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 53,
                                                                end: 56,
                                                                as_str(): "Bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 56,
                                                                end: 57,
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
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 58,
                                                                            end: 59,
                                                                            as_str(): "X",
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
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 59,
                                                        end: 60,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 47,
                                        end: 62,
                                        as_str(): "{\n    Bar: X,\n}",
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
                                            src (ptr): 0x00007fe0935f24b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                            ),
                                            start: 64,
                                            end: 66,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0935f24b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                            ),
                                            start: 67,
                                            end: 71,
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
                                            src (ptr): 0x00007fe0935f24b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                            ),
                                            start: 71,
                                            end: 73,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0935f24b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                    ),
                                                    start: 74,
                                                    end: 76,
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
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 77,
                                                                end: 80,
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
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 87,
                                                            end: 90,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 91,
                                                                    end: 94,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 95,
                                                                end: 103,
                                                                as_str(): "my_array",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 103,
                                                                    end: 104,
                                                                    as_str(): ":",
                                                                },
                                                            },
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
                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                            ),
                                                                                            start: 106,
                                                                                            end: 109,
                                                                                            as_str(): "Foo",
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
                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 111,
                                                                                        end: 112,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 105,
                                                                        end: 113,
                                                                        as_str(): "[Foo; 1]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 115,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
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
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 117,
                                                                                                end: 120,
                                                                                                as_str(): "Foo",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        generics_opt: None,
                                                                                    },
                                                                                    suffix: [
                                                                                        (
                                                                                            DoubleColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 120,
                                                                                                    end: 122,
                                                                                                    as_str(): "::",
                                                                                                },
                                                                                            },
                                                                                            PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 122,
                                                                                                        end: 125,
                                                                                                        as_str(): "Bar",
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
                                                                                        Struct {
                                                                                            path: PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                            ),
                                                                                                            start: 126,
                                                                                                            end: 127,
                                                                                                            as_str(): "X",
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
                                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 128,
                                                                                                                    end: 133,
                                                                                                                    as_str(): "value",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            expr_opt: Some(
                                                                                                                (
                                                                                                                    ColonToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 133,
                                                                                                                            end: 134,
                                                                                                                            as_str(): ":",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    Literal(
                                                                                                                        Int(
                                                                                                                            LitInt {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 135,
                                                                                                                                    end: 137,
                                                                                                                                    as_str(): "10",
                                                                                                                                },
                                                                                                                                parsed: 10,
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
                                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                    ),
                                                                                                    start: 127,
                                                                                                    end: 138,
                                                                                                    as_str(): "{value: 10}",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 125,
                                                                                    end: 139,
                                                                                    as_str(): "(X{value: 10})",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 116,
                                                                end: 140,
                                                                as_str(): "[Foo::Bar(X{value: 10})]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 140,
                                                            end: 141,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Index {
                                                        target: Var(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 146,
                                                                    end: 154,
                                                                    as_str(): "my_array",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        arg: SquareBrackets {
                                                            inner: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 155,
                                                                            end: 156,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 154,
                                                                end: 157,
                                                                as_str(): "[0]",
                                                            },
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 159,
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
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 160,
                                                                            end: 163,
                                                                            as_str(): "Foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                ),
                                                                                start: 163,
                                                                                end: 165,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 165,
                                                                                    end: 168,
                                                                                    as_str(): "Bar",
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
                                                                    Struct {
                                                                        path: PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 169,
                                                                                        end: 170,
                                                                                        as_str(): "X",
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
                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                ),
                                                                                                start: 171,
                                                                                                end: 176,
                                                                                                as_str(): "value",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        expr_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                        ),
                                                                                                        start: 176,
                                                                                                        end: 177,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                                                ),
                                                                                                                start: 178,
                                                                                                                end: 180,
                                                                                                                as_str(): "20",
                                                                                                            },
                                                                                                            parsed: 20,
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
                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                ),
                                                                                start: 170,
                                                                                end: 181,
                                                                                as_str(): "{value: 20}",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0935f24b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                ),
                                                                start: 168,
                                                                end: 182,
                                                                as_str(): "(X{value: 20})",
                                                            },
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 183,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Match {
                                                match_token: MatchToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 188,
                                                        end: 193,
                                                        as_str(): "match",
                                                    },
                                                },
                                                value: Index {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 194,
                                                                        end: 202,
                                                                        as_str(): "my_array",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    arg: SquareBrackets {
                                                        inner: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 203,
                                                                        end: 204,
                                                                        as_str(): "0",
                                                                    },
                                                                    parsed: 0,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0935f24b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                            ),
                                                            start: 202,
                                                            end: 205,
                                                            as_str(): "[0]",
                                                        },
                                                    },
                                                },
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
                                                                                src (ptr): 0x00007fe0935f24b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                ),
                                                                                start: 216,
                                                                                end: 219,
                                                                                as_str(): "Foo",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [
                                                                        (
                                                                            DoubleColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                    ),
                                                                                    start: 219,
                                                                                    end: 221,
                                                                                    as_str(): "::",
                                                                                },
                                                                            },
                                                                            PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 221,
                                                                                        end: 224,
                                                                                        as_str(): "Bar",
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
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 225,
                                                                                        end: 226,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 224,
                                                                        end: 227,
                                                                        as_str(): "(x)",
                                                                    },
                                                                },
                                                            },
                                                            fat_right_arrow_token: FatRightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0935f24b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                    ),
                                                                    start: 228,
                                                                    end: 230,
                                                                    as_str(): "=>",
                                                                },
                                                            },
                                                            kind: Expr {
                                                                expr: FieldProjection {
                                                                    target: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                                        ),
                                                                                        start: 231,
                                                                                        end: 232,
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
                                                                    dot_token: DotToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 232,
                                                                            end: 233,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0935f24b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                            ),
                                                                            start: 233,
                                                                            end: 238,
                                                                            as_str(): "value",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0935f24b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                                        ),
                                                                        start: 238,
                                                                        end: 239,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0935f24b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                                        ),
                                                        start: 206,
                                                        end: 245,
                                                        as_str(): "{\n        Foo::Bar(x) => x.value,\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0935f24b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRd5lufm/mutable_arrays_enum/src/main.sw",
                                        ),
                                        start: 81,
                                        end: 247,
                                        as_str(): "{\n    let mut my_array: [Foo; 1] = [Foo::Bar(X{value: 10})];\n    my_array[0] = Foo::Bar(X{value: 20});\n    match my_array[0] {\n        Foo::Bar(x) => x.value,\n    }\n}",
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
