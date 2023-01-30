Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007f8a0e6187a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007f8a0e6187a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
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
                                        src (ptr): 0x00007f8a0e6187a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a0e6187a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 34,
                                        as_str(): "StructWithNoFields",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a0e6187a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                        ),
                                        start: 35,
                                        end: 37,
                                        as_str(): "{}",
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
                                        src (ptr): 0x00007f8a0e6187a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                        ),
                                        start: 38,
                                        end: 42,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a0e6187a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 61,
                                        as_str(): "EnumWithNoVariants",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a0e6187a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                        ),
                                        start: 62,
                                        end: 64,
                                        as_str(): "{}",
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
                                            src (ptr): 0x00007f8a0e6187a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 68,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a0e6187a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                            ),
                                            start: 69,
                                            end: 73,
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
                                            src (ptr): 0x00007f8a0e6187a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                            ),
                                            start: 73,
                                            end: 75,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007f8a0e6187a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                    ),
                                                    start: 76,
                                                    end: 78,
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
                                                                src (ptr): 0x00007f8a0e6187a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                                ),
                                                                start: 79,
                                                                end: 82,
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
                                                            src (ptr): 0x00007f8a0e6187a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                            ),
                                                            start: 89,
                                                            end: 92,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6187a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                                ),
                                                                start: 93,
                                                                end: 104,
                                                                as_str(): "unit_struct",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6187a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                            ),
                                                            start: 105,
                                                            end: 106,
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
                                                                        src (ptr): 0x00007f8a0e6187a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                                        ),
                                                                        start: 107,
                                                                        end: 125,
                                                                        as_str(): "StructWithNoFields",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007f8a0e6187a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                                ),
                                                                start: 126,
                                                                end: 128,
                                                                as_str(): "{}",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6187a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                            ),
                                                            start: 128,
                                                            end: 129,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007f8a0e6187a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                            ),
                                                            start: 134,
                                                            end: 136,
                                                            as_str(): "10",
                                                        },
                                                        parsed: 10,
                                                        ty_opt: Some(
                                                            (
                                                                U64,
                                                                Span {
                                                                    src (ptr): 0x00007f8a0e6187a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                                                    ),
                                                                    start: 136,
                                                                    end: 139,
                                                                    as_str(): "u64",
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007f8a0e6187a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR5LtYmJ/zero_field_types/src/main.sw",
                                        ),
                                        start: 83,
                                        end: 141,
                                        as_str(): "{\n    let unit_struct = StructWithNoFields {};\n    10u64\n}",
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
