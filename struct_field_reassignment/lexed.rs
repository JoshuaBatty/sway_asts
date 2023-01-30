Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe056c4b800,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe056c4b800,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
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
                                            src (ptr): 0x00007fe056c4b800,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                            ),
                                            start: 54,
                                            end: 56,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe056c4b800,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                            ),
                                            start: 57,
                                            end: 61,
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
                                            src (ptr): 0x00007fe056c4b800,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                            ),
                                            start: 61,
                                            end: 63,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe056c4b800,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                    ),
                                                    start: 64,
                                                    end: 66,
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
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 67,
                                                                end: 70,
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
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 77,
                                                            end: 80,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 81,
                                                                    end: 84,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 85,
                                                                end: 89,
                                                                as_str(): "data",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 90,
                                                            end: 91,
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
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 92,
                                                                        end: 96,
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
                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 107,
                                                                                    end: 112,
                                                                                    as_str(): "value",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056c4b800,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 112,
                                                                                            end: 113,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    FuncApp {
                                                                                        func: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe056c4b800,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 114,
                                                                                                            end: 128,
                                                                                                            as_str(): "NumberOrString",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                                suffix: [
                                                                                                    (
                                                                                                        DoubleColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 128,
                                                                                                                end: 130,
                                                                                                                as_str(): "::",
                                                                                                            },
                                                                                                        },
                                                                                                        PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 130,
                                                                                                                    end: 136,
                                                                                                                    as_str(): "Number",
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
                                                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 137,
                                                                                                                    end: 139,
                                                                                                                    as_str(): "20",
                                                                                                                },
                                                                                                                parsed: 20,
                                                                                                                ty_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 136,
                                                                                                end: 140,
                                                                                                as_str(): "(20)",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 140,
                                                                                end: 141,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 150,
                                                                                    end: 157,
                                                                                    as_str(): "address",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056c4b800,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 159,
                                                                                                    end: 169,
                                                                                                    as_str(): "0b00001111",
                                                                                                },
                                                                                                parsed: 15,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 169,
                                                                                end: 170,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 97,
                                                                end: 176,
                                                                as_str(): "{\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 176,
                                                            end: 177,
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
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 183,
                                                                    end: 187,
                                                                    as_str(): "data",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        dot_token: DotToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 187,
                                                                end: 188,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 188,
                                                                end: 193,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 195,
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
                                                                            src (ptr): 0x00007fe056c4b800,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 196,
                                                                            end: 210,
                                                                            as_str(): "NumberOrString",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 210,
                                                                                end: 212,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 212,
                                                                                    end: 218,
                                                                                    as_str(): "String",
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
                                                                        String(
                                                                            LitString {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056c4b800,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 220,
                                                                                    end: 226,
                                                                                    as_str(): "\"sway\"",
                                                                                },
                                                                                parsed: "sway",
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 218,
                                                                end: 227,
                                                                as_str(): "( \"sway\")",
                                                            },
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 227,
                                                            end: 228,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 233,
                                                            end: 239,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056c4b800,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 240,
                                                                        end: 241,
                                                                        as_str(): "0",
                                                                    },
                                                                    parsed: 0,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe056c4b800,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                            ),
                                                            start: 241,
                                                            end: 242,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 71,
                                        end: 244,
                                        as_str(): "{\n    let mut data = Data {\n        value: NumberOrString::Number(20),\n        address: 0b00001111,\n    };\n\n    data.value = NumberOrString::String( \"sway\");\n    return 0;\n}",
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
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 246,
                                        end: 250,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 251,
                                        end: 265,
                                        as_str(): "NumberOrString",
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
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 272,
                                                                end: 278,
                                                                as_str(): "Number",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 278,
                                                                end: 279,
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
                                                                            src (ptr): 0x00007fe056c4b800,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 280,
                                                                            end: 283,
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
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 283,
                                                        end: 284,
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
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 289,
                                                                end: 295,
                                                                as_str(): "String",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 295,
                                                                end: 296,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 297,
                                                                    end: 300,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056c4b800,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 301,
                                                                                end: 302,
                                                                                as_str(): "4",
                                                                            },
                                                                            parsed: 4,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056c4b800,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 300,
                                                                    end: 303,
                                                                    as_str(): "[4]",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 303,
                                                        end: 304,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 266,
                                        end: 306,
                                        as_str(): "{\n    Number: u64,\n    String: str[4],\n}",
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
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 308,
                                        end: 314,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 315,
                                        end: 319,
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
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 326,
                                                                end: 331,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 331,
                                                                end: 332,
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
                                                                            src (ptr): 0x00007fe056c4b800,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 333,
                                                                            end: 347,
                                                                            as_str(): "NumberOrString",
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
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                        ),
                                                        start: 347,
                                                        end: 348,
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
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                ),
                                                                start: 353,
                                                                end: 360,
                                                                as_str(): "address",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe056c4b800,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
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
                                                                            src (ptr): 0x00007fe056c4b800,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 362,
                                                                            end: 364,
                                                                            as_str(): "u8",
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
                                                        src (ptr): 0x00007fe056c4b800,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
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
                                        src (ptr): 0x00007fe056c4b800,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYeWiqs/struct_field_reassignment/src/main.sw",
                                        ),
                                        start: 320,
                                        end: 367,
                                        as_str(): "{\n    value: NumberOrString,\n    address: u8,\n}",
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
