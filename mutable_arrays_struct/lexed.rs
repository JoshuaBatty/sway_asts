Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe096cd2c70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe096cd2c70,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
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
                                        src (ptr): 0x00007fe096cd2c70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe096cd2c70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
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
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                            ),
                                                            start: 26,
                                                            end: 31,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
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
                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
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
                                        src (ptr): 0x00007fe096cd2c70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
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
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 42,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 43,
                                            end: 47,
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
                                            src (ptr): 0x00007fe096cd2c70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                            ),
                                            start: 47,
                                            end: 49,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe096cd2c70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                    ),
                                                    start: 50,
                                                    end: 52,
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
                                                                src (ptr): 0x00007fe096cd2c70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                ),
                                                                start: 53,
                                                                end: 56,
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
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                            ),
                                                            start: 63,
                                                            end: 66,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                    ),
                                                                    start: 67,
                                                                    end: 70,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe096cd2c70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                ),
                                                                start: 71,
                                                                end: 79,
                                                                as_str(): "my_array",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                    ),
                                                                    start: 79,
                                                                    end: 80,
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
                                                                                            src (ptr): 0x00007fe096cd2c70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 82,
                                                                                            end: 85,
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
                                                                                src (ptr): 0x00007fe096cd2c70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                ),
                                                                                start: 85,
                                                                                end: 86,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 87,
                                                                                        end: 88,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                        ),
                                                                        start: 81,
                                                                        end: 89,
                                                                        as_str(): "[Foo; 1]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 91,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Struct {
                                                                            path: PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe096cd2c70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 93,
                                                                                            end: 96,
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
                                                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 97,
                                                                                                    end: 102,
                                                                                                    as_str(): "value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                (
                                                                                                    ColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe096cd2c70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                                            ),
                                                                                                            start: 102,
                                                                                                            end: 103,
                                                                                                            as_str(): ":",
                                                                                                        },
                                                                                                    },
                                                                                                    Literal(
                                                                                                        Int(
                                                                                                            LitInt {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 104,
                                                                                                                    end: 106,
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
                                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 96,
                                                                                    end: 107,
                                                                                    as_str(): "{value: 10}",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe096cd2c70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                ),
                                                                start: 92,
                                                                end: 108,
                                                                as_str(): "[Foo{value: 10}]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 109,
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
                                                                    src (ptr): 0x00007fe096cd2c70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                    ),
                                                                    start: 114,
                                                                    end: 122,
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
                                                                            src (ptr): 0x00007fe096cd2c70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                            ),
                                                                            start: 123,
                                                                            end: 124,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe096cd2c70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 125,
                                                                as_str(): "[0]",
                                                            },
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                            ),
                                                            start: 126,
                                                            end: 127,
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
                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                        ),
                                                                        start: 128,
                                                                        end: 131,
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
                                                                                src (ptr): 0x00007fe096cd2c70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                ),
                                                                                start: 132,
                                                                                end: 137,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 137,
                                                                                        end: 138,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe096cd2c70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 139,
                                                                                                end: 141,
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
                                                                src (ptr): 0x00007fe096cd2c70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                ),
                                                                start: 131,
                                                                end: 142,
                                                                as_str(): "{value: 20}",
                                                            },
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                            ),
                                                            start: 142,
                                                            end: 143,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            FieldProjection {
                                                target: Index {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                        ),
                                                                        start: 148,
                                                                        end: 156,
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
                                                                        src (ptr): 0x00007fe096cd2c70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                                        ),
                                                                        start: 157,
                                                                        end: 158,
                                                                        as_str(): "0",
                                                                    },
                                                                    parsed: 0,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe096cd2c70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                            ),
                                                            start: 156,
                                                            end: 159,
                                                            as_str(): "[0]",
                                                        },
                                                    },
                                                },
                                                dot_token: DotToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 159,
                                                        end: 160,
                                                        as_str(): ".",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe096cd2c70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                                        ),
                                                        start: 160,
                                                        end: 165,
                                                        as_str(): "value",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe096cd2c70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRDOXMcC/mutable_arrays_struct/src/main.sw",
                                        ),
                                        start: 57,
                                        end: 167,
                                        as_str(): "{\n    let mut my_array: [Foo; 1] = [Foo{value: 10}];\n    my_array[0] = Foo{value: 20};\n    my_array[0].value\n}",
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
