Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe05b2d99e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe05b2d99e0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
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
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 10,
                                        end: 13,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 14,
                                            end: 17,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 17,
                                        end: 18,
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
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 20,
                                        end: 26,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 28,
                                        as_str(): "S",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 28,
                                                    end: 29,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 29,
                                                            end: 30,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 30,
                                                    end: 31,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 32,
                                        end: 35,
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
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 37,
                                        end: 41,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 41,
                                                    end: 42,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 42,
                                                            end: 43,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 43,
                                                    end: 44,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
                                trait_opt: None,
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 46,
                                                    as_str(): "S",
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
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 46,
                                                                    end: 47,
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
                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                        ),
                                                                                        start: 47,
                                                                                        end: 48,
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
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 48,
                                                                    end: 49,
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
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 54,
                                                            end: 56,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 57,
                                                            end: 58,
                                                            as_str(): "f",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 59,
                                                                    end: 63,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 58,
                                                            end: 64,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 65,
                                                                    end: 67,
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
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 68,
                                                                                end: 71,
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
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 78,
                                                                            end: 79,
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
                                                        src (ptr): 0x00007fe05b2d99e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                        ),
                                                        start: 72,
                                                        end: 83,
                                                        as_str(): "{\n    5\n  }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 50,
                                        end: 85,
                                        as_str(): "{\n  fn f(self) -> u64 {\n    5\n  }\n}",
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
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 87,
                                            end: 89,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 90,
                                            end: 94,
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
                                            src (ptr): 0x00007fe05b2d99e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                            ),
                                            start: 94,
                                            end: 96,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05b2d99e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                    ),
                                                    start: 97,
                                                    end: 99,
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
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 100,
                                                                end: 103,
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
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 111,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 112,
                                                                end: 113,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 115,
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 116,
                                                                        end: 117,
                                                                        as_str(): "S",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: Some(
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 117,
                                                                                end: 119,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        GenericArgs {
                                                                            parameters: AngleBrackets {
                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                        ),
                                                                                        start: 119,
                                                                                        end: 120,
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
                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                            ),
                                                                                                            start: 120,
                                                                                                            end: 123,
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
                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                        ),
                                                                                        start: 123,
                                                                                        end: 124,
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
                                                        fields: Braces {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 125,
                                                                end: 128,
                                                                as_str(): "{ }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 129,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 132,
                                                            end: 135,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 136,
                                                                end: 137,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 140,
                                                                        end: 143,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 143,
                                                                            end: 145,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 145,
                                                                                end: 148,
                                                                                as_str(): "baz",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                ),
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 148,
                                                                            end: 150,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 150,
                                                                                end: 163,
                                                                                as_str(): "ExampleStruct",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: Some(
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                        ),
                                                                                        start: 163,
                                                                                        end: 165,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                GenericArgs {
                                                                                    parameters: AngleBrackets {
                                                                                        open_angle_bracket_token: OpenAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                ),
                                                                                                start: 165,
                                                                                                end: 166,
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
                                                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 166,
                                                                                                                        end: 169,
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
                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                            ),
                                                                                                            start: 169,
                                                                                                            end: 170,
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
                                                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 171,
                                                                                                                    end: 175,
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
                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                ),
                                                                                                start: 175,
                                                                                                end: 176,
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
                                                        fields: Braces {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                    ),
                                                                                    start: 179,
                                                                                    end: 186,
                                                                                    as_str(): "a_field",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                            ),
                                                                                            start: 186,
                                                                                            end: 187,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                    ),
                                                                                                    start: 188,
                                                                                                    end: 189,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                            ),
                                                                                                            start: 189,
                                                                                                            end: 192,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 192,
                                                                                end: 193,
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
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 194,
                                                                                end: 201,
                                                                                as_str(): "b_field",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                        ),
                                                                                        start: 201,
                                                                                        end: 202,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Bool(
                                                                                        LitBool {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                ),
                                                                                                start: 203,
                                                                                                end: 207,
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
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 177,
                                                                end: 209,
                                                                as_str(): "{ a_field: 5u64, b_field: true }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 209,
                                                            end: 210,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Use(
                                                        ItemUse {
                                                            visibility: None,
                                                            use_token: UseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 213,
                                                                    end: 216,
                                                                    as_str(): "use",
                                                                },
                                                            },
                                                            root_import: None,
                                                            tree: Path {
                                                                prefix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 217,
                                                                        end: 220,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                double_colon_token: DoubleColonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 220,
                                                                        end: 222,
                                                                        as_str(): "::",
                                                                    },
                                                                },
                                                                suffix: Path {
                                                                    prefix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 222,
                                                                            end: 225,
                                                                            as_str(): "baz",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    double_colon_token: DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 225,
                                                                            end: 227,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    suffix: Name {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 227,
                                                                                end: 240,
                                                                                as_str(): "ExampleStruct",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            semicolon_token: SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 240,
                                                                    end: 241,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 244,
                                                            end: 247,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 248,
                                                                end: 249,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 250,
                                                            end: 251,
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
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 252,
                                                                        end: 255,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 255,
                                                                            end: 257,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 257,
                                                                                end: 260,
                                                                                as_str(): "baz",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                ),
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 260,
                                                                            end: 262,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 262,
                                                                                end: 266,
                                                                                as_str(): "quux",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                ),
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                            ),
                                                                            start: 266,
                                                                            end: 268,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 268,
                                                                                end: 272,
                                                                                as_str(): "Quux",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: Some(
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                        ),
                                                                                        start: 272,
                                                                                        end: 274,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                GenericArgs {
                                                                                    parameters: AngleBrackets {
                                                                                        open_angle_bracket_token: OpenAngleBracketToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                ),
                                                                                                start: 274,
                                                                                                end: 275,
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
                                                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 275,
                                                                                                                        end: 278,
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
                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                            ),
                                                                                                            start: 278,
                                                                                                            end: 279,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    Path(
                                                                                                        PathType {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathTypeSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 280,
                                                                                                                        end: 284,
                                                                                                                        as_str(): "bool",
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
                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                            ),
                                                                                                            start: 284,
                                                                                                            end: 285,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    Path(
                                                                                                        PathType {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathTypeSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 286,
                                                                                                                        end: 299,
                                                                                                                        as_str(): "ExampleStruct",
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
                                                                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 299,
                                                                                                                                        end: 300,
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
                                                                                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 300,
                                                                                                                                                                end: 303,
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
                                                                                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 303,
                                                                                                                                                    end: 304,
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
                                                                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 305,
                                                                                                                                                            end: 309,
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
                                                                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 309,
                                                                                                                                        end: 310,
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
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                            ),
                                                                                                            start: 310,
                                                                                                            end: 311,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    Path(
                                                                                                        PathType {
                                                                                                            root_opt: None,
                                                                                                            prefix: PathTypeSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 312,
                                                                                                                        end: 315,
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
                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                            ),
                                                                                                            start: 315,
                                                                                                            end: 316,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    Str {
                                                                                                        str_token: StrToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                ),
                                                                                                                start: 317,
                                                                                                                end: 320,
                                                                                                                as_str(): "str",
                                                                                                            },
                                                                                                        },
                                                                                                        length: SquareBrackets {
                                                                                                            inner: Literal(
                                                                                                                Int(
                                                                                                                    LitInt {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 321,
                                                                                                                            end: 322,
                                                                                                                            as_str(): "3",
                                                                                                                        },
                                                                                                                        parsed: 3,
                                                                                                                        ty_opt: None,
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                ),
                                                                                                                start: 320,
                                                                                                                end: 323,
                                                                                                                as_str(): "[3]",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                            ),
                                                                                                            start: 323,
                                                                                                            end: 324,
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
                                                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 325,
                                                                                                                    end: 328,
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
                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                ),
                                                                                                start: 328,
                                                                                                end: 329,
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
                                                        fields: Braces {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                    ),
                                                                                    start: 332,
                                                                                    end: 333,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                            ),
                                                                                            start: 333,
                                                                                            end: 334,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                    ),
                                                                                                    start: 335,
                                                                                                    end: 337,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 337,
                                                                                end: 338,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                    ),
                                                                                    start: 339,
                                                                                    end: 340,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                            ),
                                                                                            start: 340,
                                                                                            end: 341,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                    ),
                                                                                                    start: 342,
                                                                                                    end: 346,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 346,
                                                                                end: 347,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                    ),
                                                                                    start: 348,
                                                                                    end: 349,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                            ),
                                                                                            start: 349,
                                                                                            end: 350,
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
                                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                        ),
                                                                                                        start: 351,
                                                                                                        end: 352,
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
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 352,
                                                                                end: 353,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                    ),
                                                                                    start: 354,
                                                                                    end: 355,
                                                                                    as_str(): "d",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                            ),
                                                                                            start: 355,
                                                                                            end: 356,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                    ),
                                                                                                    start: 357,
                                                                                                    end: 359,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 359,
                                                                                end: 360,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                    ),
                                                                                    start: 361,
                                                                                    end: 362,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05b2d99e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                            ),
                                                                                            start: 362,
                                                                                            end: 363,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        String(
                                                                                            LitString {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                    ),
                                                                                                    start: 364,
                                                                                                    end: 369,
                                                                                                    as_str(): "\"foo\"",
                                                                                                },
                                                                                                parsed: "foo",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 369,
                                                                                end: 370,
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
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 371,
                                                                                end: 372,
                                                                                as_str(): "f",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                        ),
                                                                                        start: 372,
                                                                                        end: 373,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                                ),
                                                                                                start: 374,
                                                                                                end: 376,
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
                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                ),
                                                                start: 330,
                                                                end: 378,
                                                                as_str(): "{ a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 378,
                                                            end: 379,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 382,
                                                            end: 388,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        MethodCall {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05b2d99e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                                ),
                                                                                start: 389,
                                                                                end: 390,
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
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 390,
                                                                    end: 391,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            path_seg: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b2d99e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                        ),
                                                                        start: 391,
                                                                        end: 392,
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
                                                                    src (ptr): 0x00007fe05b2d99e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                                    ),
                                                                    start: 392,
                                                                    end: 394,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b2d99e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                                            ),
                                                            start: 394,
                                                            end: 395,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05b2d99e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/main.sw",
                                        ),
                                        start: 104,
                                        end: 397,
                                        as_str(): "{\n  let a = S::<u64> { };\n  let b = foo::baz::ExampleStruct::<u64, bool> { a_field: 5u64, b_field: true };\n  use foo::baz::ExampleStruct;\n  let c = foo::baz::quux::Quux::<u64, bool, ExampleStruct<u64, bool>, u64, str[3], u64> { a: 10, b: true, c: b, d: 10, e: \"foo\", f: 10 };\n  return a.f();\n}",
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
                            src (ptr): 0x00007fe05b556510,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
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
                                src (ptr): 0x00007fe05b556510,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
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
                                            src (ptr): 0x00007fe05b556510,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05b556510,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
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
                                        src (ptr): 0x00007fe05b556510,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
                                        ),
                                        start: 11,
                                        end: 12,
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
                                                        src (ptr): 0x00007fe05b556510,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
                                                        ),
                                                        start: 14,
                                                        end: 17,
                                                        as_str(): "dep",
                                                    },
                                                },
                                                path: DependencyPath {
                                                    prefix: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b556510,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
                                                            ),
                                                            start: 18,
                                                            end: 21,
                                                            as_str(): "bar",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    suffixes: [
                                                        (
                                                            ForwardSlashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b556510,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
                                                                    ),
                                                                    start: 21,
                                                                    end: 22,
                                                                    as_str(): "/",
                                                                },
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05b556510,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
                                                                    ),
                                                                    start: 22,
                                                                    end: 25,
                                                                    as_str(): "baz",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                    ],
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe05b556510,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/foo.sw",
                                                        ),
                                                        start: 25,
                                                        end: 26,
                                                        as_str(): ";",
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
                                            src (ptr): 0x00007fe05b1b2b70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                            ),
                                            start: 8,
                                            end: 11,
                                            as_str(): "baz",
                                        },
                                        is_raw_ident: false,
                                    },
                                    LexedSubmodule {
                                        library_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe05b1b2b70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                ),
                                                start: 8,
                                                end: 11,
                                                as_str(): "baz",
                                            },
                                            is_raw_ident: false,
                                        },
                                        module: LexedModule {
                                            tree: Module {
                                                kind: Library {
                                                    library_token: LibraryToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b1b2b70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                            ),
                                                            start: 0,
                                                            end: 7,
                                                            as_str(): "library",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05b1b2b70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                            ),
                                                            start: 8,
                                                            end: 11,
                                                            as_str(): "baz",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe05b1b2b70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                        ),
                                                        start: 11,
                                                        end: 12,
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
                                                                        src (ptr): 0x00007fe05b1b2b70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                        ),
                                                                        start: 14,
                                                                        end: 17,
                                                                        as_str(): "dep",
                                                                    },
                                                                },
                                                                path: DependencyPath {
                                                                    prefix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b1b2b70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                            ),
                                                                            start: 18,
                                                                            end: 22,
                                                                            as_str(): "quux",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    suffixes: [],
                                                                },
                                                                semicolon_token: SemicolonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b1b2b70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                        ),
                                                                        start: 22,
                                                                        end: 23,
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
                                                                visibility: Some(
                                                                    PubToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b1b2b70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                            ),
                                                                            start: 25,
                                                                            end: 28,
                                                                            as_str(): "pub",
                                                                        },
                                                                    },
                                                                ),
                                                                struct_token: StructToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b1b2b70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                        ),
                                                                        start: 29,
                                                                        end: 35,
                                                                        as_str(): "struct",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b1b2b70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                        ),
                                                                        start: 36,
                                                                        end: 49,
                                                                        as_str(): "ExampleStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics: Some(
                                                                    GenericParams {
                                                                        parameters: AngleBrackets {
                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05b1b2b70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                    ),
                                                                                    start: 49,
                                                                                    end: 50,
                                                                                    as_str(): "<",
                                                                                },
                                                                            },
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05b1b2b70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                                ),
                                                                                                start: 50,
                                                                                                end: 51,
                                                                                                as_str(): "T",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05b1b2b70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                                ),
                                                                                                start: 51,
                                                                                                end: 52,
                                                                                                as_str(): ",",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                ],
                                                                                final_value_opt: Some(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05b1b2b70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                            ),
                                                                                            start: 53,
                                                                                            end: 54,
                                                                                            as_str(): "U",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                            },
                                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05b1b2b70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                    ),
                                                                                    start: 54,
                                                                                    end: 55,
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
                                                                                                src (ptr): 0x00007fe05b1b2b70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                                ),
                                                                                                start: 60,
                                                                                                end: 67,
                                                                                                as_str(): "a_field",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        colon_token: ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05b1b2b70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                                ),
                                                                                                start: 67,
                                                                                                end: 68,
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
                                                                                                            src (ptr): 0x00007fe05b1b2b70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                                            ),
                                                                                                            start: 69,
                                                                                                            end: 70,
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
                                                                                        src (ptr): 0x00007fe05b1b2b70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                        ),
                                                                                        start: 70,
                                                                                        end: 71,
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
                                                                                            src (ptr): 0x00007fe05b1b2b70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                            ),
                                                                                            start: 74,
                                                                                            end: 81,
                                                                                            as_str(): "b_field",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    colon_token: ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe05b1b2b70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                            ),
                                                                                            start: 81,
                                                                                            end: 82,
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
                                                                                                        src (ptr): 0x00007fe05b1b2b70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                                                        ),
                                                                                                        start: 83,
                                                                                                        end: 84,
                                                                                                        as_str(): "U",
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
                                                                        src (ptr): 0x00007fe05b1b2b70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/baz.sw",
                                                                        ),
                                                                        start: 56,
                                                                        end: 87,
                                                                        as_str(): "{\n  a_field: T,\n  b_field: U \n}",
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
                                                            src (ptr): 0x00007fe05b1a68a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                            ),
                                                            start: 8,
                                                            end: 12,
                                                            as_str(): "quux",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    LexedSubmodule {
                                                        library_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                ),
                                                                start: 8,
                                                                end: 12,
                                                                as_str(): "quux",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        module: LexedModule {
                                                            tree: Module {
                                                                kind: Library {
                                                                    library_token: LibraryToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                            ),
                                                                            start: 0,
                                                                            end: 7,
                                                                            as_str(): "library",
                                                                        },
                                                                    },
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                            ),
                                                                            start: 8,
                                                                            end: 12,
                                                                            as_str(): "quux",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                semicolon_token: SemicolonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                        ),
                                                                        start: 12,
                                                                        end: 13,
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
                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                            ),
                                                                                            start: 14,
                                                                                            end: 17,
                                                                                            as_str(): "pub",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                struct_token: StructToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                        ),
                                                                                        start: 18,
                                                                                        end: 24,
                                                                                        as_str(): "struct",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                        ),
                                                                                        start: 25,
                                                                                        end: 29,
                                                                                        as_str(): "Quux",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics: Some(
                                                                                    GenericParams {
                                                                                        parameters: AngleBrackets {
                                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe05b1a68a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                    ),
                                                                                                    start: 29,
                                                                                                    end: 30,
                                                                                                    as_str(): "<",
                                                                                                },
                                                                                            },
                                                                                            inner: Punctuated {
                                                                                                value_separator_pairs: [
                                                                                                    (
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 30,
                                                                                                                end: 31,
                                                                                                                as_str(): "A",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 31,
                                                                                                                end: 32,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 33,
                                                                                                                end: 34,
                                                                                                                as_str(): "B",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 34,
                                                                                                                end: 35,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 36,
                                                                                                                end: 37,
                                                                                                                as_str(): "C",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 37,
                                                                                                                end: 38,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 39,
                                                                                                                end: 40,
                                                                                                                as_str(): "D",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 40,
                                                                                                                end: 41,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    (
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 42,
                                                                                                                end: 43,
                                                                                                                as_str(): "E",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        CommaToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 43,
                                                                                                                end: 44,
                                                                                                                as_str(): ",",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                ],
                                                                                                final_value_opt: Some(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                            ),
                                                                                                            start: 45,
                                                                                                            end: 46,
                                                                                                            as_str(): "F",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe05b1a68a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                    ),
                                                                                                    start: 46,
                                                                                                    end: 47,
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
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 52,
                                                                                                                end: 53,
                                                                                                                as_str(): "a",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        colon_token: ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 53,
                                                                                                                end: 54,
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
                                                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                            ),
                                                                                                                            start: 55,
                                                                                                                            end: 56,
                                                                                                                            as_str(): "A",
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
                                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                        ),
                                                                                                        start: 56,
                                                                                                        end: 57,
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
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 60,
                                                                                                                end: 61,
                                                                                                                as_str(): "b",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        colon_token: ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
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
                                                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                            ),
                                                                                                                            start: 63,
                                                                                                                            end: 64,
                                                                                                                            as_str(): "B",
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
                                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
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
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 68,
                                                                                                                end: 69,
                                                                                                                as_str(): "c",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        colon_token: ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 69,
                                                                                                                end: 70,
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
                                                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                            ),
                                                                                                                            start: 71,
                                                                                                                            end: 72,
                                                                                                                            as_str(): "C",
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
                                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                        ),
                                                                                                        start: 72,
                                                                                                        end: 73,
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
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 76,
                                                                                                                end: 77,
                                                                                                                as_str(): "d",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        colon_token: ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 77,
                                                                                                                end: 78,
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
                                                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                            ),
                                                                                                                            start: 79,
                                                                                                                            end: 80,
                                                                                                                            as_str(): "D",
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
                                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                        ),
                                                                                                        start: 80,
                                                                                                        end: 81,
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
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 84,
                                                                                                                end: 85,
                                                                                                                as_str(): "e",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        colon_token: ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe05b1a68a0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                ),
                                                                                                                start: 85,
                                                                                                                end: 86,
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
                                                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                            ),
                                                                                                                            start: 87,
                                                                                                                            end: 88,
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
                                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                        ),
                                                                                                        start: 88,
                                                                                                        end: 89,
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
                                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                            ),
                                                                                                            start: 92,
                                                                                                            end: 93,
                                                                                                            as_str(): "f",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    colon_token: ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe05b1a68a0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                            ),
                                                                                                            start: 93,
                                                                                                            end: 94,
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
                                                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                                                        ),
                                                                                                                        start: 95,
                                                                                                                        end: 96,
                                                                                                                        as_str(): "F",
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
                                                                                        src (ptr): 0x00007fe05b1a68a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRFqHLwL/primitive_type_argument/src/bar/quux.sw",
                                                                                        ),
                                                                                        start: 48,
                                                                                        end: 98,
                                                                                        as_str(): "{\n  a: A,\n  b: B,\n  c: C,\n  d: D,\n  e: E,\n  f: F\n}",
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
                                ),
                            ],
                        },
                    },
                ),
            ],
        },
    },
)
