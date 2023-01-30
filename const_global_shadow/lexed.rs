Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb12be97690,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb12be97690,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
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
                                        src (ptr): 0x00007fb12be97690,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 14,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb12be97690,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                        ),
                                        start: 15,
                                        end: 25,
                                        as_str(): "GLOBAL_VAL",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb12be97690,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                ),
                                                start: 25,
                                                end: 26,
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
                                                            src (ptr): 0x00007fb12be97690,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                            ),
                                                            start: 27,
                                                            end: 30,
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
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12be97690,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                        ),
                                        start: 31,
                                        end: 32,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fb12be97690,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                ),
                                                start: 33,
                                                end: 34,
                                                as_str(): "1",
                                            },
                                            parsed: 1,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb12be97690,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                        ),
                                        start: 34,
                                        end: 35,
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
                                            src (ptr): 0x00007fb12be97690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                            ),
                                            start: 37,
                                            end: 39,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb12be97690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                            ),
                                            start: 40,
                                            end: 44,
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
                                            src (ptr): 0x00007fb12be97690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                            ),
                                            start: 44,
                                            end: 46,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb12be97690,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                    ),
                                                    start: 47,
                                                    end: 49,
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
                                                                src (ptr): 0x00007fb12be97690,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                ),
                                                                start: 50,
                                                                end: 53,
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
                                                                    src (ptr): 0x00007fb12be97690,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                    ),
                                                                    start: 60,
                                                                    end: 65,
                                                                    as_str(): "const",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12be97690,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                    ),
                                                                    start: 66,
                                                                    end: 76,
                                                                    as_str(): "GLOBAL_VAL",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            ty_opt: None,
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12be97690,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                    ),
                                                                    start: 77,
                                                                    end: 78,
                                                                    as_str(): "=",
                                                                },
                                                            },
                                                            expr: Literal(
                                                                Int(
                                                                    LitInt {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb12be97690,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                            ),
                                                                            start: 79,
                                                                            end: 82,
                                                                            as_str(): "100",
                                                                        },
                                                                        parsed: 100,
                                                                        ty_opt: None,
                                                                    },
                                                                ),
                                                            ),
                                                            semicolon_token: SemicolonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12be97690,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                    ),
                                                                    start: 82,
                                                                    end: 83,
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
                                                                    src (ptr): 0x00007fb12be97690,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                    ),
                                                                    start: 88,
                                                                    end: 93,
                                                                    as_str(): "const",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12be97690,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                    ),
                                                                    start: 94,
                                                                    end: 103,
                                                                    as_str(): "LOCAL_VAL",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            ty_opt: None,
                                                            eq_token: EqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb12be97690,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                    ),
                                                                    start: 104,
                                                                    end: 105,
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
                                                                                src (ptr): 0x00007fb12be97690,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                                ),
                                                                                start: 106,
                                                                                end: 116,
                                                                                as_str(): "GLOBAL_VAL",
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
                                                                    src (ptr): 0x00007fb12be97690,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                    ),
                                                                    start: 116,
                                                                    end: 117,
                                                                    as_str(): ";",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb12be97690,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 131,
                                                                as_str(): "LOCAL_VAL",
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
                                        src (ptr): 0x00007fb12be97690,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                        ),
                                        start: 54,
                                        end: 133,
                                        as_str(): "{\n    const GLOBAL_VAL = 100;\n    const LOCAL_VAL = GLOBAL_VAL;\n    LOCAL_VAL\n}",
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
