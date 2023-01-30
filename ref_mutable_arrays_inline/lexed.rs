Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe05be7da40,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe05be7da40,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
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
                                            src (ptr): 0x00007fe05be7da40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05be7da40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
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
                                            src (ptr): 0x00007fe05be7da40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
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
                                                    src (ptr): 0x00007fe05be7da40,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
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
                                                                src (ptr): 0x00007fe05be7da40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
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
                                                            src (ptr): 0x00007fe05be7da40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
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
                                                                    src (ptr): 0x00007fe05be7da40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
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
                                                                src (ptr): 0x00007fe05be7da40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                ),
                                                                start: 40,
                                                                end: 43,
                                                                as_str(): "arr",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05be7da40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                    ),
                                                                    start: 43,
                                                                    end: 44,
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
                                                                                            src (ptr): 0x00007fe05be7da40,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                                            ),
                                                                                            start: 46,
                                                                                            end: 49,
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
                                                                                src (ptr): 0x00007fe05be7da40,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                                ),
                                                                                start: 49,
                                                                                end: 50,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05be7da40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                                        ),
                                                                                        start: 51,
                                                                                        end: 52,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                    parsed: 1,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05be7da40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                        ),
                                                                        start: 45,
                                                                        end: 53,
                                                                        as_str(): "[u64; 1]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05be7da40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                            ),
                                                            start: 54,
                                                            end: 55,
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
                                                                                        src (ptr): 0x00007fe05be7da40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                                        ),
                                                                                        start: 57,
                                                                                        end: 58,
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
                                                                src (ptr): 0x00007fe05be7da40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                ),
                                                                start: 56,
                                                                end: 59,
                                                                as_str(): "[1]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05be7da40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 60,
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
                                                                        src (ptr): 0x00007fe05be7da40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                        ),
                                                                        start: 65,
                                                                        end: 84,
                                                                        as_str(): "takes_ref_mut_array",
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
                                                                                    src (ptr): 0x00007fe05be7da40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                                    ),
                                                                                    start: 85,
                                                                                    end: 88,
                                                                                    as_str(): "arr",
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
                                                            src (ptr): 0x00007fe05be7da40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                            ),
                                                            start: 84,
                                                            end: 89,
                                                            as_str(): "(arr)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05be7da40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                            ),
                                                            start: 89,
                                                            end: 90,
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
                                                                    src (ptr): 0x00007fe05be7da40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 98,
                                                                    as_str(): "arr",
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
                                                                    src (ptr): 0x00007fe05be7da40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                    ),
                                                                    start: 99,
                                                                    end: 100,
                                                                    as_str(): "0",
                                                                },
                                                                parsed: 0,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05be7da40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                        ),
                                                        start: 98,
                                                        end: 101,
                                                        as_str(): "[0]",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05be7da40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                        ),
                                        start: 26,
                                        end: 103,
                                        as_str(): "{\n    let mut arr: [u64; 1] = [1];\n    takes_ref_mut_array(arr);\n    arr[0]\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [
                            AttributeDecl {
                                hash_token: HashToken {
                                    span: Span {
                                        src (ptr): 0x00007fe05be7da40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                        ),
                                        start: 105,
                                        end: 106,
                                        as_str(): "#",
                                    },
                                },
                                attribute: SquareBrackets {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Attribute {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe05be7da40,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                        ),
                                                        start: 107,
                                                        end: 113,
                                                        as_str(): "inline",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                args: Some(
                                                    Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05be7da40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                        ),
                                                                        start: 114,
                                                                        end: 120,
                                                                        as_str(): "always",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05be7da40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                            ),
                                                            start: 113,
                                                            end: 121,
                                                            as_str(): "(always)",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05be7da40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                        ),
                                        start: 106,
                                        end: 122,
                                        as_str(): "[inline(always)]",
                                    },
                                },
                            },
                        ],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe05be7da40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                            ),
                                            start: 123,
                                            end: 125,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05be7da40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                            ),
                                            start: 126,
                                            end: 145,
                                            as_str(): "takes_ref_mut_array",
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
                                                                        src (ptr): 0x00007fe05be7da40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                        ),
                                                                        start: 146,
                                                                        end: 149,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05be7da40,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                        ),
                                                                        start: 150,
                                                                        end: 153,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05be7da40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                    ),
                                                                    start: 154,
                                                                    end: 157,
                                                                    as_str(): "arr",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe05be7da40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 158,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Array(
                                                            SquareBrackets {
                                                                inner: TyArrayDescriptor {
                                                                    ty: Path(
                                                                        PathType {
                                                                            root_opt: None,
                                                                            prefix: PathTypeSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05be7da40,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                                        ),
                                                                                        start: 160,
                                                                                        end: 163,
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
                                                                            src (ptr): 0x00007fe05be7da40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                            ),
                                                                            start: 163,
                                                                            end: 164,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                    length: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05be7da40,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                                    ),
                                                                                    start: 165,
                                                                                    end: 166,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05be7da40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                    ),
                                                                    start: 159,
                                                                    end: 167,
                                                                    as_str(): "[u64; 1]",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05be7da40,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                            ),
                                            start: 145,
                                            end: 168,
                                            as_str(): "(ref mut arr: [u64; 1])",
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
                                                    assignable: Index {
                                                        target: Var(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05be7da40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                    ),
                                                                    start: 175,
                                                                    end: 178,
                                                                    as_str(): "arr",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        arg: SquareBrackets {
                                                            inner: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05be7da40,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                            ),
                                                                            start: 179,
                                                                            end: 180,
                                                                            as_str(): "0",
                                                                        },
                                                                        parsed: 0,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe05be7da40,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                ),
                                                                start: 178,
                                                                end: 181,
                                                                as_str(): "[0]",
                                                            },
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05be7da40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 183,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05be7da40,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                                    ),
                                                                    start: 184,
                                                                    end: 186,
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
                                                            src (ptr): 0x00007fe05be7da40,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                                            ),
                                                            start: 186,
                                                            end: 187,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05be7da40,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRMZ0cXm/ref_mutable_arrays_inline/src/main.sw",
                                        ),
                                        start: 169,
                                        end: 189,
                                        as_str(): "{\n    arr[0] = 10;\n}",
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
