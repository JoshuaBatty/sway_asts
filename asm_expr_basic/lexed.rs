Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb1355c1840,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb1355c1840,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
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
                                        src (ptr): 0x00007fb1355c1840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
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
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1355c1840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 24,
                                                as_str(): "assert",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb1355c1840,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                ),
                                                start: 24,
                                                end: 26,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 26,
                                                    end: 32,
                                                    as_str(): "assert",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb1355c1840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                        ),
                                        start: 32,
                                        end: 33,
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
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 102,
                                            end: 104,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 105,
                                            end: 116,
                                            as_str(): "blockheight",
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
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 116,
                                            end: 118,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 121,
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 125,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            Asm(
                                                AsmBlock {
                                                    asm_token: AsmToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 132,
                                                            end: 135,
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 136,
                                                                            end: 138,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    value_opt: None,
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 139,
                                                            as_str(): "(r1)",
                                                        },
                                                    },
                                                    contents: Braces {
                                                        inner: AsmBlockContents {
                                                            instructions: [
                                                                (
                                                                    Bhei {
                                                                        token: BheiOpcode {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1355c1840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                ),
                                                                                start: 150,
                                                                                end: 154,
                                                                                as_str(): "bhei",
                                                                            },
                                                                        },
                                                                        ret: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1355c1840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                ),
                                                                                start: 155,
                                                                                end: 157,
                                                                                as_str(): "r1",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 157,
                                                                            end: 158,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_expr_opt: Some(
                                                                AsmFinalExpr {
                                                                    register: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 167,
                                                                            end: 169,
                                                                            as_str(): "r1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    ty_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 169,
                                                                                    end: 170,
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
                                                                                                src (ptr): 0x00007fb1355c1840,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                                ),
                                                                                                start: 171,
                                                                                                end: 174,
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
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 140,
                                                            end: 180,
                                                            as_str(): "{\n        bhei r1;\n        r1: u64\n    }",
                                                        },
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1355c1840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                        ),
                                        start: 126,
                                        end: 182,
                                        as_str(): "{\n    asm(r1) {\n        bhei r1;\n        r1: u64\n    }\n}",
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
                                        src (ptr): 0x00007fb1355c1840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                        ),
                                        start: 184,
                                        end: 190,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1355c1840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                        ),
                                        start: 191,
                                        end: 200,
                                        as_str(): "GasCounts",
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 207,
                                                                end: 217,
                                                                as_str(): "global_gas",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 217,
                                                                end: 218,
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 219,
                                                                            end: 222,
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
                                                        src (ptr): 0x00007fb1355c1840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                        ),
                                                        start: 222,
                                                        end: 223,
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 228,
                                                                end: 239,
                                                                as_str(): "context_gas",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 239,
                                                                end: 240,
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 241,
                                                                            end: 244,
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
                                                        src (ptr): 0x00007fb1355c1840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                        ),
                                                        start: 244,
                                                        end: 245,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1355c1840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                        ),
                                        start: 201,
                                        end: 247,
                                        as_str(): "{\n    global_gas: u64,\n    context_gas: u64,\n}",
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
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 249,
                                            end: 251,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 252,
                                            end: 259,
                                            as_str(): "get_gas",
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
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 259,
                                            end: 261,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 262,
                                                    end: 264,
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 265,
                                                                end: 274,
                                                                as_str(): "GasCounts",
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 281,
                                                                end: 290,
                                                                as_str(): "GasCounts",
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 301,
                                                                            end: 311,
                                                                            as_str(): "global_gas",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    expr_opt: Some(
                                                                        (
                                                                            ColonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 311,
                                                                                    end: 312,
                                                                                    as_str(): ":",
                                                                                },
                                                                            },
                                                                            Asm(
                                                                                AsmBlock {
                                                                                    asm_token: AsmToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1355c1840,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                            ),
                                                                                            start: 313,
                                                                                            end: 316,
                                                                                            as_str(): "asm",
                                                                                        },
                                                                                    },
                                                                                    registers: Parens {
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [],
                                                                                            final_value_opt: None,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1355c1840,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                            ),
                                                                                            start: 316,
                                                                                            end: 318,
                                                                                            as_str(): "()",
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
                                                                                                            src (ptr): 0x00007fb1355c1840,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                                            ),
                                                                                                            start: 333,
                                                                                                            end: 337,
                                                                                                            as_str(): "ggas",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1355c1840,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                            ),
                                                                                            start: 319,
                                                                                            end: 347,
                                                                                            as_str(): "{\n            ggas\n        }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 347,
                                                                        end: 348,
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
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 357,
                                                                        end: 368,
                                                                        as_str(): "context_gas",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                expr_opt: Some(
                                                                    (
                                                                        ColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1355c1840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                ),
                                                                                start: 368,
                                                                                end: 369,
                                                                                as_str(): ":",
                                                                            },
                                                                        },
                                                                        Asm(
                                                                            AsmBlock {
                                                                                asm_token: AsmToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 370,
                                                                                        end: 373,
                                                                                        as_str(): "asm",
                                                                                    },
                                                                                },
                                                                                registers: Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: None,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 373,
                                                                                        end: 375,
                                                                                        as_str(): "()",
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
                                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                                        ),
                                                                                                        start: 390,
                                                                                                        end: 394,
                                                                                                        as_str(): "cgas",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 376,
                                                                                        end: 404,
                                                                                        as_str(): "{\n            cgas\n        }",
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb1355c1840,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                        ),
                                                        start: 291,
                                                        end: 410,
                                                        as_str(): "{\n        global_gas: asm() {\n            ggas\n        },\n        context_gas: asm() {\n            cgas\n        }\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1355c1840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                        ),
                                        start: 275,
                                        end: 412,
                                        as_str(): "{\n    GasCounts {\n        global_gas: asm() {\n            ggas\n        },\n        context_gas: asm() {\n            cgas\n        }\n    }\n}",
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
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 414,
                                            end: 416,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 417,
                                            end: 421,
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
                                            src (ptr): 0x00007fb1355c1840,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                            ),
                                            start: 421,
                                            end: 423,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb1355c1840,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                    ),
                                                    start: 424,
                                                    end: 426,
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 427,
                                                                end: 430,
                                                                as_str(): "u32",
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
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 437,
                                                            end: 440,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 441,
                                                                end: 453,
                                                                as_str(): "block_height",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 454,
                                                            end: 455,
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 456,
                                                                            end: 467,
                                                                            as_str(): "blockheight",
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 467,
                                                                end: 469,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 469,
                                                            end: 470,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 475,
                                                            end: 478,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 479,
                                                                end: 492,
                                                                as_str(): "remaining_gas",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 493,
                                                            end: 494,
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 495,
                                                                            end: 502,
                                                                            as_str(): "get_gas",
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
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 502,
                                                                end: 504,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 504,
                                                            end: 505,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 561,
                                                            end: 564,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 565,
                                                                end: 569,
                                                                as_str(): "zero",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 570,
                                                            end: 571,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 572,
                                                                    end: 575,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 575,
                                                                    end: 577,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 580,
                                                                                    end: 584,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 578,
                                                                    end: 586,
                                                                    as_str(): "{ zero }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 586,
                                                            end: 587,
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
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 592,
                                                                        end: 598,
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
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 599,
                                                                                        end: 603,
                                                                                        as_str(): "zero",
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 604,
                                                                            end: 606,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 607,
                                                                                    end: 608,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 598,
                                                            end: 609,
                                                            as_str(): "(zero == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 609,
                                                            end: 610,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 616,
                                                            end: 619,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 620,
                                                                end: 623,
                                                                as_str(): "one",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 624,
                                                            end: 625,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 626,
                                                                    end: 629,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 629,
                                                                    end: 631,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 634,
                                                                                    end: 637,
                                                                                    as_str(): "one",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 632,
                                                                    end: 639,
                                                                    as_str(): "{ one }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 639,
                                                            end: 640,
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
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 645,
                                                                        end: 651,
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
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 652,
                                                                                        end: 655,
                                                                                        as_str(): "one",
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 656,
                                                                            end: 658,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 659,
                                                                                    end: 660,
                                                                                    as_str(): "1",
                                                                                },
                                                                                parsed: 1,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 651,
                                                            end: 661,
                                                            as_str(): "(one == 1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 661,
                                                            end: 662,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 668,
                                                            end: 671,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 672,
                                                                end: 674,
                                                                as_str(): "of",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 675,
                                                            end: 676,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 677,
                                                                    end: 680,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 680,
                                                                    end: 682,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 685,
                                                                                    end: 687,
                                                                                    as_str(): "of",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 683,
                                                                    end: 689,
                                                                    as_str(): "{ of }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 689,
                                                            end: 690,
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
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 695,
                                                                        end: 701,
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
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 702,
                                                                                        end: 704,
                                                                                        as_str(): "of",
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 705,
                                                                            end: 707,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 708,
                                                                                    end: 709,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 701,
                                                            end: 710,
                                                            as_str(): "(of == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 710,
                                                            end: 711,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 717,
                                                            end: 720,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 721,
                                                                end: 723,
                                                                as_str(): "pc",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 724,
                                                            end: 725,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 726,
                                                                    end: 729,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 729,
                                                                    end: 731,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 734,
                                                                                    end: 736,
                                                                                    as_str(): "pc",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 732,
                                                                    end: 738,
                                                                    as_str(): "{ pc }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 738,
                                                            end: 739,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 745,
                                                            end: 748,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 749,
                                                                end: 752,
                                                                as_str(): "ssp",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 753,
                                                            end: 754,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 755,
                                                                    end: 758,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 758,
                                                                    end: 760,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 763,
                                                                                    end: 766,
                                                                                    as_str(): "ssp",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 761,
                                                                    end: 768,
                                                                    as_str(): "{ ssp }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 768,
                                                            end: 769,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 775,
                                                            end: 778,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 779,
                                                                end: 781,
                                                                as_str(): "sp",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 782,
                                                            end: 783,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 784,
                                                                    end: 787,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 787,
                                                                    end: 789,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 792,
                                                                                    end: 794,
                                                                                    as_str(): "sp",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 790,
                                                                    end: 796,
                                                                    as_str(): "{ sp }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 796,
                                                            end: 797,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 803,
                                                            end: 806,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 807,
                                                                end: 809,
                                                                as_str(): "fp",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 810,
                                                            end: 811,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 812,
                                                                    end: 815,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 815,
                                                                    end: 817,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 820,
                                                                                    end: 822,
                                                                                    as_str(): "fp",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 818,
                                                                    end: 824,
                                                                    as_str(): "{ fp }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 824,
                                                            end: 825,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 831,
                                                            end: 834,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 835,
                                                                end: 837,
                                                                as_str(): "hp",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 838,
                                                            end: 839,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 840,
                                                                    end: 843,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 843,
                                                                    end: 845,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 848,
                                                                                    end: 850,
                                                                                    as_str(): "hp",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 846,
                                                                    end: 852,
                                                                    as_str(): "{ hp }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 852,
                                                            end: 853,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 859,
                                                            end: 862,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 863,
                                                                end: 866,
                                                                as_str(): "err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 867,
                                                            end: 868,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 869,
                                                                    end: 872,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 872,
                                                                    end: 874,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 877,
                                                                                    end: 880,
                                                                                    as_str(): "err",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 875,
                                                                    end: 882,
                                                                    as_str(): "{ err }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 882,
                                                            end: 883,
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
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 888,
                                                                        end: 894,
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
                                                                                        src (ptr): 0x00007fb1355c1840,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                        ),
                                                                                        start: 895,
                                                                                        end: 898,
                                                                                        as_str(): "err",
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
                                                                            src (ptr): 0x00007fb1355c1840,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                            ),
                                                                            start: 899,
                                                                            end: 901,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 902,
                                                                                    end: 903,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 894,
                                                            end: 904,
                                                            as_str(): "(err == 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 904,
                                                            end: 905,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 911,
                                                            end: 914,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 915,
                                                                end: 919,
                                                                as_str(): "ggas",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 920,
                                                            end: 921,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 922,
                                                                    end: 925,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 925,
                                                                    end: 927,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 930,
                                                                                    end: 934,
                                                                                    as_str(): "ggas",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 928,
                                                                    end: 936,
                                                                    as_str(): "{ ggas }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 936,
                                                            end: 937,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 943,
                                                            end: 946,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 947,
                                                                end: 951,
                                                                as_str(): "cgas",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 952,
                                                            end: 953,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 954,
                                                                    end: 957,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 957,
                                                                    end: 959,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 962,
                                                                                    end: 966,
                                                                                    as_str(): "cgas",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 960,
                                                                    end: 968,
                                                                    as_str(): "{ cgas }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 968,
                                                            end: 969,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 975,
                                                            end: 978,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 979,
                                                                end: 982,
                                                                as_str(): "bal",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 983,
                                                            end: 984,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 985,
                                                                    end: 988,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 988,
                                                                    end: 990,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 993,
                                                                                    end: 996,
                                                                                    as_str(): "bal",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 991,
                                                                    end: 998,
                                                                    as_str(): "{ bal }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 998,
                                                            end: 999,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1005,
                                                            end: 1008,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 1009,
                                                                end: 1011,
                                                                as_str(): "is",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1012,
                                                            end: 1013,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1014,
                                                                    end: 1017,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1017,
                                                                    end: 1019,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 1022,
                                                                                    end: 1024,
                                                                                    as_str(): "is",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1020,
                                                                    end: 1026,
                                                                    as_str(): "{ is }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1026,
                                                            end: 1027,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1033,
                                                            end: 1036,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 1037,
                                                                end: 1040,
                                                                as_str(): "ret",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1041,
                                                            end: 1042,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1043,
                                                                    end: 1046,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1046,
                                                                    end: 1048,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 1051,
                                                                                    end: 1054,
                                                                                    as_str(): "ret",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1049,
                                                                    end: 1056,
                                                                    as_str(): "{ ret }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1056,
                                                            end: 1057,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1063,
                                                            end: 1066,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 1067,
                                                                end: 1071,
                                                                as_str(): "retl",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1072,
                                                            end: 1073,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1074,
                                                                    end: 1077,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1077,
                                                                    end: 1079,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 1082,
                                                                                    end: 1086,
                                                                                    as_str(): "retl",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1080,
                                                                    end: 1088,
                                                                    as_str(): "{ retl }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1088,
                                                            end: 1089,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1095,
                                                            end: 1098,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1355c1840,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                ),
                                                                start: 1099,
                                                                end: 1103,
                                                                as_str(): "flag",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1104,
                                                            end: 1105,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Asm(
                                                        AsmBlock {
                                                            asm_token: AsmToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1106,
                                                                    end: 1109,
                                                                    as_str(): "asm",
                                                                },
                                                            },
                                                            registers: Parens {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1109,
                                                                    end: 1111,
                                                                    as_str(): "()",
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
                                                                                    src (ptr): 0x00007fb1355c1840,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                    ),
                                                                                    start: 1114,
                                                                                    end: 1118,
                                                                                    as_str(): "flag",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1355c1840,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                    ),
                                                                    start: 1112,
                                                                    end: 1120,
                                                                    as_str(): "{ flag }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1120,
                                                            end: 1121,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1127,
                                                            end: 1133,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1355c1840,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                        ),
                                                                        start: 1134,
                                                                        end: 1135,
                                                                        as_str(): "6",
                                                                    },
                                                                    parsed: 6,
                                                                    ty_opt: Some(
                                                                        (
                                                                            U32,
                                                                            Span {
                                                                                src (ptr): 0x00007fb1355c1840,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                                                ),
                                                                                start: 1135,
                                                                                end: 1138,
                                                                                as_str(): "u32",
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb1355c1840,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                                            ),
                                                            start: 1138,
                                                            end: 1139,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb1355c1840,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbLQBtI/asm_expr_basic/src/main.sw",
                                        ),
                                        start: 431,
                                        end: 1141,
                                        as_str(): "{\n    let block_height = blockheight();\n    let remaining_gas = get_gas();\n\n    // Test the spelling of all special registers\n    let zero = asm() { zero };\n    assert(zero == 0);\n\n    let one = asm() { one };\n    assert(one == 1);\n\n    let of = asm() { of };\n    assert(of == 0);\n\n    let pc = asm() { pc };\n\n    let ssp = asm() { ssp };\n\n    let sp = asm() { sp };\n\n    let fp = asm() { fp };\n\n    let hp = asm() { hp };\n\n    let err = asm() { err };\n    assert(err == 0);\n\n    let ggas = asm() { ggas };\n\n    let cgas = asm() { cgas };\n\n    let bal = asm() { bal };\n\n    let is = asm() { is };\n\n    let ret = asm() { ret };\n\n    let retl = asm() { retl };\n\n    let flag = asm() { flag };\n\n    return 6u32;\n}",
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
