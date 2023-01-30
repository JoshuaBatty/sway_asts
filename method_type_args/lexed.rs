Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0800b2770,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0800b2770,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
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
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
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
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 20,
                                        as_str(): "{}",
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
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 22,
                                        end: 26,
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
                                                    src (ptr): 0x00007fe0800b2770,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                    ),
                                                    start: 27,
                                                    end: 28,
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
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 35,
                                                            end: 37,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 38,
                                                            end: 45,
                                                            as_str(): "generic",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: Some(
                                                        GenericParams {
                                                            parameters: AngleBrackets {
                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0800b2770,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                        ),
                                                                        start: 45,
                                                                        end: 46,
                                                                        as_str(): "<",
                                                                    },
                                                                },
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0800b2770,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                                ),
                                                                                start: 46,
                                                                                end: 47,
                                                                                as_str(): "T",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                },
                                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0800b2770,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                        ),
                                                                        start: 47,
                                                                        end: 48,
                                                                        as_str(): ">",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0800b2770,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                    ),
                                                                    start: 49,
                                                                    end: 53,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0800b2770,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                            ),
                                                                            start: 53,
                                                                            end: 54,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
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
                                                                                            src (ptr): 0x00007fe0800b2770,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                                            ),
                                                                                            start: 55,
                                                                                            end: 56,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0800b2770,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe0800b2770,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                                                    ),
                                                                                                    start: 58,
                                                                                                    end: 59,
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
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 48,
                                                            end: 60,
                                                            as_str(): "(self, x: T)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0800b2770,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
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
                                                                                src (ptr): 0x00007fe0800b2770,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                                ),
                                                                                start: 64,
                                                                                end: 65,
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
                                                            Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0800b2770,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                                ),
                                                                                start: 68,
                                                                                end: 69,
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
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0800b2770,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                        ),
                                                        start: 66,
                                                        end: 71,
                                                        as_str(): "{ x }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 29,
                                        end: 73,
                                        as_str(): "{\n    fn generic<T>(self, x: T) -> T { x }\n}",
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
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 75,
                                            end: 77,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 81,
                                            as_str(): "foo",
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
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 83,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0800b2770,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                    ),
                                                    start: 84,
                                                    end: 86,
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
                                                                src (ptr): 0x00007fe0800b2770,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                ),
                                                                start: 87,
                                                                end: 91,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            MethodCall {
                                                target: Struct {
                                                    path: PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0800b2770,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                    ),
                                                                    start: 98,
                                                                    end: 99,
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
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 102,
                                                            as_str(): "{}",
                                                        },
                                                    },
                                                },
                                                dot_token: DotToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0800b2770,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                        ),
                                                        start: 102,
                                                        end: 103,
                                                        as_str(): ".",
                                                    },
                                                },
                                                path_seg: PathExprSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 103,
                                                            end: 110,
                                                            as_str(): "generic",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: Some(
                                                        (
                                                            DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0800b2770,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                    ),
                                                                    start: 110,
                                                                    end: 112,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            GenericArgs {
                                                                parameters: AngleBrackets {
                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0800b2770,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 113,
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
                                                                                                src (ptr): 0x00007fe0800b2770,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                                                ),
                                                                                                start: 113,
                                                                                                end: 117,
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
                                                                            src (ptr): 0x00007fe0800b2770,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                            ),
                                                                            start: 117,
                                                                            end: 118,
                                                                            as_str(): ">",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                                contract_args_opt: None,
                                                args: Parens {
                                                    inner: Punctuated {
                                                        value_separator_pairs: [],
                                                        final_value_opt: Some(
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0800b2770,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                            ),
                                                                            start: 119,
                                                                            end: 123,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0800b2770,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                        ),
                                                        start: 118,
                                                        end: 124,
                                                        as_str(): "(true)",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 126,
                                        as_str(): "{\n    A {}.generic::<bool>(true)\n}",
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
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 128,
                                            end: 130,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
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
                                            src (ptr): 0x00007fe0800b2770,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                            ),
                                            start: 135,
                                            end: 137,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 144,
                                                            end: 147,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Wildcard {
                                                        underscore_token: UnderscoreToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0800b2770,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                ),
                                                                start: 148,
                                                                end: 149,
                                                                as_str(): "_",
                                                            },
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 150,
                                                            end: 151,
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
                                                                            src (ptr): 0x00007fe0800b2770,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                            ),
                                                                            start: 152,
                                                                            end: 155,
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0800b2770,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                                ),
                                                                start: 155,
                                                                end: 157,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0800b2770,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                                            ),
                                                            start: 157,
                                                            end: 158,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0800b2770,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDXajOg/method_type_args/src/main.sw",
                                        ),
                                        start: 138,
                                        end: 160,
                                        as_str(): "{\n    let _ = foo();\n}",
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
