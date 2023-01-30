Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0f9b4a470,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0f9b4a470,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
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
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 19,
                                        as_str(): "Foo",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 20,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f9b4a470,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                            ),
                                                            start: 20,
                                                            end: 21,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 21,
                                                    end: 22,
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
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 27,
                                                                end: 28,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 28,
                                                                end: 29,
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
                                                                            src (ptr): 0x00007fe0f9b4a470,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                            ),
                                                                            start: 30,
                                                                            end: 31,
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
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 31,
                                                        end: 32,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 23,
                                        end: 34,
                                        as_str(): "{\n  a: T,\n}",
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
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 36,
                                            end: 38,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 39,
                                            end: 44,
                                            as_str(): "get_a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 44,
                                                        end: 45,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 45,
                                                                end: 46,
                                                                as_str(): "V",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 46,
                                                        end: 47,
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
                                                                    src (ptr): 0x00007fe0f9b4a470,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 51,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0f9b4a470,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                            ),
                                                                            start: 53,
                                                                            end: 56,
                                                                            as_str(): "Foo",
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
                                                                                            src (ptr): 0x00007fe0f9b4a470,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 56,
                                                                                            end: 57,
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
                                                                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                                                ),
                                                                                                                start: 57,
                                                                                                                end: 58,
                                                                                                                as_str(): "V",
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
                                                                                            src (ptr): 0x00007fe0f9b4a470,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 58,
                                                                                            end: 59,
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
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 47,
                                            end: 60,
                                            as_str(): "(foo: Foo<V>)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 61,
                                                    end: 63,
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
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 65,
                                                                as_str(): "V",
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
                                            FieldProjection {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f9b4a470,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                    ),
                                                                    start: 70,
                                                                    end: 73,
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
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 73,
                                                        end: 74,
                                                        as_str(): ".",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 74,
                                                        end: 75,
                                                        as_str(): "a",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 66,
                                        end: 77,
                                        as_str(): "{\n  foo.a\n}",
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
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 79,
                                            end: 81,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 82,
                                            end: 86,
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
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 86,
                                            end: 88,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 89,
                                                    end: 91,
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
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 96,
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
                                                            src (ptr): 0x00007fe0f9b4a470,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 104,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 108,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f9b4a470,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                            ),
                                                            start: 109,
                                                            end: 110,
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
                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                        ),
                                                                        start: 111,
                                                                        end: 114,
                                                                        as_str(): "Foo",
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
                                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                ),
                                                                                start: 117,
                                                                                end: 118,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 118,
                                                                                        end: 119,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 120,
                                                                                                end: 124,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                            kind: True,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 115,
                                                                end: 126,
                                                                as_str(): "{ a: true }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f9b4a470,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                            ),
                                                            start: 126,
                                                            end: 127,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f9b4a470,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                            ),
                                                            start: 130,
                                                            end: 133,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 137,
                                                                as_str(): "bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f9b4a470,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                            ),
                                                            start: 138,
                                                            end: 139,
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
                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                        ),
                                                                        start: 140,
                                                                        end: 143,
                                                                        as_str(): "Foo",
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
                                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                ),
                                                                                start: 146,
                                                                                end: 147,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f9b4a470,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 147,
                                                                                        end: 148,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 149,
                                                                                                end: 151,
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
                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                ),
                                                                start: 144,
                                                                end: 153,
                                                                as_str(): "{ a: 10 }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f9b4a470,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                            ),
                                                            start: 153,
                                                            end: 154,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f9b4a470,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                    ),
                                                                    start: 158,
                                                                    end: 163,
                                                                    as_str(): "get_a",
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
                                                                                src (ptr): 0x00007fe0f9b4a470,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                                                ),
                                                                                start: 164,
                                                                                end: 167,
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
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f9b4a470,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                        ),
                                                        start: 163,
                                                        end: 168,
                                                        as_str(): "(foo)",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 97,
                                        end: 170,
                                        as_str(): "{\n  let foo = Foo { a: true };\n  let bar = Foo { a: 10 };\n\n  get_a(foo)\n}",
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
