Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 34,
                                        end: 40,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 45,
                                        as_str(): "Foo1",
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 52,
                                                                end: 53,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 55,
                                                                            end: 58,
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 58,
                                                        end: 59,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 64,
                                                                end: 65,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 65,
                                                                end: 66,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 67,
                                                                            end: 70,
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 70,
                                                        end: 71,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 46,
                                        end: 73,
                                        as_str(): "{\n    a: u64,\n    b: u64,\n}",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 75,
                                        end: 79,
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 80,
                                                    end: 84,
                                                    as_str(): "Foo1",
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 91,
                                                            end: 93,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 94,
                                                            end: 101,
                                                            as_str(): "trivial",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 102,
                                                                    end: 106,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 101,
                                                            end: 107,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 108,
                                                                    end: 110,
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 111,
                                                                                end: 115,
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
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 124,
                                                                            end: 129,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 116,
                                                        end: 135,
                                                        as_str(): "{\n      false\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 85,
                                        end: 137,
                                        as_str(): "{\n    fn trivial(self) -> bool {\n      false\n    }\n}",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 139,
                                            end: 141,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 142,
                                            end: 147,
                                            as_str(): "func1",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 147,
                                            end: 149,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 150,
                                                    end: 152,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 153,
                                                                end: 157,
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 167,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 168,
                                                                end: 169,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 170,
                                                            end: 171,
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
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 176,
                                                                        as_str(): "Foo1",
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
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 178,
                                                                                    end: 179,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 179,
                                                                                            end: 180,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 181,
                                                                                                    end: 182,
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 182,
                                                                                end: 183,
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 184,
                                                                                end: 185,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 185,
                                                                                        end: 186,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 187,
                                                                                                end: 188,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 177,
                                                                end: 189,
                                                                as_str(): "{a: 0, b: 0}",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 189,
                                                            end: 190,
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 195,
                                                                    end: 196,
                                                                    as_str(): "f",
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 196,
                                                        end: 197,
                                                        as_str(): ".",
                                                    },
                                                },
                                                path_seg: PathExprSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 197,
                                                            end: 204,
                                                            as_str(): "trivial",
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 204,
                                                        end: 206,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 208,
                                        as_str(): "{\n    let f = Foo1 {a: 0, b: 0};\n    f.trivial()\n}",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 235,
                                        end: 239,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 240,
                                        end: 243,
                                        as_str(): "Bar",
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 250,
                                                                end: 251,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 251,
                                                                end: 252,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 253,
                                                                    end: 255,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 255,
                                                        end: 256,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 261,
                                                                end: 262,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 262,
                                                                end: 263,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 264,
                                                                    end: 266,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 266,
                                                        end: 267,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 244,
                                        end: 269,
                                        as_str(): "{\n    a: (),\n    b: (),\n}",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 271,
                                        end: 275,
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 276,
                                                    end: 279,
                                                    as_str(): "Bar",
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 288,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 289,
                                                            end: 296,
                                                            as_str(): "trivial",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 297,
                                                                    end: 301,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 296,
                                                            end: 302,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 303,
                                                                    end: 305,
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 306,
                                                                                end: 310,
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
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 321,
                                                                            end: 326,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 311,
                                                        end: 332,
                                                        as_str(): "{\n        false\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 280,
                                        end: 334,
                                        as_str(): "{\n    fn trivial(self) -> bool {\n        false\n    }\n}",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 336,
                                            end: 338,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 339,
                                            end: 344,
                                            as_str(): "func2",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 345,
                                                                    end: 346,
                                                                    as_str(): "m",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 346,
                                                                end: 347,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 348,
                                                                            end: 351,
                                                                            as_str(): "Bar",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 344,
                                            end: 352,
                                            as_str(): "(m: Bar)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 353,
                                                    end: 355,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 356,
                                                                end: 360,
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
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 367,
                                                                    end: 368,
                                                                    as_str(): "m",
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 368,
                                                        end: 369,
                                                        as_str(): ".",
                                                    },
                                                },
                                                path_seg: PathExprSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 369,
                                                            end: 376,
                                                            as_str(): "trivial",
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 376,
                                                        end: 378,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 361,
                                        end: 380,
                                        as_str(): "{\n    m.trivial()\n}",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 407,
                                        end: 413,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 414,
                                        end: 418,
                                        as_str(): "Foo2",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 418,
                                                    end: 419,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 419,
                                                            end: 420,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 420,
                                                    end: 421,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 428,
                                                                end: 431,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 431,
                                                                end: 432,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 433,
                                                                            end: 434,
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 434,
                                                        end: 435,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 422,
                                        end: 437,
                                        as_str(): "{\n    foo: T,\n}",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 439,
                                        end: 443,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 443,
                                                    end: 444,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 444,
                                                            end: 445,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 445,
                                                    end: 446,
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 447,
                                                    end: 451,
                                                    as_str(): "Foo2",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 451,
                                                                    end: 452,
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
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 452,
                                                                                        end: 453,
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 453,
                                                                    end: 454,
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 461,
                                                            end: 463,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 464,
                                                            end: 471,
                                                            as_str(): "trivial",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 472,
                                                                    end: 476,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 471,
                                                            end: 477,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 478,
                                                                    end: 480,
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 481,
                                                                                end: 485,
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
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 496,
                                                                            end: 501,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 486,
                                                        end: 507,
                                                        as_str(): "{\n        false\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 455,
                                        end: 509,
                                        as_str(): "{\n    fn trivial(self) -> bool {\n        false\n    }\n}",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 511,
                                            end: 513,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 514,
                                            end: 519,
                                            as_str(): "func3",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 520,
                                                                    end: 521,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 521,
                                                                end: 522,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 523,
                                                                            end: 527,
                                                                            as_str(): "Foo2",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 527,
                                                                                            end: 528,
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
                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                ),
                                                                                                                start: 528,
                                                                                                                end: 530,
                                                                                                                as_str(): "u8",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 530,
                                                                                            end: 531,
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 519,
                                            end: 532,
                                            as_str(): "(a: Foo2<u8>)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 533,
                                                    end: 535,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 536,
                                                                end: 540,
                                                                as_str(): "Foo2",
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 540,
                                                                                end: 541,
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
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 541,
                                                                                                    end: 545,
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 545,
                                                                                end: 546,
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 553,
                                                            end: 555,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        MethodCall {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 556,
                                                                                end: 557,
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 557,
                                                                    end: 558,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            path_seg: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 558,
                                                                        end: 565,
                                                                        as_str(): "trivial",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 565,
                                                                    end: 567,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Struct {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 578,
                                                                                    end: 582,
                                                                                    as_str(): "Foo2",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 584,
                                                                                            end: 587,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 587,
                                                                                                    end: 588,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Literal(
                                                                                                Bool(
                                                                                                    LitBool {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 589,
                                                                                                            end: 594,
                                                                                                            as_str(): "false",
                                                                                                        },
                                                                                                        kind: False,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 583,
                                                                            end: 595,
                                                                            as_str(): "{foo: false}",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 568,
                                                            end: 601,
                                                            as_str(): "{\n        Foo2 {foo: false}\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 602,
                                                                    end: 606,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Struct {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 617,
                                                                                                end: 621,
                                                                                                as_str(): "Foo2",
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
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 623,
                                                                                                        end: 626,
                                                                                                        as_str(): "foo",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                expr_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                ),
                                                                                                                start: 626,
                                                                                                                end: 627,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Literal(
                                                                                                            Bool(
                                                                                                                LitBool {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 628,
                                                                                                                        end: 632,
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
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 622,
                                                                                        end: 633,
                                                                                        as_str(): "{foo: true}",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 607,
                                                                        end: 639,
                                                                        as_str(): "{\n        Foo2 {foo: true}\n    }",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 547,
                                        end: 641,
                                        as_str(): "{\n    if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }\n}",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 643,
                                            end: 645,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 646,
                                            end: 651,
                                            as_str(): "func4",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 652,
                                                                    end: 653,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 653,
                                                                end: 654,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 655,
                                                                            end: 659,
                                                                            as_str(): "Foo2",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 659,
                                                                                            end: 660,
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
                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                ),
                                                                                                                start: 660,
                                                                                                                end: 664,
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 664,
                                                                                            end: 665,
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 651,
                                            end: 666,
                                            as_str(): "(b: Foo2<bool>)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 667,
                                                    end: 669,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 670,
                                                                end: 674,
                                                                as_str(): "Foo2",
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 674,
                                                                                end: 675,
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
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 675,
                                                                                                    end: 677,
                                                                                                    as_str(): "u8",
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 677,
                                                                                end: 678,
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 685,
                                                            end: 687,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        MethodCall {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 688,
                                                                                end: 689,
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
                                                            dot_token: DotToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 689,
                                                                    end: 690,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            path_seg: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 690,
                                                                        end: 697,
                                                                        as_str(): "trivial",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 697,
                                                                    end: 699,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Struct {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 710,
                                                                                    end: 714,
                                                                                    as_str(): "Foo2",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 716,
                                                                                            end: 719,
                                                                                            as_str(): "foo",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    expr_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 719,
                                                                                                    end: 720,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 721,
                                                                                                            end: 722,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                        parsed: 0,
                                                                                                        ty_opt: Some(
                                                                                                            (
                                                                                                                U8,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 722,
                                                                                                                    end: 724,
                                                                                                                    as_str(): "u8",
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 715,
                                                                            end: 725,
                                                                            as_str(): "{foo: 0u8}",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 700,
                                                            end: 732,
                                                            as_str(): "{\n        Foo2 {foo: 0u8} \n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 733,
                                                                    end: 737,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Struct {
                                                                                path: PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 748,
                                                                                                end: 752,
                                                                                                as_str(): "Foo2",
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
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 754,
                                                                                                        end: 757,
                                                                                                        as_str(): "foo",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                expr_opt: Some(
                                                                                                    (
                                                                                                        ColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                ),
                                                                                                                start: 757,
                                                                                                                end: 758,
                                                                                                                as_str(): ":",
                                                                                                            },
                                                                                                        },
                                                                                                        Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 759,
                                                                                                                        end: 760,
                                                                                                                        as_str(): "1",
                                                                                                                    },
                                                                                                                    parsed: 1,
                                                                                                                    ty_opt: Some(
                                                                                                                        (
                                                                                                                            U8,
                                                                                                                            Span {
                                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 760,
                                                                                                                                end: 762,
                                                                                                                                as_str(): "u8",
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 753,
                                                                                        end: 763,
                                                                                        as_str(): "{foo: 1u8}",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 738,
                                                                        end: 769,
                                                                        as_str(): "{\n        Foo2 {foo: 1u8}\n    }",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 679,
                                        end: 771,
                                        as_str(): "{\n    if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 798,
                                            end: 801,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 802,
                                        end: 806,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 807,
                                        end: 813,
                                        as_str(): "Rezult",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 813,
                                                    end: 814,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 814,
                                                                end: 815,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 815,
                                                                end: 816,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 817,
                                                            end: 818,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 818,
                                                    end: 819,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 826,
                                                                end: 828,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 828,
                                                                end: 829,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 830,
                                                                            end: 831,
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 831,
                                                        end: 832,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 837,
                                                                end: 840,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 840,
                                                                end: 841,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 842,
                                                                            end: 843,
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
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 843,
                                                        end: 844,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 820,
                                        end: 846,
                                        as_str(): "{\n    Ok: T,\n    Err: E,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 848,
                                            end: 851,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 852,
                                        end: 856,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 857,
                                        end: 866,
                                        as_str(): "DumbError",
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 873,
                                                                end: 878,
                                                                as_str(): "Error",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 878,
                                                                end: 879,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 880,
                                                                    end: 882,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 882,
                                                        end: 883,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 867,
                                        end: 885,
                                        as_str(): "{\n    Error: (),\n}",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 887,
                                        end: 891,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 891,
                                                    end: 892,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 892,
                                                                end: 893,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 893,
                                                                end: 894,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 895,
                                                            end: 896,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 896,
                                                    end: 897,
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
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 898,
                                                    end: 904,
                                                    as_str(): "Rezult",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 904,
                                                                    end: 905,
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 905,
                                                                                            end: 906,
                                                                                            as_str(): "T",
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 906,
                                                                                end: 907,
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
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 908,
                                                                                        end: 909,
                                                                                        as_str(): "E",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 909,
                                                                    end: 910,
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 917,
                                                            end: 919,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 920,
                                                            end: 927,
                                                            as_str(): "trivial",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 928,
                                                                    end: 932,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 927,
                                                            end: 933,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 934,
                                                                    end: 936,
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 937,
                                                                                end: 941,
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
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 952,
                                                                            end: 957,
                                                                            as_str(): "false",
                                                                        },
                                                                        kind: False,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 942,
                                                        end: 963,
                                                        as_str(): "{\n        false\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 911,
                                        end: 965,
                                        as_str(): "{\n    fn trivial(self) -> bool {\n        false\n    }\n}",
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
                                    visibility: Some(
                                        PubToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 967,
                                                end: 970,
                                                as_str(): "pub",
                                            },
                                        },
                                    ),
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 971,
                                            end: 973,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 974,
                                            end: 979,
                                            as_str(): "func5",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 980,
                                                                    end: 981,
                                                                    as_str(): "r",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 981,
                                                                end: 982,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 983,
                                                                            end: 989,
                                                                            as_str(): "Rezult",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 989,
                                                                                            end: 990,
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
                                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 990,
                                                                                                                    end: 992,
                                                                                                                    as_str(): "u8",
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
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 992,
                                                                                                        end: 993,
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
                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                ),
                                                                                                                start: 994,
                                                                                                                end: 1003,
                                                                                                                as_str(): "DumbError",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1003,
                                                                                            end: 1004,
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 979,
                                            end: 1005,
                                            as_str(): "(r: Rezult<u8, DumbError>)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 1006,
                                                    end: 1008,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1009,
                                                                end: 1015,
                                                                as_str(): "Rezult",
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1015,
                                                                                end: 1016,
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
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1016,
                                                                                                        end: 1018,
                                                                                                        as_str(): "u8",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1018,
                                                                                            end: 1019,
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
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1020,
                                                                                                    end: 1029,
                                                                                                    as_str(): "DumbError",
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1029,
                                                                                end: 1030,
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1037,
                                                            end: 1039,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        MethodCall {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1040,
                                                                                end: 1041,
                                                                                as_str(): "r",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1041,
                                                                    end: 1042,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            path_seg: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1042,
                                                                        end: 1049,
                                                                        as_str(): "trivial",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1049,
                                                                    end: 1051,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    then_block: Braces {
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
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 1062,
                                                                                        end: 1068,
                                                                                        as_str(): "Rezult",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1068,
                                                                                            end: 1070,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 1070,
                                                                                                end: 1073,
                                                                                                as_str(): "Err",
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
                                                                                Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1074,
                                                                                                    end: 1083,
                                                                                                    as_str(): "DumbError",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1083,
                                                                                                        end: 1085,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1085,
                                                                                                            end: 1090,
                                                                                                            as_str(): "Error",
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1073,
                                                                            end: 1091,
                                                                            as_str(): "(DumbError::Error)",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1052,
                                                            end: 1097,
                                                            as_str(): "{\n        Rezult::Err(DumbError::Error)\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1098,
                                                                    end: 1102,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
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
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1113,
                                                                                                    end: 1119,
                                                                                                    as_str(): "Rezult",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1119,
                                                                                                        end: 1121,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1121,
                                                                                                            end: 1123,
                                                                                                            as_str(): "Ok",
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
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1124,
                                                                                                            end: 1125,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                        parsed: 1,
                                                                                                        ty_opt: Some(
                                                                                                            (
                                                                                                                U8,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1125,
                                                                                                                    end: 1127,
                                                                                                                    as_str(): "u8",
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 1123,
                                                                                        end: 1128,
                                                                                        as_str(): "(1u8)",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1103,
                                                                        end: 1134,
                                                                        as_str(): "{\n        Rezult::Ok(1u8)\n    }",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1031,
                                        end: 1136,
                                        as_str(): "{\n    if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }\n}",
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
                                    visibility: Some(
                                        PubToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 1138,
                                                end: 1141,
                                                as_str(): "pub",
                                            },
                                        },
                                    ),
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1142,
                                            end: 1144,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1145,
                                            end: 1150,
                                            as_str(): "func6",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1151,
                                                                    end: 1152,
                                                                    as_str(): "r",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1152,
                                                                end: 1153,
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1154,
                                                                            end: 1160,
                                                                            as_str(): "Rezult",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1160,
                                                                                            end: 1161,
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
                                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1161,
                                                                                                                    end: 1165,
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
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1165,
                                                                                                        end: 1166,
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
                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1167,
                                                                                                                end: 1176,
                                                                                                                as_str(): "DumbError",
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1176,
                                                                                            end: 1177,
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1150,
                                            end: 1178,
                                            as_str(): "(r: Rezult<bool, DumbError>)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 1179,
                                                    end: 1181,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1182,
                                                                end: 1188,
                                                                as_str(): "Rezult",
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1188,
                                                                                end: 1189,
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
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1189,
                                                                                                        end: 1193,
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
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1193,
                                                                                            end: 1194,
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
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1195,
                                                                                                    end: 1204,
                                                                                                    as_str(): "DumbError",
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
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1204,
                                                                                end: 1205,
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
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1211,
                                                            end: 1213,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        MethodCall {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1214,
                                                                                end: 1215,
                                                                                as_str(): "r",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1215,
                                                                    end: 1216,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            path_seg: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1216,
                                                                        end: 1223,
                                                                        as_str(): "trivial",
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
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1223,
                                                                    end: 1225,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    then_block: Braces {
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
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 1236,
                                                                                        end: 1242,
                                                                                        as_str(): "Rezult",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1242,
                                                                                            end: 1244,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 1244,
                                                                                                end: 1247,
                                                                                                as_str(): "Err",
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
                                                                                Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1248,
                                                                                                    end: 1257,
                                                                                                    as_str(): "DumbError",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1257,
                                                                                                        end: 1259,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1259,
                                                                                                            end: 1264,
                                                                                                            as_str(): "Error",
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
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1247,
                                                                            end: 1265,
                                                                            as_str(): "(DumbError::Error)",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1226,
                                                            end: 1271,
                                                            as_str(): "{\n        Rezult::Err(DumbError::Error)\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1272,
                                                                    end: 1276,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
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
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1287,
                                                                                                    end: 1293,
                                                                                                    as_str(): "Rezult",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1293,
                                                                                                        end: 1295,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1295,
                                                                                                            end: 1297,
                                                                                                            as_str(): "Ok",
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
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1298,
                                                                                                            end: 1302,
                                                                                                            as_str(): "true",
                                                                                                        },
                                                                                                        kind: True,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                        ),
                                                                                        start: 1297,
                                                                                        end: 1303,
                                                                                        as_str(): "(true)",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1277,
                                                                        end: 1309,
                                                                        as_str(): "{\n        Rezult::Ok(true)\n    }",
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
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1206,
                                        end: 1311,
                                        as_str(): "{\n   if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }\n}",
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1338,
                                            end: 1340,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1341,
                                            end: 1345,
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
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1345,
                                            end: 1347,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 1348,
                                                    end: 1350,
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
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1351,
                                                                end: 1355,
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
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1360,
                                                            end: 1364,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1356,
                                        end: 1366,
                                        as_str(): "{\n  true\n}",
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
