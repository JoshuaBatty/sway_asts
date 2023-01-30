Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe08f9c9f10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe08f9c9f10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
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
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 16,
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
                                            src (ptr): 0x00007fe08f9c9f10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe08f9c9f10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                    ),
                                                    start: 19,
                                                    end: 21,
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
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 22,
                                                                end: 25,
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
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 32,
                                                            end: 35,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 36,
                                                                    end: 39,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 40,
                                                                end: 50,
                                                                as_str(): "my_array_0",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 50,
                                                                    end: 51,
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
                                                                                            src (ptr): 0x00007fe08f9c9f10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
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
                                                                        semicolon_token: SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                ),
                                                                                start: 56,
                                                                                end: 57,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                        ),
                                                                                        start: 58,
                                                                                        end: 59,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                        ),
                                                                        start: 52,
                                                                        end: 60,
                                                                        as_str(): "[u64; 1]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 61,
                                                            end: 62,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                        ),
                                                                                        start: 64,
                                                                                        end: 65,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 66,
                                                                as_str(): "[1]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 66,
                                                            end: 67,
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
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 72,
                                                                    end: 82,
                                                                    as_str(): "my_array_0",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        arg: SquareBrackets {
                                                            inner: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f9c9f10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                            ),
                                                                            start: 83,
                                                                            end: 84,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 82,
                                                                end: 85,
                                                                as_str(): "[0]",
                                                            },
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 86,
                                                            end: 87,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 88,
                                                                    end: 90,
                                                                    as_str(): "10",
                                                                },
                                                                parsed: 10,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 91,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 97,
                                                            end: 100,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 101,
                                                                    end: 104,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 115,
                                                                as_str(): "my_array_1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 115,
                                                                    end: 116,
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
                                                                                            src (ptr): 0x00007fe08f9c9f10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                            ),
                                                                                            start: 118,
                                                                                            end: 121,
                                                                                            as_str(): "u64",
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
                                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                ),
                                                                                start: 121,
                                                                                end: 122,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                        ),
                                                                                        start: 123,
                                                                                        end: 124,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                        ),
                                                                        start: 117,
                                                                        end: 125,
                                                                        as_str(): "[u64; 1]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 126,
                                                            end: 127,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08f9c9f10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                                        ),
                                                                                        start: 129,
                                                                                        end: 130,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 128,
                                                                end: 131,
                                                                as_str(): "[1]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 131,
                                                            end: 132,
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
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 137,
                                                                    end: 147,
                                                                    as_str(): "my_array_1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        arg: SquareBrackets {
                                                            inner: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f9c9f10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                            ),
                                                                            start: 148,
                                                                            end: 149,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 147,
                                                                end: 150,
                                                                as_str(): "[0]",
                                                            },
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 151,
                                                            end: 152,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 153,
                                                                    end: 155,
                                                                    as_str(): "20",
                                                                },
                                                                parsed: 20,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 155,
                                                            end: 156,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Index {
                                                        target: Var(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 162,
                                                                    end: 172,
                                                                    as_str(): "my_array_0",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        arg: SquareBrackets {
                                                            inner: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f9c9f10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                            ),
                                                                            start: 173,
                                                                            end: 174,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 172,
                                                                end: 175,
                                                                as_str(): "[0]",
                                                            },
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 176,
                                                            end: 177,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Index {
                                                        target: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f9c9f10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                            ),
                                                                            start: 178,
                                                                            end: 188,
                                                                            as_str(): "my_array_1",
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
                                                                            src (ptr): 0x00007fe08f9c9f10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                            ),
                                                                            start: 189,
                                                                            end: 190,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe08f9c9f10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                ),
                                                                start: 188,
                                                                end: 191,
                                                                as_str(): "[0]",
                                                            },
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f9c9f10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                            ),
                                                            start: 191,
                                                            end: 192,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Index {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 197,
                                                                    end: 207,
                                                                    as_str(): "my_array_0",
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
                                                                    src (ptr): 0x00007fe08f9c9f10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                                    ),
                                                                    start: 208,
                                                                    end: 209,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f9c9f10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                                        ),
                                                        start: 207,
                                                        end: 210,
                                                        as_str(): "[0]",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe08f9c9f10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR32qRJl/mutable_arrays_swap/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 212,
                                        as_str(): "{\n    let mut my_array_0: [u64; 1] = [1];\n    my_array_0[0] = 10;\n\n    let mut my_array_1: [u64; 1] = [1];\n    my_array_1[0] = 20;\n\n    my_array_0[0] = my_array_1[0];\n    my_array_0[0]\n}",
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
