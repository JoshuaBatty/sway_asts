Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe08b1f4f00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Dependency(
                            Dependency {
                                dep_token: DepToken {
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "dep",
                                    },
                                },
                                path: DependencyPath {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08b1f4f00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    suffixes: [],
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 17,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 19,
                                        end: 22,
                                        as_str(): "use",
                                    },
                                },
                                root_import: Some(
                                    DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe08b1f4f00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                            ),
                                            start: 23,
                                            end: 25,
                                            as_str(): "::",
                                        },
                                    },
                                ),
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08b1f4f00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                            ),
                                            start: 25,
                                            end: 28,
                                            as_str(): "bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe08b1f4f00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                            ),
                                            start: 28,
                                            end: 30,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Group {
                                        imports: Braces {
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Rename {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 31,
                                                                    end: 35,
                                                                    as_str(): "Bar1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            as_token: AsToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 36,
                                                                    end: 38,
                                                                    as_str(): "as",
                                                                },
                                                            },
                                                            alias: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 39,
                                                                    end: 45,
                                                                    as_str(): "MyBar1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 45,
                                                                end: 46,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 47,
                                                                    end: 51,
                                                                    as_str(): "Bar2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 51,
                                                                end: 52,
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
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 53,
                                                                end: 63,
                                                                as_str(): "double_bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 65,
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
                                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                        ),
                                                                                        start: 66,
                                                                                        end: 76,
                                                                                        as_str(): "DoubleBar1",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                double_colon_token: DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                        ),
                                                                                        start: 76,
                                                                                        end: 78,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                suffix: Group {
                                                                                    imports: Braces {
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [],
                                                                                            final_value_opt: Some(
                                                                                                Rename {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 79,
                                                                                                            end: 83,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    as_token: AsToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 84,
                                                                                                            end: 86,
                                                                                                            as_str(): "as",
                                                                                                        },
                                                                                                    },
                                                                                                    alias: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 87,
                                                                                                            end: 99,
                                                                                                            as_str(): "MyDoubleBar1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                            ),
                                                                                            start: 78,
                                                                                            end: 100,
                                                                                            as_str(): "{self as MyDoubleBar1}",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                    ),
                                                                                    start: 100,
                                                                                    end: 101,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Path {
                                                                                prefix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                        ),
                                                                                        start: 102,
                                                                                        end: 112,
                                                                                        as_str(): "DoubleBar2",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                double_colon_token: DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                        ),
                                                                                        start: 112,
                                                                                        end: 114,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                suffix: Group {
                                                                                    imports: Braces {
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [],
                                                                                            final_value_opt: Some(
                                                                                                Rename {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 115,
                                                                                                            end: 119,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    as_token: AsToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 120,
                                                                                                            end: 122,
                                                                                                            as_str(): "as",
                                                                                                        },
                                                                                                    },
                                                                                                    alias: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 123,
                                                                                                            end: 135,
                                                                                                            as_str(): "MyDoubleBar2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                            ),
                                                                                            start: 114,
                                                                                            end: 136,
                                                                                            as_str(): "{self as MyDoubleBar2}",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                    ),
                                                                                    start: 136,
                                                                                    end: 137,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    final_value_opt: Some(
                                                                        Name {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                    ),
                                                                                    start: 138,
                                                                                    end: 148,
                                                                                    as_str(): "DoubleBar3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 65,
                                                                    end: 149,
                                                                    as_str(): "{DoubleBar1::{self as MyDoubleBar1}, DoubleBar2::{self as MyDoubleBar2}, DoubleBar3}",
                                                                },
                                                            },
                                                        },
                                                    },
                                                ),
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe08b1f4f00,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                ),
                                                start: 30,
                                                end: 150,
                                                as_str(): "{Bar1 as MyBar1, Bar2, double_bar::{DoubleBar1::{self as MyDoubleBar1}, DoubleBar2::{self as MyDoubleBar2}, DoubleBar3}}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 150,
                                        end: 151,
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
                                            src (ptr): 0x00007fe08b1f4f00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                            ),
                                            start: 153,
                                            end: 155,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08b1f4f00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                            ),
                                            start: 156,
                                            end: 160,
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
                                            src (ptr): 0x00007fe08b1f4f00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                            ),
                                            start: 160,
                                            end: 162,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 163,
                                                    end: 165,
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
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 166,
                                                                end: 170,
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
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 177,
                                                            end: 180,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 181,
                                                                end: 185,
                                                                as_str(): "bar1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 186,
                                                            end: 187,
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 188,
                                                                        end: 194,
                                                                        as_str(): "MyBar1",
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
                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                    ),
                                                                                    start: 205,
                                                                                    end: 206,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                            ),
                                                                                            start: 206,
                                                                                            end: 207,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                    ),
                                                                                                    start: 208,
                                                                                                    end: 209,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U32,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 209,
                                                                                                            end: 212,
                                                                                                            as_str(): "u32",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                ),
                                                                                start: 212,
                                                                                end: 213,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 195,
                                                                end: 219,
                                                                as_str(): "{\n        a: 5u32,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 219,
                                                            end: 220,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 225,
                                                            end: 228,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 229,
                                                                end: 233,
                                                                as_str(): "bar2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 234,
                                                            end: 235,
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 236,
                                                                        end: 240,
                                                                        as_str(): "Bar2",
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
                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                    ),
                                                                                    start: 251,
                                                                                    end: 252,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                            ),
                                                                                            start: 252,
                                                                                            end: 253,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                    ),
                                                                                                    start: 254,
                                                                                                    end: 255,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 255,
                                                                                                            end: 258,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                ),
                                                                                start: 258,
                                                                                end: 259,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 241,
                                                                end: 265,
                                                                as_str(): "{\n        a: 5u64,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 266,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 271,
                                                            end: 274,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 275,
                                                                end: 278,
                                                                as_str(): "db1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 279,
                                                            end: 280,
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 281,
                                                                        end: 293,
                                                                        as_str(): "MyDoubleBar1",
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
                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                    ),
                                                                                    start: 304,
                                                                                    end: 305,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                            ),
                                                                                            start: 305,
                                                                                            end: 306,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                    ),
                                                                                                    start: 307,
                                                                                                    end: 308,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U32,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 308,
                                                                                                            end: 311,
                                                                                                            as_str(): "u32",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                ),
                                                                                start: 311,
                                                                                end: 312,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 294,
                                                                end: 318,
                                                                as_str(): "{\n        a: 5u32,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 318,
                                                            end: 319,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 324,
                                                            end: 327,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 328,
                                                                end: 331,
                                                                as_str(): "db2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 332,
                                                            end: 333,
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 334,
                                                                        end: 346,
                                                                        as_str(): "MyDoubleBar2",
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
                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                    ),
                                                                                    start: 357,
                                                                                    end: 358,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                            ),
                                                                                            start: 358,
                                                                                            end: 359,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                    ),
                                                                                                    start: 360,
                                                                                                    end: 361,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 361,
                                                                                                            end: 364,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                ),
                                                                                start: 364,
                                                                                end: 365,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 347,
                                                                end: 371,
                                                                as_str(): "{\n        a: 5u64,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 371,
                                                            end: 372,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 377,
                                                            end: 380,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 381,
                                                                end: 384,
                                                                as_str(): "db3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 385,
                                                            end: 386,
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 387,
                                                                        end: 397,
                                                                        as_str(): "DoubleBar3",
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
                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                    ),
                                                                                    start: 408,
                                                                                    end: 409,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                            ),
                                                                                            start: 409,
                                                                                            end: 410,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                    ),
                                                                                                    start: 411,
                                                                                                    end: 412,
                                                                                                    as_str(): "5",
                                                                                                },
                                                                                                parsed: 5,
                                                                                                ty_opt: Some(
                                                                                                    (
                                                                                                        U64,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe08b1f4f00,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                                            ),
                                                                                                            start: 412,
                                                                                                            end: 415,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                                ),
                                                                                start: 415,
                                                                                end: 416,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 398,
                                                                end: 422,
                                                                as_str(): "{\n        a: 5u64,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 422,
                                                            end: 423,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08b1f4f00,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                            ),
                                                            start: 428,
                                                            end: 433,
                                                            as_str(): "false",
                                                        },
                                                        kind: False,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe08b1f4f00,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                        ),
                                        start: 171,
                                        end: 435,
                                        as_str(): "{\n    let bar1 = MyBar1 {\n        a: 5u32,\n    };\n    let bar2 = Bar2 {\n        a: 5u64,\n    };\n    let db1 = MyDoubleBar1 {\n        a: 5u32,\n    };\n    let db2 = MyDoubleBar2 {\n        a: 5u64,\n    };\n    let db3 = DoubleBar3 {\n        a: 5u64,\n    };\n    false\n}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [
                (
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0933fb3a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                            ),
                            start: 8,
                            end: 11,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                    LexedSubmodule {
                        library_name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0933fb3a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                ),
                                start: 8,
                                end: 11,
                                as_str(): "bar",
                            },
                            is_raw_ident: false,
                        },
                        module: LexedModule {
                            tree: Module {
                                kind: Library {
                                    library_token: LibraryToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0933fb3a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                            ),
                                            start: 0,
                                            end: 7,
                                            as_str(): "library",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0933fb3a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                            ),
                                            start: 8,
                                            end: 11,
                                            as_str(): "bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0933fb3a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                        ),
                                        start: 11,
                                        end: 12,
                                        as_str(): ";",
                                    },
                                },
                                items: [
                                    Annotated {
                                        attribute_list: [],
                                        value: Dependency(
                                            Dependency {
                                                dep_token: DepToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0933fb3a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                        ),
                                                        start: 13,
                                                        end: 16,
                                                        as_str(): "dep",
                                                    },
                                                },
                                                path: DependencyPath {
                                                    prefix: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0933fb3a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                            ),
                                                            start: 17,
                                                            end: 22,
                                                            as_str(): "inner",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    suffixes: [
                                                        (
                                                            ForwardSlashToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0933fb3a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                    ),
                                                                    start: 22,
                                                                    end: 23,
                                                                    as_str(): "/",
                                                                },
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0933fb3a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                    ),
                                                                    start: 23,
                                                                    end: 33,
                                                                    as_str(): "double_bar",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                    ],
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0933fb3a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                        ),
                                                        start: 33,
                                                        end: 34,
                                                        as_str(): ";",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Struct(
                                            ItemStruct {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0933fb3a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                            ),
                                                            start: 36,
                                                            end: 39,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0933fb3a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                        ),
                                                        start: 40,
                                                        end: 46,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0933fb3a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                        ),
                                                        start: 47,
                                                        end: 51,
                                                        as_str(): "Bar1",
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
                                                                                src (ptr): 0x00007fe0933fb3a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                                ),
                                                                                start: 58,
                                                                                end: 59,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0933fb3a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                                ),
                                                                                start: 59,
                                                                                end: 60,
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
                                                                                            src (ptr): 0x00007fe0933fb3a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                                            ),
                                                                                            start: 61,
                                                                                            end: 64,
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
                                                                        src (ptr): 0x00007fe0933fb3a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                        ),
                                                                        start: 64,
                                                                        end: 65,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0933fb3a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                        ),
                                                        start: 52,
                                                        end: 67,
                                                        as_str(): "{\n    a: u32,\n}",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    Annotated {
                                        attribute_list: [],
                                        value: Struct(
                                            ItemStruct {
                                                visibility: Some(
                                                    PubToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0933fb3a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                            ),
                                                            start: 69,
                                                            end: 72,
                                                            as_str(): "pub",
                                                        },
                                                    },
                                                ),
                                                struct_token: StructToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0933fb3a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                        ),
                                                        start: 73,
                                                        end: 79,
                                                        as_str(): "struct",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0933fb3a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                        ),
                                                        start: 80,
                                                        end: 84,
                                                        as_str(): "Bar2",
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
                                                                                src (ptr): 0x00007fe0933fb3a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                                ),
                                                                                start: 91,
                                                                                end: 92,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        colon_token: ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0933fb3a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                                ),
                                                                                start: 92,
                                                                                end: 93,
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
                                                                                            src (ptr): 0x00007fe0933fb3a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                                            ),
                                                                                            start: 94,
                                                                                            end: 97,
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
                                                                        src (ptr): 0x00007fe0933fb3a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                                        ),
                                                                        start: 97,
                                                                        end: 98,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0933fb3a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/bar.sw",
                                                        ),
                                                        start: 85,
                                                        end: 100,
                                                        as_str(): "{\n    a: u64,\n}",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                ],
                            },
                            submodules: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe08f6cb6e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                            ),
                                            start: 8,
                                            end: 18,
                                            as_str(): "double_bar",
                                        },
                                        is_raw_ident: false,
                                    },
                                    LexedSubmodule {
                                        library_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe08f6cb6e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                ),
                                                start: 8,
                                                end: 18,
                                                as_str(): "double_bar",
                                            },
                                            is_raw_ident: false,
                                        },
                                        module: LexedModule {
                                            tree: Module {
                                                kind: Library {
                                                    library_token: LibraryToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f6cb6e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                            ),
                                                            start: 0,
                                                            end: 7,
                                                            as_str(): "library",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe08f6cb6e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                            ),
                                                            start: 8,
                                                            end: 18,
                                                            as_str(): "double_bar",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                },
                                                semicolon_token: SemicolonToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                        ),
                                                        start: 18,
                                                        end: 19,
                                                        as_str(): ";",
                                                    },
                                                },
                                                items: [
                                                    Annotated {
                                                        attribute_list: [],
                                                        value: Struct(
                                                            ItemStruct {
                                                                visibility: Some(
                                                                    PubToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f6cb6e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                            ),
                                                                            start: 21,
                                                                            end: 24,
                                                                            as_str(): "pub",
                                                                        },
                                                                    },
                                                                ),
                                                                struct_token: StructToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 25,
                                                                        end: 31,
                                                                        as_str(): "struct",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 32,
                                                                        end: 42,
                                                                        as_str(): "DoubleBar1",
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
                                                                                                src (ptr): 0x00007fe08f6cb6e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                ),
                                                                                                start: 49,
                                                                                                end: 50,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        colon_token: ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f6cb6e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                ),
                                                                                                start: 50,
                                                                                                end: 51,
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
                                                                                                            src (ptr): 0x00007fe08f6cb6e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                            ),
                                                                                                            start: 52,
                                                                                                            end: 55,
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
                                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                        ),
                                                                                        start: 55,
                                                                                        end: 56,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        final_value_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 43,
                                                                        end: 58,
                                                                        as_str(): "{\n    a: u32,\n}",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    Annotated {
                                                        attribute_list: [],
                                                        value: Struct(
                                                            ItemStruct {
                                                                visibility: Some(
                                                                    PubToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f6cb6e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                            ),
                                                                            start: 60,
                                                                            end: 63,
                                                                            as_str(): "pub",
                                                                        },
                                                                    },
                                                                ),
                                                                struct_token: StructToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 64,
                                                                        end: 70,
                                                                        as_str(): "struct",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 71,
                                                                        end: 81,
                                                                        as_str(): "DoubleBar2",
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
                                                                                                src (ptr): 0x00007fe08f6cb6e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                ),
                                                                                                start: 88,
                                                                                                end: 89,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        colon_token: ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f6cb6e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                ),
                                                                                                start: 89,
                                                                                                end: 90,
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
                                                                                                            src (ptr): 0x00007fe08f6cb6e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                            ),
                                                                                                            start: 91,
                                                                                                            end: 94,
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
                                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                        ),
                                                                                        start: 94,
                                                                                        end: 95,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        final_value_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 82,
                                                                        end: 97,
                                                                        as_str(): "{\n    a: u64,\n}",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    Annotated {
                                                        attribute_list: [],
                                                        value: Struct(
                                                            ItemStruct {
                                                                visibility: Some(
                                                                    PubToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe08f6cb6e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                            ),
                                                                            start: 99,
                                                                            end: 102,
                                                                            as_str(): "pub",
                                                                        },
                                                                    },
                                                                ),
                                                                struct_token: StructToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 103,
                                                                        end: 109,
                                                                        as_str(): "struct",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 110,
                                                                        end: 120,
                                                                        as_str(): "DoubleBar3",
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
                                                                                                src (ptr): 0x00007fe08f6cb6e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                ),
                                                                                                start: 127,
                                                                                                end: 128,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        colon_token: ColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe08f6cb6e0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                ),
                                                                                                start: 128,
                                                                                                end: 129,
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
                                                                                                            src (ptr): 0x00007fe08f6cb6e0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                                            ),
                                                                                                            start: 130,
                                                                                                            end: 133,
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
                                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                                        ),
                                                                                        start: 133,
                                                                                        end: 134,
                                                                                        as_str(): ",",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        final_value_opt: None,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08f6cb6e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/inner/double_bar.sw",
                                                                        ),
                                                                        start: 121,
                                                                        end: 136,
                                                                        as_str(): "{\n    a: u64,\n}",
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
                                ),
                            ],
                        },
                    },
                ),
            ],
        },
    },
)
