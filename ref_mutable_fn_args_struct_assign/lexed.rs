Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0629e5bd0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 19,
                                        as_str(): "Foo",
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
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 26,
                                                            end: 31,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 31,
                                                            end: 32,
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
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                        ),
                                                                        start: 33,
                                                                        end: 36,
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
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 20,
                                        end: 38,
                                        as_str(): "{\n    value: u64\n}",
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
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 42,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 43,
                                            end: 50,
                                            as_str(): "mut_foo",
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
                                                            reference: Some(
                                                                RefToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                        ),
                                                                        start: 51,
                                                                        end: 54,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                        ),
                                                                        start: 55,
                                                                        end: 58,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0629e5bd0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                    ),
                                                                    start: 59,
                                                                    end: 62,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                ),
                                                                start: 62,
                                                                end: 63,
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
                                                                            src (ptr): 0x00007fe0629e5bd0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                            ),
                                                                            start: 64,
                                                                            end: 67,
                                                                            as_str(): "Foo",
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
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 50,
                                            end: 68,
                                            as_str(): "(ref mut foo: Foo)",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Var(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                ),
                                                                start: 75,
                                                                end: 78,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 79,
                                                            end: 80,
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
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                        ),
                                                                        start: 81,
                                                                        end: 84,
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
                                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                                ),
                                                                                start: 87,
                                                                                end: 92,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                                        ),
                                                                                        start: 92,
                                                                                        end: 93,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                                                ),
                                                                                                start: 94,
                                                                                                end: 96,
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
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                ),
                                                                start: 85,
                                                                end: 98,
                                                                as_str(): "{ value: 10 }",
                                                            },
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 99,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 101,
                                        as_str(): "{\n    foo = Foo { value: 10 };\n}",
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
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 103,
                                            end: 105,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 106,
                                            end: 110,
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
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 110,
                                            end: 112,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0629e5bd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 115,
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
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                ),
                                                                start: 116,
                                                                end: 119,
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
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 126,
                                                            end: 129,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0629e5bd0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                    ),
                                                                    start: 130,
                                                                    end: 133,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 137,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                                ),
                                                                                start: 146,
                                                                                end: 151,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                                        ),
                                                                                        start: 151,
                                                                                        end: 152,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                                                ),
                                                                                                start: 153,
                                                                                                end: 154,
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
                                                                src (ptr): 0x00007fe0629e5bd0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                ),
                                                                start: 144,
                                                                end: 156,
                                                                as_str(): "{ value: 0 }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 156,
                                                            end: 157,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0629e5bd0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                        ),
                                                                        start: 162,
                                                                        end: 169,
                                                                        as_str(): "mut_foo",
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
                                                                                    src (ptr): 0x00007fe0629e5bd0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                                    ),
                                                                                    start: 170,
                                                                                    end: 173,
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
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 169,
                                                            end: 174,
                                                            as_str(): "(foo)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0629e5bd0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                            ),
                                                            start: 174,
                                                            end: 175,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
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
                                                                    src (ptr): 0x00007fe0629e5bd0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                                    ),
                                                                    start: 180,
                                                                    end: 183,
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
                                                        src (ptr): 0x00007fe0629e5bd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                        ),
                                                        start: 183,
                                                        end: 184,
                                                        as_str(): ".",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0629e5bd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                        ),
                                                        start: 184,
                                                        end: 189,
                                                        as_str(): "value",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 120,
                                        end: 191,
                                        as_str(): "{\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
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
