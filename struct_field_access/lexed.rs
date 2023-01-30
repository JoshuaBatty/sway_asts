Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe04b55c700,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe04b55c700,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
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
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 91,
                                            end: 93,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 94,
                                            end: 98,
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
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 100,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 101,
                                                    end: 103,
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
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 104,
                                                                end: 107,
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
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 114,
                                                            end: 117,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04b55c700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                    ),
                                                                    start: 118,
                                                                    end: 121,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 126,
                                                                as_str(): "data",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 127,
                                                            end: 128,
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
                                                                        src (ptr): 0x00007fe04b55c700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                        ),
                                                                        start: 129,
                                                                        end: 133,
                                                                        as_str(): "Data",
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
                                                                                    src (ptr): 0x00007fe04b55c700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                    ),
                                                                                    start: 144,
                                                                                    end: 157,
                                                                                    as_str(): "uselessnumber",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04b55c700,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                            ),
                                                                                            start: 157,
                                                                                            end: 158,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04b55c700,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                                    ),
                                                                                                    start: 159,
                                                                                                    end: 161,
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
                                                                                src (ptr): 0x00007fe04b55c700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                ),
                                                                                start: 161,
                                                                                end: 162,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 134,
                                                                end: 168,
                                                                as_str(): "{\n        uselessnumber: 42,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 168,
                                                            end: 169,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: FieldProjection {
                                                        target: Var(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04b55c700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                    ),
                                                                    start: 174,
                                                                    end: 178,
                                                                    as_str(): "data",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 178,
                                                                end: 179,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 192,
                                                                as_str(): "uselessnumber",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 193,
                                                            end: 194,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04b55c700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                    ),
                                                                    start: 195,
                                                                    end: 197,
                                                                    as_str(): "43",
                                                                },
                                                                parsed: 43,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 197,
                                                            end: 198,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 204,
                                                            end: 207,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 208,
                                                                end: 213,
                                                                as_str(): "other",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 214,
                                                            end: 215,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: FieldProjection {
                                                        target: FuncApp {
                                                            func: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04b55c700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                ),
                                                                                start: 216,
                                                                                end: 226,
                                                                                as_str(): "ret_struct",
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
                                                                    src (ptr): 0x00007fe04b55c700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                    ),
                                                                    start: 226,
                                                                    end: 228,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        },
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 228,
                                                                end: 229,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 229,
                                                                end: 242,
                                                                as_str(): "uselessnumber",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 242,
                                                            end: 243,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 249,
                                                            end: 255,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        FieldProjection {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04b55c700,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                ),
                                                                                start: 256,
                                                                                end: 260,
                                                                                as_str(): "data",
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
                                                                    src (ptr): 0x00007fe04b55c700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                    ),
                                                                    start: 260,
                                                                    end: 261,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04b55c700,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                    ),
                                                                    start: 261,
                                                                    end: 274,
                                                                    as_str(): "uselessnumber",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04b55c700,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                            ),
                                                            start: 274,
                                                            end: 275,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 108,
                                        end: 277,
                                        as_str(): "{\n    let mut data = Data {\n        uselessnumber: 42,\n    };\n    data.uselessnumber = 43;\n\n    let other = ret_struct().uselessnumber;\n\n    return data.uselessnumber;\n}",
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
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 279,
                                        end: 285,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 286,
                                        end: 290,
                                        as_str(): "Data",
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
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 297,
                                                                end: 310,
                                                                as_str(): "uselessnumber",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 310,
                                                                end: 311,
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
                                                                            src (ptr): 0x00007fe04b55c700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
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
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b55c700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                        ),
                                                        start: 315,
                                                        end: 316,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 291,
                                        end: 318,
                                        as_str(): "{\n    uselessnumber: u64,\n}",
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
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 320,
                                            end: 322,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 323,
                                            end: 333,
                                            as_str(): "ret_struct",
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
                                            src (ptr): 0x00007fe04b55c700,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                            ),
                                            start: 333,
                                            end: 335,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe04b55c700,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                    ),
                                                    start: 336,
                                                    end: 338,
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
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 339,
                                                                end: 343,
                                                                as_str(): "Data",
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
                                            Struct {
                                                path: PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04b55c700,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                ),
                                                                start: 350,
                                                                end: 354,
                                                                as_str(): "Data",
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
                                                                            src (ptr): 0x00007fe04b55c700,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                            ),
                                                                            start: 365,
                                                                            end: 378,
                                                                            as_str(): "uselessnumber",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    expr_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04b55c700,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                    ),
                                                                                    start: 378,
                                                                                    end: 379,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04b55c700,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                                            ),
                                                                                            start: 380,
                                                                                            end: 382,
                                                                                            as_str(): "44",
                                                                                        },
                                                                                        parsed: 44,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04b55c700,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                                        ),
                                                                        start: 382,
                                                                        end: 383,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe04b55c700,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                                        ),
                                                        start: 355,
                                                        end: 389,
                                                        as_str(): "{\n        uselessnumber: 44,\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04b55c700,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWnki15/struct_field_access/src/main.sw",
                                        ),
                                        start: 344,
                                        end: 391,
                                        as_str(): "{\n    Data {\n        uselessnumber: 44,\n    }\n}",
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
