Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb14ccd5c90,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Group {
                                        imports: Braces {
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 19,
                                                                    end: 26,
                                                                    as_str(): "address",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 26,
                                                                    end: 28,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 28,
                                                                        end: 35,
                                                                        as_str(): "Address",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 36,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 37,
                                                                    end: 45,
                                                                    as_str(): "identity",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 45,
                                                                    end: 47,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 47,
                                                                        end: 55,
                                                                        as_str(): "Identity",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 55,
                                                                end: 56,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 57,
                                                                    end: 63,
                                                                    as_str(): "assert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 63,
                                                                    end: 65,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 65,
                                                                        end: 71,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 71,
                                                                end: 72,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    Path {
                                                        prefix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 73,
                                                                end: 79,
                                                                as_str(): "revert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 79,
                                                                end: 81,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 87,
                                                                    as_str(): "revert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                ),
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 88,
                                                as_str(): "{address::Address, identity::Identity, assert::assert, revert::revert}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 88,
                                        end: 89,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 91,
                                        end: 96,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 97,
                                        end: 99,
                                        as_str(): "B1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 100,
                                        end: 101,
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
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 102,
                                                    end: 109,
                                                    as_str(): "Address",
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
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 116,
                                                            end: 121,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    expr_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 121,
                                                                    end: 122,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 123,
                                                                            end: 189,
                                                                            as_str(): "0x0100000000000000000000000000000000000000000000000000000000000010",
                                                                        },
                                                                        parsed: 452312848583266388373324160190187140051835877600158453279131187530910662672,
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
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 110,
                                            end: 191,
                                            as_str(): "{\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n}",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 191,
                                        end: 192,
                                        as_str(): ";",
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 194,
                                                end: 197,
                                                as_str(): "pub",
                                            },
                                        },
                                    ),
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 198,
                                            end: 200,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 201,
                                            end: 208,
                                            as_str(): "addr_of",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 208,
                                                        end: 209,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 209,
                                                                end: 210,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 210,
                                                        end: 211,
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
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 212,
                                                                    end: 215,
                                                                    as_str(): "val",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 215,
                                                                end: 216,
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 217,
                                                                            end: 218,
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
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 211,
                                            end: 219,
                                            as_str(): "(val: T)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 220,
                                                    end: 222,
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
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 223,
                                                                end: 230,
                                                                as_str(): "raw_ptr",
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
                                            Expr {
                                                expr: If(
                                                    IfExpr {
                                                        if_token: IfToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 237,
                                                                end: 239,
                                                                as_str(): "if",
                                                            },
                                                        },
                                                        condition: Expr(
                                                            Not {
                                                                bang_token: BangToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 240,
                                                                        end: 241,
                                                                        as_str(): "!",
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
                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 241,
                                                                                        end: 260,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: Some(
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 260,
                                                                                                end: 262,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        GenericArgs {
                                                                                            parameters: AngleBrackets {
                                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 262,
                                                                                                        end: 263,
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
                                                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 263,
                                                                                                                            end: 264,
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 264,
                                                                                                        end: 265,
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
                                                                    ),
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 265,
                                                                            end: 267,
                                                                            as_str(): "()",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        then_block: Braces {
                                                            inner: CodeBlockContents {
                                                                statements: [
                                                                    Expr {
                                                                        expr: FuncApp {
                                                                            func: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 278,
                                                                                                end: 284,
                                                                                                as_str(): "revert",
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
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 285,
                                                                                                        end: 286,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 284,
                                                                                    end: 287,
                                                                                    as_str(): "(0)",
                                                                                },
                                                                            },
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 287,
                                                                                    end: 288,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 268,
                                                                end: 294,
                                                                as_str(): "{\n        revert(0);\n    }",
                                                            },
                                                        },
                                                        else_opt: None,
                                                    },
                                                ),
                                                semicolon_token_opt: None,
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Asm(
                                                AsmBlock {
                                                    asm_token: AsmToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 299,
                                                            end: 302,
                                                            as_str(): "asm",
                                                        },
                                                    },
                                                    registers: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
                                                                AsmRegisterDeclaration {
                                                                    register: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 303,
                                                                            end: 306,
                                                                            as_str(): "ptr",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    value_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 306,
                                                                                    end: 307,
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
                                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 308,
                                                                                                end: 311,
                                                                                                as_str(): "val",
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 302,
                                                            end: 312,
                                                            as_str(): "(ptr: val)",
                                                        },
                                                    },
                                                    contents: Braces {
                                                        inner: AsmBlockContents {
                                                            instructions: [],
                                                            final_expr_opt: Some(
                                                                AsmFinalExpr {
                                                                    register: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 323,
                                                                            end: 326,
                                                                            as_str(): "ptr",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    ty_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 326,
                                                                                    end: 327,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Path(
                                                                                PathType {
                                                                                    root_opt: None,
                                                                                    prefix: PathTypeSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                ),
                                                                                                start: 328,
                                                                                                end: 335,
                                                                                                as_str(): "raw_ptr",
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
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 313,
                                                            end: 341,
                                                            as_str(): "{\n        ptr: raw_ptr\n    }",
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 231,
                                        end: 343,
                                        as_str(): "{\n    if !__is_reference_type::<T>() {\n        revert(0);\n    }\n    asm(ptr: val) {\n        ptr: raw_ptr\n    }\n}",
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 345,
                                        end: 349,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 350,
                                        end: 351,
                                        as_str(): "X",
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
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 359,
                                                                end: 360,
                                                                as_str(): "A",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 360,
                                                                end: 361,
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 362,
                                                                            end: 365,
                                                                            as_str(): "u32",
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 365,
                                                        end: 366,
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
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 372,
                                                                end: 373,
                                                                as_str(): "B",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 373,
                                                                end: 374,
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 375,
                                                                            end: 378,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 378,
                                                        end: 379,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 352,
                                        end: 381,
                                        as_str(): "{\n     A: u32,\n     B: u64,\n}",
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
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 383,
                                            end: 385,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 386,
                                            end: 390,
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
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 390,
                                            end: 392,
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
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 399,
                                                            end: 402,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 403,
                                                                end: 409,
                                                                as_str(): "sender",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 410,
                                                            end: 411,
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 412,
                                                                            end: 420,
                                                                            as_str(): "Identity",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 420,
                                                                                end: 422,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 422,
                                                                                    end: 429,
                                                                                    as_str(): "Address",
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
                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 430,
                                                                                        end: 432,
                                                                                        as_str(): "B1",
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
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 429,
                                                                end: 433,
                                                                as_str(): "(B1)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 433,
                                                            end: 434,
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 439,
                                                                        end: 445,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 447,
                                                                                            end: 456,
                                                                                            as_str(): "__addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 457,
                                                                                                        end: 463,
                                                                                                        as_str(): "sender",
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
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 456,
                                                                                end: 464,
                                                                                as_str(): "(sender)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 465,
                                                                            end: 467,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 468,
                                                                                            end: 475,
                                                                                            as_str(): "addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 476,
                                                                                                        end: 482,
                                                                                                        as_str(): "sender",
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
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 475,
                                                                                end: 483,
                                                                                as_str(): "(sender)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 446,
                                                            end: 484,
                                                            as_str(): "(__addr_of(sender) == addr_of(sender))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 484,
                                                            end: 485,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 491,
                                                            end: 494,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 495,
                                                                end: 496,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 497,
                                                            end: 498,
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 499,
                                                                            end: 500,
                                                                            as_str(): "X",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 500,
                                                                                end: 502,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 502,
                                                                                    end: 503,
                                                                                    as_str(): "A",
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
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 504,
                                                                                    end: 505,
                                                                                    as_str(): "2",
                                                                                },
                                                                                parsed: 2,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 503,
                                                                end: 506,
                                                                as_str(): "(2)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 506,
                                                            end: 507,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 512,
                                                            end: 515,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 516,
                                                                end: 517,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 518,
                                                            end: 519,
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 520,
                                                                            end: 521,
                                                                            as_str(): "X",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 521,
                                                                                end: 523,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 523,
                                                                                    end: 524,
                                                                                    as_str(): "B",
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
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 525,
                                                                                    end: 527,
                                                                                    as_str(): "22",
                                                                                },
                                                                                parsed: 22,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 524,
                                                                end: 528,
                                                                as_str(): "(22)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 528,
                                                            end: 529,
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 534,
                                                                        end: 540,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 541,
                                                                                            end: 550,
                                                                                            as_str(): "__addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 551,
                                                                                                        end: 552,
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
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 550,
                                                                                end: 553,
                                                                                as_str(): "(x)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 554,
                                                                            end: 556,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 557,
                                                                                            end: 564,
                                                                                            as_str(): "addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 565,
                                                                                                        end: 566,
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
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 564,
                                                                                end: 567,
                                                                                as_str(): "(x)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 540,
                                                            end: 568,
                                                            as_str(): "(__addr_of(x) == addr_of(x))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 568,
                                                            end: 569,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 574,
                                                                        end: 580,
                                                                        as_str(): "assert",
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
                                                                NotEqual {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 581,
                                                                                            end: 590,
                                                                                            as_str(): "__addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 591,
                                                                                                        end: 592,
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
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 590,
                                                                                end: 593,
                                                                                as_str(): "(x)",
                                                                            },
                                                                        },
                                                                    },
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 594,
                                                                            end: 596,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 597,
                                                                                            end: 604,
                                                                                            as_str(): "addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 605,
                                                                                                        end: 606,
                                                                                                        as_str(): "y",
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
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 604,
                                                                                end: 607,
                                                                                as_str(): "(y)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 580,
                                                            end: 608,
                                                            as_str(): "(__addr_of(x) != addr_of(y))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 608,
                                                            end: 609,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 615,
                                                            end: 618,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 619,
                                                                end: 620,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 621,
                                                            end: 622,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [
                                                                        (
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 624,
                                                                                            end: 625,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 625,
                                                                                    end: 626,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 626,
                                                                                            end: 627,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                        parsed: 2,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 627,
                                                                                    end: 628,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    final_value_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 628,
                                                                                        end: 629,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 623,
                                                                end: 630,
                                                                as_str(): "[1,2,3]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 630,
                                                            end: 631,
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 636,
                                                                        end: 642,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 643,
                                                                                            end: 652,
                                                                                            as_str(): "__addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 653,
                                                                                                        end: 654,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 652,
                                                                                end: 655,
                                                                                as_str(): "(a)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 656,
                                                                            end: 658,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 659,
                                                                                            end: 666,
                                                                                            as_str(): "addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 667,
                                                                                                        end: 668,
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 666,
                                                                                end: 669,
                                                                                as_str(): "(a)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 642,
                                                            end: 670,
                                                            as_str(): "(__addr_of(a) == addr_of(a))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 670,
                                                            end: 671,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 677,
                                                            end: 680,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 681,
                                                                end: 682,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 683,
                                                            end: 684,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        String(
                                                            LitString {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 685,
                                                                    end: 692,
                                                                    as_str(): "\"hello\"",
                                                                },
                                                                parsed: "hello",
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 692,
                                                            end: 693,
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 698,
                                                                        end: 704,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 705,
                                                                                            end: 714,
                                                                                            as_str(): "__addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 715,
                                                                                                        end: 716,
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
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 714,
                                                                                end: 717,
                                                                                as_str(): "(b)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 718,
                                                                            end: 720,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 721,
                                                                                            end: 728,
                                                                                            as_str(): "addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 729,
                                                                                                        end: 730,
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
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 728,
                                                                                end: 731,
                                                                                as_str(): "(b)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 704,
                                                            end: 732,
                                                            as_str(): "(__addr_of(b) == addr_of(b))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 732,
                                                            end: 733,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 739,
                                                            end: 742,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 743,
                                                                end: 744,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 745,
                                                            end: 746,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Tuple(
                                                        Parens {
                                                            inner: Cons {
                                                                head: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 748,
                                                                                end: 749,
                                                                                as_str(): "1",
                                                                            },
                                                                            parsed: 1,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 749,
                                                                        end: 750,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                tail: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                        ),
                                                                                        start: 751,
                                                                                        end: 752,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                ),
                                                                start: 747,
                                                                end: 753,
                                                                as_str(): "(1, 2)",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 753,
                                                            end: 754,
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 759,
                                                                        end: 765,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 766,
                                                                                            end: 775,
                                                                                            as_str(): "__addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 776,
                                                                                                        end: 777,
                                                                                                        as_str(): "c",
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
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 775,
                                                                                end: 778,
                                                                                as_str(): "(c)",
                                                                            },
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 779,
                                                                            end: 781,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                            ),
                                                                                            start: 782,
                                                                                            end: 789,
                                                                                            as_str(): "addr_of",
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
                                                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 790,
                                                                                                        end: 791,
                                                                                                        as_str(): "c",
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
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 789,
                                                                                end: 792,
                                                                                as_str(): "(c)",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 765,
                                                            end: 793,
                                                            as_str(): "(__addr_of(c) == addr_of(c))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 793,
                                                            end: 794,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 393,
                                        end: 796,
                                        as_str(): "{\n    let sender = Identity::Address(B1);\n    assert (__addr_of(sender) == addr_of(sender));\n\n    let x = X::A(2);\n    let y = X::B(22);\n    assert(__addr_of(x) == addr_of(x));\n    assert(__addr_of(x) != addr_of(y));\n\n    let a = [1,2,3];\n    assert(__addr_of(a) == addr_of(a));\n\n    let b = \"hello\";\n    assert(__addr_of(b) == addr_of(b));\n\n    let c = (1, 2);\n    assert(__addr_of(c) == addr_of(c));\n}",
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
