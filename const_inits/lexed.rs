Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 14,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 15,
                                        end: 22,
                                        as_str(): "ETH_ID0",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 23,
                                        end: 24,
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
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 25,
                                                        end: 35,
                                                        as_str(): "ContractId",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [
                                                (
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 35,
                                                            end: 37,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 37,
                                                                end: 41,
                                                                as_str(): "from",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 42,
                                                                end: 108,
                                                                as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                            },
                                                            parsed: 0,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 41,
                                            end: 109,
                                            as_str(): "(0x0000000000000000000000000000000000000000000000000000000000000000)",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 109,
                                        end: 110,
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
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 111,
                                            end: 113,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 114,
                                            end: 133,
                                            as_str(): "contract_id_wrapper",
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
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 134,
                                                                    end: 135,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 135,
                                                                end: 136,
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
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 137,
                                                                            end: 141,
                                                                            as_str(): "b256",
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
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 133,
                                            end: 142,
                                            as_str(): "(b: b256)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 145,
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 146,
                                                                end: 156,
                                                                as_str(): "ContractId",
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
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 163,
                                                                    end: 173,
                                                                    as_str(): "ContractId",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [
                                                            (
                                                                DoubleColonToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 173,
                                                                        end: 175,
                                                                        as_str(): "::",
                                                                    },
                                                                },
                                                                PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 175,
                                                                            end: 179,
                                                                            as_str(): "from",
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
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 180,
                                                                                end: 181,
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
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 179,
                                                        end: 182,
                                                        as_str(): "(b)",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 157,
                                        end: 184,
                                        as_str(): "{\n    ContractId::from(b)\n}",
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 185,
                                        end: 190,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 191,
                                        end: 198,
                                        as_str(): "ETH_ID1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 199,
                                        end: 200,
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
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 201,
                                                        end: 220,
                                                        as_str(): "contract_id_wrapper",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 221,
                                                                end: 287,
                                                                as_str(): "0x0000000000000000000000000000000000000000000000000000000000000001",
                                                            },
                                                            parsed: 1,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 220,
                                            end: 288,
                                            as_str(): "(0x0000000000000000000000000000000000000000000000000000000000000001)",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 288,
                                        end: 289,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 291,
                                        end: 296,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 297,
                                        end: 301,
                                        as_str(): "TUP1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 302,
                                        end: 303,
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
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 305,
                                                            end: 306,
                                                            as_str(): "2",
                                                        },
                                                        parsed: 2,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                            comma_token: CommaToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 306,
                                                    end: 307,
                                                    as_str(): ",",
                                                },
                                            },
                                            tail: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 308,
                                                                        end: 309,
                                                                        as_str(): "1",
                                                                    },
                                                                    parsed: 1,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 309,
                                                                end: 310,
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
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 311,
                                                                    end: 313,
                                                                    as_str(): "21",
                                                                },
                                                                parsed: 21,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 304,
                                            end: 314,
                                            as_str(): "(2, 1, 21)",
                                        },
                                    },
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 314,
                                        end: 315,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 316,
                                        end: 321,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 322,
                                        end: 326,
                                        as_str(): "ARR1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 327,
                                        end: 328,
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 330,
                                                                        end: 331,
                                                                        as_str(): "1",
                                                                    },
                                                                    parsed: 1,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 331,
                                                                end: 332,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 333,
                                                                        end: 334,
                                                                        as_str(): "2",
                                                                    },
                                                                    parsed: 2,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 334,
                                                                end: 335,
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
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 336,
                                                                    end: 337,
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
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 329,
                                            end: 338,
                                            as_str(): "[1, 2, 3]",
                                        },
                                    },
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 338,
                                        end: 339,
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
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 341,
                                            end: 343,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 344,
                                            end: 355,
                                            as_str(): "tup_wrapper",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 356,
                                                                        end: 357,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 357,
                                                                    end: 358,
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
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 359,
                                                                                end: 362,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 362,
                                                                end: 363,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 364,
                                                                        end: 365,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 365,
                                                                    end: 366,
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
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 367,
                                                                                end: 370,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 370,
                                                                end: 371,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 372,
                                                                    end: 373,
                                                                    as_str(): "c",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
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
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 355,
                                            end: 379,
                                            as_str(): "(a: u64, b: u64, c: u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 380,
                                                    end: 382,
                                                    as_str(): "->",
                                                },
                                            },
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 384,
                                                                            end: 387,
                                                                            as_str(): "u64",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 387,
                                                                end: 388,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [
                                                                (
                                                                    Path(
                                                                        PathType {
                                                                            root_opt: None,
                                                                            prefix: PathTypeSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 389,
                                                                                        end: 392,
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
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 392,
                                                                            end: 393,
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 394,
                                                                                    end: 397,
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
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 383,
                                                        end: 398,
                                                        as_str(): "(u64, u64, u64)",
                                                    },
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
                                            Tuple(
                                                Parens {
                                                    inner: Cons {
                                                        head: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 406,
                                                                            end: 407,
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
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 407,
                                                                end: 408,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        tail: Punctuated {
                                                            value_separator_pairs: [
                                                                (
                                                                    Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 409,
                                                                                        end: 410,
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
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 410,
                                                                            end: 411,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 412,
                                                                                    end: 413,
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
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 405,
                                                        end: 414,
                                                        as_str(): "(a, b, c)",
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 399,
                                        end: 416,
                                        as_str(): "{\n    (a, b, c)\n}",
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 417,
                                        end: 422,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 423,
                                        end: 427,
                                        as_str(): "TUP2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 428,
                                        end: 429,
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
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 430,
                                                        end: 441,
                                                        as_str(): "tup_wrapper",
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
                                            value_separator_pairs: [
                                                (
                                                    Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 442,
                                                                    end: 443,
                                                                    as_str(): "2",
                                                                },
                                                                parsed: 2,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    CommaToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 443,
                                                            end: 444,
                                                            as_str(): ",",
                                                        },
                                                    },
                                                ),
                                                (
                                                    Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 445,
                                                                    end: 446,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    CommaToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 446,
                                                            end: 447,
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 448,
                                                                end: 450,
                                                                as_str(): "21",
                                                            },
                                                            parsed: 21,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 441,
                                            end: 451,
                                            as_str(): "(2, 1, 21)",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 451,
                                        end: 452,
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
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 454,
                                            end: 456,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 457,
                                            end: 468,
                                            as_str(): "arr_wrapper",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 469,
                                                                        end: 470,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 470,
                                                                    end: 471,
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
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 472,
                                                                                end: 475,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 475,
                                                                end: 476,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 477,
                                                                        end: 478,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 478,
                                                                    end: 479,
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
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 480,
                                                                                end: 483,
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
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 483,
                                                                end: 484,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 485,
                                                                    end: 486,
                                                                    as_str(): "c",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 486,
                                                                end: 487,
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
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 488,
                                                                            end: 491,
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 468,
                                            end: 492,
                                            as_str(): "(a: u64, b: u64, c: u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 493,
                                                    end: 495,
                                                    as_str(): "->",
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
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 497,
                                                                            end: 500,
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 500,
                                                                end: 501,
                                                                as_str(): ";",
                                                            },
                                                        },
                                                        length: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 502,
                                                                        end: 503,
                                                                        as_str(): "3",
                                                                    },
                                                                    parsed: 3,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 496,
                                                        end: 504,
                                                        as_str(): "[u64; 3]",
                                                    },
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
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 511,
                                                            end: 517,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Array(
                                                            SquareBrackets {
                                                                inner: Sequence(
                                                                    Punctuated {
                                                                        value_separator_pairs: [
                                                                            (
                                                                                Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 519,
                                                                                                    end: 520,
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
                                                                                CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 520,
                                                                                        end: 521,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 522,
                                                                                                    end: 523,
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
                                                                                CommaToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 523,
                                                                                        end: 524,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        final_value_opt: Some(
                                                                            Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 525,
                                                                                                end: 526,
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
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 518,
                                                                    end: 527,
                                                                    as_str(): "[a, b, c]",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 527,
                                                            end: 528,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 505,
                                        end: 530,
                                        as_str(): "{\n    return [a, b, c];\n}",
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 531,
                                        end: 536,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 537,
                                        end: 541,
                                        as_str(): "ARR2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 542,
                                        end: 543,
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
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 544,
                                                        end: 555,
                                                        as_str(): "arr_wrapper",
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
                                            value_separator_pairs: [
                                                (
                                                    Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 556,
                                                                    end: 557,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    CommaToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 557,
                                                            end: 558,
                                                            as_str(): ",",
                                                        },
                                                    },
                                                ),
                                                (
                                                    Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 559,
                                                                    end: 560,
                                                                    as_str(): "2",
                                                                },
                                                                parsed: 2,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    CommaToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 560,
                                                            end: 561,
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 562,
                                                                end: 563,
                                                                as_str(): "3",
                                                            },
                                                            parsed: 3,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 555,
                                            end: 564,
                                            as_str(): "(1, 2, 3)",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 564,
                                        end: 565,
                                        as_str(): ";",
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 567,
                                        end: 571,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 572,
                                        end: 575,
                                        as_str(): "En1",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 582,
                                                                end: 585,
                                                                as_str(): "Int",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 585,
                                                                end: 586,
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
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 587,
                                                                            end: 590,
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
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 590,
                                                        end: 591,
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 596,
                                                                end: 599,
                                                                as_str(): "Arr",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 599,
                                                                end: 600,
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 602,
                                                                                        end: 605,
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
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 605,
                                                                            end: 606,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                    length: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 607,
                                                                                    end: 608,
                                                                                    as_str(): "3",
                                                                                },
                                                                                parsed: 3,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 601,
                                                                    end: 609,
                                                                    as_str(): "[u64; 3]",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 609,
                                                        end: 610,
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 615,
                                                                end: 620,
                                                                as_str(): "NoVal",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 620,
                                                                end: 621,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 622,
                                                                    end: 624,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 624,
                                                        end: 625,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 576,
                                        end: 627,
                                        as_str(): "{\n    Int: u64,\n    Arr: [u64; 3],\n    NoVal: (),\n}",
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 629,
                                        end: 634,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 635,
                                        end: 639,
                                        as_str(): "EN1a",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 640,
                                        end: 641,
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
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 642,
                                                        end: 645,
                                                        as_str(): "En1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [
                                                (
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 645,
                                                            end: 647,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 647,
                                                                end: 650,
                                                                as_str(): "Int",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 651,
                                                                end: 654,
                                                                as_str(): "101",
                                                            },
                                                            parsed: 101,
                                                            ty_opt: None,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 650,
                                            end: 655,
                                            as_str(): "(101)",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 655,
                                        end: 656,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 657,
                                        end: 662,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 663,
                                        end: 667,
                                        as_str(): "EN1b",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 668,
                                        end: 669,
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
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 670,
                                                        end: 673,
                                                        as_str(): "En1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [
                                                (
                                                    DoubleColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 673,
                                                            end: 675,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 675,
                                                                end: 678,
                                                                as_str(): "Arr",
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
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 679,
                                                                    end: 683,
                                                                    as_str(): "ARR2",
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
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 678,
                                            end: 684,
                                            as_str(): "(ARR2)",
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 684,
                                        end: 685,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 686,
                                        end: 691,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 692,
                                        end: 696,
                                        as_str(): "EN1c",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 697,
                                        end: 698,
                                        as_str(): "=",
                                    },
                                },
                                expr: Path(
                                    PathExpr {
                                        root_opt: None,
                                        prefix: PathExprSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 699,
                                                    end: 702,
                                                    as_str(): "En1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [
                                            (
                                                DoubleColonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 702,
                                                        end: 704,
                                                        as_str(): "::",
                                                    },
                                                },
                                                PathExprSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 704,
                                                            end: 709,
                                                            as_str(): "NoVal",
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
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 709,
                                        end: 710,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 712,
                                        end: 717,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 718,
                                        end: 731,
                                        as_str(): "ETH_ID0_VALUE",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 732,
                                        end: 733,
                                        as_str(): "=",
                                    },
                                },
                                expr: FieldProjection {
                                    target: Path(
                                        PathExpr {
                                            root_opt: None,
                                            prefix: PathExprSegment {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 734,
                                                        end: 741,
                                                        as_str(): "ETH_ID0",
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
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 741,
                                            end: 742,
                                            as_str(): ".",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 742,
                                            end: 747,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 747,
                                        end: 748,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 749,
                                        end: 754,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 755,
                                        end: 764,
                                        as_str(): "TUP1_idx2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 765,
                                        end: 766,
                                        as_str(): "=",
                                    },
                                },
                                expr: TupleFieldProjection {
                                    target: Path(
                                        PathExpr {
                                            root_opt: None,
                                            prefix: PathExprSegment {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 767,
                                                        end: 771,
                                                        as_str(): "TUP1",
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
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 771,
                                            end: 772,
                                            as_str(): ".",
                                        },
                                    },
                                    field: 2,
                                    field_span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 772,
                                        end: 773,
                                        as_str(): "2",
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 773,
                                        end: 774,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 776,
                                        end: 781,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 782,
                                        end: 786,
                                        as_str(): "INT1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 787,
                                        end: 788,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 789,
                                                end: 790,
                                                as_str(): "1",
                                            },
                                            parsed: 1,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 790,
                                        end: 791,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 793,
                                        end: 798,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 799,
                                        end: 808,
                                        as_str(): "ZERO_B256",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 809,
                                        end: 810,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 811,
                                                end: 877,
                                                as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                            },
                                            parsed: 0,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 877,
                                        end: 878,
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
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 879,
                                        end: 884,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 885,
                                        end: 888,
                                        as_str(): "KEY",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 889,
                                        end: 890,
                                        as_str(): "=",
                                    },
                                },
                                expr: Path(
                                    PathExpr {
                                        root_opt: None,
                                        prefix: PathExprSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 891,
                                                    end: 900,
                                                    as_str(): "ZERO_B256",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                        incomplete_suffix: false,
                                    },
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 900,
                                        end: 901,
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
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 903,
                                            end: 905,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 906,
                                            end: 910,
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
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 910,
                                            end: 912,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 913,
                                                    end: 915,
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 916,
                                                                end: 919,
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
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Const(
                                                        ItemConst {
                                                            visibility: None,
                                                            const_token: ConstToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 926,
                                                                    end: 931,
                                                                    as_str(): "const",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 932,
                                                                    end: 936,
                                                                    as_str(): "int1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            ty_opt: None,
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 937,
                                                                    end: 938,
                                                                    as_str(): "=",
                                                                },
                                                            },
                                                            expr: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 939,
                                                                            end: 940,
                                                                            as_str(): "1",
                                                                        },
                                                                        parsed: 1,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            semicolon_token: SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 940,
                                                                    end: 941,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        },
                                                    ),
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 946,
                                                                        end: 952,
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
                                                                LogicalAnd {
                                                                    lhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 953,
                                                                                            end: 957,
                                                                                            as_str(): "int1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 958,
                                                                                end: 960,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 961,
                                                                                            end: 965,
                                                                                            as_str(): "INT1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 966,
                                                                            end: 968,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 969,
                                                                                            end: 978,
                                                                                            as_str(): "ZERO_B256",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 979,
                                                                                end: 981,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 982,
                                                                                            end: 985,
                                                                                            as_str(): "KEY",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 952,
                                                            end: 986,
                                                            as_str(): "(int1 == INT1 && ZERO_B256 == KEY)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 986,
                                                            end: 987,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Const(
                                                        ItemConst {
                                                            visibility: None,
                                                            const_token: ConstToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1046,
                                                                    end: 1051,
                                                                    as_str(): "const",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1052,
                                                                    end: 1059,
                                                                    as_str(): "eth_id0",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            ty_opt: None,
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1060,
                                                                    end: 1061,
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1062,
                                                                                    end: 1072,
                                                                                    as_str(): "ContractId",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1072,
                                                                                        end: 1074,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1074,
                                                                                            end: 1078,
                                                                                            as_str(): "from",
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
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1079,
                                                                                            end: 1145,
                                                                                            as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                                                        },
                                                                                        parsed: 0,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1078,
                                                                        end: 1146,
                                                                        as_str(): "(0x0000000000000000000000000000000000000000000000000000000000000000)",
                                                                    },
                                                                },
                                                            },
                                                            semicolon_token: SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1146,
                                                                    end: 1147,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Const(
                                                        ItemConst {
                                                            visibility: None,
                                                            const_token: ConstToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1152,
                                                                    end: 1157,
                                                                    as_str(): "const",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1158,
                                                                    end: 1165,
                                                                    as_str(): "eth_id1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            ty_opt: None,
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1166,
                                                                    end: 1167,
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1168,
                                                                                    end: 1178,
                                                                                    as_str(): "ContractId",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1178,
                                                                                        end: 1180,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1180,
                                                                                            end: 1184,
                                                                                            as_str(): "from",
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
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1185,
                                                                                            end: 1251,
                                                                                            as_str(): "0x0000000000000000000000000000000000000000000000000000000000000001",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: None,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1184,
                                                                        end: 1252,
                                                                        as_str(): "(0x0000000000000000000000000000000000000000000000000000000000000001)",
                                                                    },
                                                                },
                                                            },
                                                            semicolon_token: SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1252,
                                                                    end: 1253,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        },
                                                    ),
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1258,
                                                                        end: 1264,
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
                                                                LogicalAnd {
                                                                    lhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1265,
                                                                                            end: 1272,
                                                                                            as_str(): "eth_id0",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1273,
                                                                                end: 1275,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1276,
                                                                                            end: 1283,
                                                                                            as_str(): "ETH_ID0",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1284,
                                                                            end: 1286,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1287,
                                                                                            end: 1294,
                                                                                            as_str(): "eth_id1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1295,
                                                                                end: 1297,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1298,
                                                                                            end: 1305,
                                                                                            as_str(): "ETH_ID1",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1264,
                                                            end: 1306,
                                                            as_str(): "(eth_id0 == ETH_ID0 && eth_id1 == ETH_ID1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1306,
                                                            end: 1307,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Const(
                                                        ItemConst {
                                                            visibility: None,
                                                            const_token: ConstToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1339,
                                                                    end: 1344,
                                                                    as_str(): "const",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1345,
                                                                    end: 1347,
                                                                    as_str(): "t1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            ty_opt: None,
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1348,
                                                                    end: 1349,
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1351,
                                                                                        end: 1352,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        comma_token: CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1352,
                                                                                end: 1353,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                        tail: Punctuated {
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1354,
                                                                                                    end: 1355,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1355,
                                                                                            end: 1356,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1357,
                                                                                                end: 1359,
                                                                                                as_str(): "21",
                                                                                            },
                                                                                            parsed: 21,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1350,
                                                                        end: 1360,
                                                                        as_str(): "(2, 1, 21)",
                                                                    },
                                                                },
                                                            ),
                                                            semicolon_token: SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1360,
                                                                    end: 1361,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        },
                                                    ),
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1366,
                                                                        end: 1372,
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
                                                                LogicalAnd {
                                                                    lhs: LogicalAnd {
                                                                        lhs: Equal {
                                                                            lhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1373,
                                                                                                    end: 1375,
                                                                                                    as_str(): "t1",
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1375,
                                                                                        end: 1376,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 0,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1376,
                                                                                    end: 1377,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1378,
                                                                                    end: 1380,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1381,
                                                                                                    end: 1385,
                                                                                                    as_str(): "TUP1",
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1385,
                                                                                        end: 1386,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 0,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1386,
                                                                                    end: 1387,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        },
                                                                        double_ampersand_token: DoubleAmpersandToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1388,
                                                                                end: 1390,
                                                                                as_str(): "&&",
                                                                            },
                                                                        },
                                                                        rhs: Equal {
                                                                            lhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1391,
                                                                                                    end: 1393,
                                                                                                    as_str(): "t1",
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1393,
                                                                                        end: 1394,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 1,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1394,
                                                                                    end: 1395,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1396,
                                                                                    end: 1398,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1399,
                                                                                                    end: 1403,
                                                                                                    as_str(): "TUP1",
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1403,
                                                                                        end: 1404,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 1,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1404,
                                                                                    end: 1405,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1406,
                                                                            end: 1408,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: TupleFieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1409,
                                                                                                end: 1411,
                                                                                                as_str(): "t1",
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1411,
                                                                                    end: 1412,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            field: 2,
                                                                            field_span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1412,
                                                                                end: 1413,
                                                                                as_str(): "2",
                                                                            },
                                                                        },
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1414,
                                                                                end: 1416,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: TupleFieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1417,
                                                                                                end: 1421,
                                                                                                as_str(): "TUP1",
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1421,
                                                                                    end: 1422,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            field: 2,
                                                                            field_span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1422,
                                                                                end: 1423,
                                                                                as_str(): "2",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1372,
                                                            end: 1424,
                                                            as_str(): "(t1.0 == TUP1.0 && t1.1 == TUP1.1 && t1.2 == TUP1.2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1424,
                                                            end: 1425,
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1430,
                                                                        end: 1436,
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
                                                                LogicalAnd {
                                                                    lhs: LogicalAnd {
                                                                        lhs: Equal {
                                                                            lhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1437,
                                                                                                    end: 1439,
                                                                                                    as_str(): "t1",
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1439,
                                                                                        end: 1440,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 0,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1440,
                                                                                    end: 1441,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1442,
                                                                                    end: 1444,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1445,
                                                                                                    end: 1449,
                                                                                                    as_str(): "TUP2",
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1449,
                                                                                        end: 1450,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 0,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1450,
                                                                                    end: 1451,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        },
                                                                        double_ampersand_token: DoubleAmpersandToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1452,
                                                                                end: 1454,
                                                                                as_str(): "&&",
                                                                            },
                                                                        },
                                                                        rhs: Equal {
                                                                            lhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1455,
                                                                                                    end: 1457,
                                                                                                    as_str(): "t1",
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1457,
                                                                                        end: 1458,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 1,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1458,
                                                                                    end: 1459,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1460,
                                                                                    end: 1462,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: TupleFieldProjection {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1463,
                                                                                                    end: 1467,
                                                                                                    as_str(): "TUP2",
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1467,
                                                                                        end: 1468,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                field: 1,
                                                                                field_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1468,
                                                                                    end: 1469,
                                                                                    as_str(): "1",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1470,
                                                                            end: 1472,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: TupleFieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1473,
                                                                                                end: 1475,
                                                                                                as_str(): "t1",
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1475,
                                                                                    end: 1476,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            field: 2,
                                                                            field_span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1476,
                                                                                end: 1477,
                                                                                as_str(): "2",
                                                                            },
                                                                        },
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1478,
                                                                                end: 1480,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: TupleFieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1481,
                                                                                                end: 1485,
                                                                                                as_str(): "TUP2",
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1485,
                                                                                    end: 1486,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            field: 2,
                                                                            field_span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1486,
                                                                                end: 1487,
                                                                                as_str(): "2",
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1436,
                                                            end: 1488,
                                                            as_str(): "(t1.0 == TUP2.0 && t1.1 == TUP2.1 && t1.2 == TUP2.2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1488,
                                                            end: 1489,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Const(
                                                        ItemConst {
                                                            visibility: None,
                                                            const_token: ConstToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1494,
                                                                    end: 1499,
                                                                    as_str(): "const",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1500,
                                                                    end: 1502,
                                                                    as_str(): "a1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            ty_opt: None,
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1503,
                                                                    end: 1504,
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1506,
                                                                                                    end: 1507,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1507,
                                                                                            end: 1508,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1509,
                                                                                                    end: 1510,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                                parsed: 2,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1510,
                                                                                            end: 1511,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1512,
                                                                                                end: 1513,
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1505,
                                                                        end: 1514,
                                                                        as_str(): "[1, 2, 3]",
                                                                    },
                                                                },
                                                            ),
                                                            semicolon_token: SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1514,
                                                                    end: 1515,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        },
                                                    ),
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1520,
                                                                        end: 1526,
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
                                                                LogicalAnd {
                                                                    lhs: LogicalAnd {
                                                                        lhs: Equal {
                                                                            lhs: Index {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1527,
                                                                                                    end: 1529,
                                                                                                    as_str(): "a1",
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1530,
                                                                                                    end: 1531,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1529,
                                                                                        end: 1532,
                                                                                        as_str(): "[0]",
                                                                                    },
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1533,
                                                                                    end: 1535,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: Index {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1536,
                                                                                                    end: 1540,
                                                                                                    as_str(): "ARR1",
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1541,
                                                                                                    end: 1542,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1540,
                                                                                        end: 1543,
                                                                                        as_str(): "[0]",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        double_ampersand_token: DoubleAmpersandToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1544,
                                                                                end: 1546,
                                                                                as_str(): "&&",
                                                                            },
                                                                        },
                                                                        rhs: Equal {
                                                                            lhs: Index {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1547,
                                                                                                    end: 1549,
                                                                                                    as_str(): "a1",
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1550,
                                                                                                    end: 1551,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1549,
                                                                                        end: 1552,
                                                                                        as_str(): "[1]",
                                                                                    },
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1553,
                                                                                    end: 1555,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: Index {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1556,
                                                                                                    end: 1560,
                                                                                                    as_str(): "ARR1",
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1561,
                                                                                                    end: 1562,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1560,
                                                                                        end: 1563,
                                                                                        as_str(): "[1]",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1564,
                                                                            end: 1566,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: Index {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1567,
                                                                                                end: 1569,
                                                                                                as_str(): "a1",
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1570,
                                                                                                end: 1571,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                            parsed: 2,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1569,
                                                                                    end: 1572,
                                                                                    as_str(): "[2]",
                                                                                },
                                                                            },
                                                                        },
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1573,
                                                                                end: 1575,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Index {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1576,
                                                                                                end: 1580,
                                                                                                as_str(): "ARR1",
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1581,
                                                                                                end: 1582,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                            parsed: 2,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1580,
                                                                                    end: 1583,
                                                                                    as_str(): "[2]",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1526,
                                                            end: 1584,
                                                            as_str(): "(a1[0] == ARR1[0] && a1[1] == ARR1[1] && a1[2] == ARR1[2])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1584,
                                                            end: 1585,
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1590,
                                                                        end: 1596,
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
                                                                LogicalAnd {
                                                                    lhs: LogicalAnd {
                                                                        lhs: Equal {
                                                                            lhs: Index {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1597,
                                                                                                    end: 1599,
                                                                                                    as_str(): "a1",
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1600,
                                                                                                    end: 1601,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1599,
                                                                                        end: 1602,
                                                                                        as_str(): "[0]",
                                                                                    },
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1603,
                                                                                    end: 1605,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: Index {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1606,
                                                                                                    end: 1610,
                                                                                                    as_str(): "ARR2",
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1611,
                                                                                                    end: 1612,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1610,
                                                                                        end: 1613,
                                                                                        as_str(): "[0]",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        double_ampersand_token: DoubleAmpersandToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1614,
                                                                                end: 1616,
                                                                                as_str(): "&&",
                                                                            },
                                                                        },
                                                                        rhs: Equal {
                                                                            lhs: Index {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1617,
                                                                                                    end: 1619,
                                                                                                    as_str(): "a1",
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1620,
                                                                                                    end: 1621,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1619,
                                                                                        end: 1622,
                                                                                        as_str(): "[1]",
                                                                                    },
                                                                                },
                                                                            },
                                                                            double_eq_token: DoubleEqToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1623,
                                                                                    end: 1625,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            rhs: Index {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1626,
                                                                                                    end: 1630,
                                                                                                    as_str(): "ARR2",
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1631,
                                                                                                    end: 1632,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1630,
                                                                                        end: 1633,
                                                                                        as_str(): "[1]",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    double_ampersand_token: DoubleAmpersandToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1634,
                                                                            end: 1636,
                                                                            as_str(): "&&",
                                                                        },
                                                                    },
                                                                    rhs: Equal {
                                                                        lhs: Index {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1637,
                                                                                                end: 1639,
                                                                                                as_str(): "a1",
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1640,
                                                                                                end: 1641,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                            parsed: 2,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1639,
                                                                                    end: 1642,
                                                                                    as_str(): "[2]",
                                                                                },
                                                                            },
                                                                        },
                                                                        double_eq_token: DoubleEqToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1643,
                                                                                end: 1645,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                        rhs: Index {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1646,
                                                                                                end: 1650,
                                                                                                as_str(): "ARR2",
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1651,
                                                                                                end: 1652,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                            parsed: 2,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1650,
                                                                                    end: 1653,
                                                                                    as_str(): "[2]",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1596,
                                                            end: 1654,
                                                            as_str(): "(a1[0] == ARR2[0] && a1[1] == ARR2[1] && a1[2] == ARR2[2])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1654,
                                                            end: 1655,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1673,
                                                            end: 1678,
                                                            as_str(): "match",
                                                        },
                                                    },
                                                    value: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1679,
                                                                        end: 1683,
                                                                        as_str(): "EN1a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    branches: Braces {
                                                        inner: [
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1694,
                                                                                    end: 1697,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1697,
                                                                                        end: 1699,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1699,
                                                                                            end: 1702,
                                                                                            as_str(): "Int",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Var {
                                                                                    reference: None,
                                                                                    mutable: None,
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1703,
                                                                                            end: 1704,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1702,
                                                                            end: 1705,
                                                                            as_str(): "(i)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1706,
                                                                        end: 1708,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1709,
                                                                                            end: 1715,
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
                                                                                        lhs: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1716,
                                                                                                            end: 1717,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                                suffix: [],
                                                                                                incomplete_suffix: false,
                                                                                            },
                                                                                        ),
                                                                                        double_eq_token: DoubleEqToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1718,
                                                                                                end: 1720,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        rhs: Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1721,
                                                                                                        end: 1724,
                                                                                                        as_str(): "101",
                                                                                                    },
                                                                                                    parsed: 101,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1715,
                                                                                end: 1725,
                                                                                as_str(): "(i == 101)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1725,
                                                                            end: 1726,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1735,
                                                                                    end: 1738,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1738,
                                                                                        end: 1740,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1740,
                                                                                            end: 1743,
                                                                                            as_str(): "Arr",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Wildcard {
                                                                                    underscore_token: UnderscoreToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1744,
                                                                                            end: 1745,
                                                                                            as_str(): "_",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1743,
                                                                            end: 1746,
                                                                            as_str(): "(_)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1747,
                                                                        end: 1749,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1750,
                                                                                            end: 1756,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1757,
                                                                                                    end: 1762,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1756,
                                                                                end: 1763,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1763,
                                                                            end: 1764,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Constant(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1773,
                                                                                    end: 1776,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1776,
                                                                                        end: 1778,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1778,
                                                                                            end: 1783,
                                                                                            as_str(): "NoVal",
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
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1784,
                                                                        end: 1786,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1787,
                                                                                            end: 1793,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1794,
                                                                                                    end: 1799,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1793,
                                                                                end: 1800,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1800,
                                                                            end: 1801,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1684,
                                                            end: 1807,
                                                            as_str(): "{\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1812,
                                                            end: 1817,
                                                            as_str(): "match",
                                                        },
                                                    },
                                                    value: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1818,
                                                                        end: 1822,
                                                                        as_str(): "EN1b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    branches: Braces {
                                                        inner: [
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1833,
                                                                                    end: 1836,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1836,
                                                                                        end: 1838,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1838,
                                                                                            end: 1841,
                                                                                            as_str(): "Int",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Var {
                                                                                    reference: None,
                                                                                    mutable: None,
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1842,
                                                                                            end: 1843,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1841,
                                                                            end: 1844,
                                                                            as_str(): "(i)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1845,
                                                                        end: 1847,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1848,
                                                                                            end: 1854,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1855,
                                                                                                    end: 1860,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1854,
                                                                                end: 1861,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1861,
                                                                            end: 1862,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1871,
                                                                                    end: 1874,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1874,
                                                                                        end: 1876,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1876,
                                                                                            end: 1879,
                                                                                            as_str(): "Arr",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Var {
                                                                                    reference: None,
                                                                                    mutable: None,
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1880,
                                                                                            end: 1883,
                                                                                            as_str(): "arr",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1879,
                                                                            end: 1884,
                                                                            as_str(): "(arr)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1885,
                                                                        end: 1887,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1902,
                                                                                                            end: 1908,
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
                                                                                                    LogicalAnd {
                                                                                                        lhs: LogicalAnd {
                                                                                                            lhs: Equal {
                                                                                                                lhs: Index {
                                                                                                                    target: Path(
                                                                                                                        PathExpr {
                                                                                                                            root_opt: None,
                                                                                                                            prefix: PathExprSegment {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1909,
                                                                                                                                        end: 1912,
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
                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1913,
                                                                                                                                        end: 1914,
                                                                                                                                        as_str(): "0",
                                                                                                                                    },
                                                                                                                                    parsed: 0,
                                                                                                                                    ty_opt: None,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1912,
                                                                                                                            end: 1915,
                                                                                                                            as_str(): "[0]",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                                double_eq_token: DoubleEqToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1916,
                                                                                                                        end: 1918,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                },
                                                                                                                rhs: Index {
                                                                                                                    target: Path(
                                                                                                                        PathExpr {
                                                                                                                            root_opt: None,
                                                                                                                            prefix: PathExprSegment {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1919,
                                                                                                                                        end: 1923,
                                                                                                                                        as_str(): "ARR1",
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
                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1924,
                                                                                                                                        end: 1925,
                                                                                                                                        as_str(): "0",
                                                                                                                                    },
                                                                                                                                    parsed: 0,
                                                                                                                                    ty_opt: None,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1923,
                                                                                                                            end: 1926,
                                                                                                                            as_str(): "[0]",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                            double_ampersand_token: DoubleAmpersandToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1927,
                                                                                                                    end: 1929,
                                                                                                                    as_str(): "&&",
                                                                                                                },
                                                                                                            },
                                                                                                            rhs: Equal {
                                                                                                                lhs: Index {
                                                                                                                    target: Path(
                                                                                                                        PathExpr {
                                                                                                                            root_opt: None,
                                                                                                                            prefix: PathExprSegment {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1930,
                                                                                                                                        end: 1933,
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
                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1934,
                                                                                                                                        end: 1935,
                                                                                                                                        as_str(): "1",
                                                                                                                                    },
                                                                                                                                    parsed: 1,
                                                                                                                                    ty_opt: None,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1933,
                                                                                                                            end: 1936,
                                                                                                                            as_str(): "[1]",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                                double_eq_token: DoubleEqToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1937,
                                                                                                                        end: 1939,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                },
                                                                                                                rhs: Index {
                                                                                                                    target: Path(
                                                                                                                        PathExpr {
                                                                                                                            root_opt: None,
                                                                                                                            prefix: PathExprSegment {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1940,
                                                                                                                                        end: 1944,
                                                                                                                                        as_str(): "ARR1",
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
                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 1945,
                                                                                                                                        end: 1946,
                                                                                                                                        as_str(): "1",
                                                                                                                                    },
                                                                                                                                    parsed: 1,
                                                                                                                                    ty_opt: None,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1944,
                                                                                                                            end: 1947,
                                                                                                                            as_str(): "[1]",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                        double_ampersand_token: DoubleAmpersandToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1948,
                                                                                                                end: 1950,
                                                                                                                as_str(): "&&",
                                                                                                            },
                                                                                                        },
                                                                                                        rhs: Equal {
                                                                                                            lhs: Index {
                                                                                                                target: Path(
                                                                                                                    PathExpr {
                                                                                                                        root_opt: None,
                                                                                                                        prefix: PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1951,
                                                                                                                                    end: 1954,
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
                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1955,
                                                                                                                                    end: 1956,
                                                                                                                                    as_str(): "2",
                                                                                                                                },
                                                                                                                                parsed: 2,
                                                                                                                                ty_opt: None,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1954,
                                                                                                                        end: 1957,
                                                                                                                        as_str(): "[2]",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                            double_eq_token: DoubleEqToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1958,
                                                                                                                    end: 1960,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                            },
                                                                                                            rhs: Index {
                                                                                                                target: Path(
                                                                                                                    PathExpr {
                                                                                                                        root_opt: None,
                                                                                                                        prefix: PathExprSegment {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1961,
                                                                                                                                    end: 1965,
                                                                                                                                    as_str(): "ARR1",
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
                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1966,
                                                                                                                                    end: 1967,
                                                                                                                                    as_str(): "2",
                                                                                                                                },
                                                                                                                                parsed: 2,
                                                                                                                                ty_opt: None,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1965,
                                                                                                                        end: 1968,
                                                                                                                        as_str(): "[2]",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1908,
                                                                                                end: 1969,
                                                                                                as_str(): "(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2])",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                    semicolon_token_opt: Some(
                                                                                        SemicolonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1969,
                                                                                                end: 1970,
                                                                                                as_str(): ";",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1888,
                                                                            end: 1980,
                                                                            as_str(): "{\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }",
                                                                        },
                                                                    },
                                                                    comma_token_opt: None,
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Constant(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1989,
                                                                                    end: 1992,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1992,
                                                                                        end: 1994,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1994,
                                                                                            end: 1999,
                                                                                            as_str(): "NoVal",
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
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2000,
                                                                        end: 2002,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2003,
                                                                                            end: 2009,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2010,
                                                                                                    end: 2015,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 2009,
                                                                                end: 2016,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2016,
                                                                            end: 2017,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1823,
                                                            end: 2023,
                                                            as_str(): "{\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
                                            },
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2028,
                                                            end: 2033,
                                                            as_str(): "match",
                                                        },
                                                    },
                                                    value: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2034,
                                                                        end: 2038,
                                                                        as_str(): "EN1c",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    branches: Braces {
                                                        inner: [
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2049,
                                                                                    end: 2052,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2052,
                                                                                        end: 2054,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2054,
                                                                                            end: 2057,
                                                                                            as_str(): "Int",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Var {
                                                                                    reference: None,
                                                                                    mutable: None,
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2058,
                                                                                            end: 2059,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2057,
                                                                            end: 2060,
                                                                            as_str(): "(i)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2061,
                                                                        end: 2063,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2064,
                                                                                            end: 2070,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2071,
                                                                                                    end: 2076,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 2070,
                                                                                end: 2077,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2077,
                                                                            end: 2078,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2087,
                                                                                    end: 2090,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2090,
                                                                                        end: 2092,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2092,
                                                                                            end: 2095,
                                                                                            as_str(): "Arr",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Wildcard {
                                                                                    underscore_token: UnderscoreToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2096,
                                                                                            end: 2097,
                                                                                            as_str(): "_",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2095,
                                                                            end: 2098,
                                                                            as_str(): "(_)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2099,
                                                                        end: 2101,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2102,
                                                                                            end: 2108,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2109,
                                                                                                    end: 2114,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                                kind: False,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 2108,
                                                                                end: 2115,
                                                                                as_str(): "(false)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2115,
                                                                            end: 2116,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Constant(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2125,
                                                                                    end: 2128,
                                                                                    as_str(): "En1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2128,
                                                                                        end: 2130,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2130,
                                                                                            end: 2135,
                                                                                            as_str(): "NoVal",
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
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2136,
                                                                        end: 2138,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2139,
                                                                                            end: 2145,
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
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2146,
                                                                                                    end: 2150,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 2145,
                                                                                end: 2151,
                                                                                as_str(): "(true)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2151,
                                                                            end: 2152,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2039,
                                                            end: 2158,
                                                            as_str(): "{\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: None,
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2201,
                                                                        end: 2207,
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
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2208,
                                                                                            end: 2215,
                                                                                            as_str(): "ETH_ID0",
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
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 2215,
                                                                                end: 2216,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 2216,
                                                                                end: 2221,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2222,
                                                                            end: 2224,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2225,
                                                                                        end: 2238,
                                                                                        as_str(): "ETH_ID0_VALUE",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2207,
                                                            end: 2239,
                                                            as_str(): "(ETH_ID0.value == ETH_ID0_VALUE)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2239,
                                                            end: 2240,
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
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2245,
                                                                        end: 2251,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2252,
                                                                                        end: 2261,
                                                                                        as_str(): "TUP1_idx2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2262,
                                                                            end: 2264,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: TupleFieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2265,
                                                                                            end: 2269,
                                                                                            as_str(): "TUP1",
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
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 2269,
                                                                                end: 2270,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        field: 2,
                                                                        field_span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2270,
                                                                            end: 2271,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2251,
                                                            end: 2272,
                                                            as_str(): "(TUP1_idx2 == TUP1.2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2272,
                                                            end: 2273,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2279,
                                                            end: 2280,
                                                            as_str(): "1",
                                                        },
                                                        parsed: 1,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 920,
                                        end: 2282,
                                        as_str(): "{\n    const int1 = 1;\n    assert(int1 == INT1 && ZERO_B256 == KEY);\n\n    // initialization through function applications.\n    const eth_id0 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);\n    const eth_id1 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000001);\n    assert(eth_id0 == ETH_ID0 && eth_id1 == ETH_ID1);\n\n    // tuples and arrays.\n    const t1 = (2, 1, 21);\n    assert(t1.0 == TUP1.0 && t1.1 == TUP1.1 && t1.2 == TUP1.2);\n    assert(t1.0 == TUP2.0 && t1.1 == TUP2.1 && t1.2 == TUP2.2);\n    const a1 = [1, 2, 3];\n    assert(a1[0] == ARR1[0] && a1[1] == ARR1[1] && a1[2] == ARR1[2]);\n    assert(a1[0] == ARR2[0] && a1[1] == ARR2[1] && a1[2] == ARR2[2]);\n\n    // enum\n    match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }\n    match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }\n    match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }\n\n    // Struct and enum field access.\n    assert(ETH_ID0.value == ETH_ID0_VALUE);\n    assert(TUP1_idx2 == TUP1.2);\n\n    1\n}",
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
