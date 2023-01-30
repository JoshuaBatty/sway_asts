[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 41,
                            end: 45,
                            as_str(): "Foo1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 52,
                                    end: 53,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 52,
                                end: 58,
                                as_str(): "a: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 55,
                                end: 58,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 64,
                                    end: 65,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 64,
                                end: 70,
                                as_str(): "b: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 67,
                                end: 70,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 34,
                        end: 73,
                        as_str(): "struct Foo1 {\n    a: u64,\n    b: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 34,
            end: 73,
            as_str(): "struct Foo1 {\n    a: u64,\n    b: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 80,
                                end: 84,
                                as_str(): "Foo1",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 80,
                        end: 84,
                        as_str(): "Foo1",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 94,
                                    end: 101,
                                    as_str(): "trivial",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 124,
                                                    end: 129,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 124,
                                            end: 129,
                                            as_str(): "false",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 116,
                                    end: 135,
                                    as_str(): "{\n      false\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 102,
                                            end: 106,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 102,
                                        end: 106,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 91,
                                end: 135,
                                as_str(): "fn trivial(self) -> bool {\n      false\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 111,
                                end: 115,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 75,
                        end: 137,
                        as_str(): "impl Foo1 {\n    fn trivial(self) -> bool {\n      false\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 75,
            end: 137,
            as_str(): "impl Foo1 {\n    fn trivial(self) -> bool {\n      false\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 142,
                            end: 147,
                            as_str(): "func1",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 168,
                                                    end: 169,
                                                    as_str(): "f",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 176,
                                                                        as_str(): "Foo1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 172,
                                                                end: 176,
                                                                as_str(): "Foo1",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 178,
                                                                        end: 179,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 181,
                                                                        end: 182,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 178,
                                                                    end: 182,
                                                                    as_str(): "a: 0",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 184,
                                                                        end: 185,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            0,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 187,
                                                                        end: 188,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 184,
                                                                    end: 188,
                                                                    as_str(): "b: 0",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 172,
                                                    end: 189,
                                                    as_str(): "Foo1 {a: 0, b: 0}",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 164,
                                    end: 190,
                                    as_str(): "let f = Foo1 {a: 0, b: 0};",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 197,
                                                                end: 204,
                                                                as_str(): "trivial",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 197,
                                                        end: 204,
                                                        as_str(): "trivial",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 195,
                                                                    end: 196,
                                                                    as_str(): "f",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 195,
                                                            end: 196,
                                                            as_str(): "f",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 195,
                                            end: 206,
                                            as_str(): "f.trivial()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 195,
                                    end: 206,
                                    as_str(): "f.trivial()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 158,
                            end: 208,
                            as_str(): "{\n    let f = Foo1 {a: 0, b: 0};\n    f.trivial()\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 139,
                        end: 208,
                        as_str(): "fn func1() -> bool {\n    let f = Foo1 {a: 0, b: 0};\n    f.trivial()\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 153,
                        end: 157,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 139,
            end: 208,
            as_str(): "fn func1() -> bool {\n    let f = Foo1 {a: 0, b: 0};\n    f.trivial()\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 240,
                            end: 243,
                            as_str(): "Bar",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 250,
                                    end: 251,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 253,
                                end: 255,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 250,
                                end: 255,
                                as_str(): "a: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 261,
                                    end: 262,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 264,
                                end: 266,
                                as_str(): "()",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 261,
                                end: 266,
                                as_str(): "b: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 235,
                        end: 269,
                        as_str(): "enum Bar {\n    a: (),\n    b: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 235,
            end: 269,
            as_str(): "enum Bar {\n    a: (),\n    b: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 276,
                                end: 279,
                                as_str(): "Bar",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 276,
                        end: 279,
                        as_str(): "Bar",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 289,
                                    end: 296,
                                    as_str(): "trivial",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 321,
                                                    end: 326,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 321,
                                            end: 326,
                                            as_str(): "false",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 311,
                                    end: 332,
                                    as_str(): "{\n        false\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 297,
                                            end: 301,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 297,
                                        end: 301,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 286,
                                end: 332,
                                as_str(): "fn trivial(self) -> bool {\n        false\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 306,
                                end: 310,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 271,
                        end: 334,
                        as_str(): "impl Bar {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 271,
            end: 334,
            as_str(): "impl Bar {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 339,
                            end: 344,
                            as_str(): "func2",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 369,
                                                                end: 376,
                                                                as_str(): "trivial",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 369,
                                                        end: 376,
                                                        as_str(): "trivial",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 367,
                                                                    end: 368,
                                                                    as_str(): "m",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 367,
                                                            end: 368,
                                                            as_str(): "m",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 367,
                                            end: 378,
                                            as_str(): "m.trivial()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 367,
                                    end: 378,
                                    as_str(): "m.trivial()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 361,
                            end: 380,
                            as_str(): "{\n    m.trivial()\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 345,
                                    end: 346,
                                    as_str(): "m",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fe0fc01dd50,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 348,
                                        end: 351,
                                        as_str(): "Bar",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 348,
                                end: 351,
                                as_str(): "Bar",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 336,
                        end: 380,
                        as_str(): "fn func2(m: Bar) -> bool {\n    m.trivial()\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 356,
                        end: 360,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 336,
            end: 380,
            as_str(): "fn func2(m: Bar) -> bool {\n    m.trivial()\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 414,
                            end: 418,
                            as_str(): "Foo2",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 428,
                                    end: 431,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 433,
                                        end: 434,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 428,
                                end: 434,
                                as_str(): "foo: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 433,
                                end: 434,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(7249),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 407,
                        end: 437,
                        as_str(): "struct Foo2<T> {\n    foo: T,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 407,
            end: 437,
            as_str(): "struct Foo2<T> {\n    foo: T,\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [
                        T: TypeId(7251),
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 447,
                                end: 451,
                                as_str(): "Foo2",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7250,
                                    ),
                                    initial_type_id: TypeId(
                                        7250,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 452,
                                        end: 453,
                                        as_str(): "T",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 447,
                        end: 454,
                        as_str(): "Foo2<T>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 464,
                                    end: 471,
                                    as_str(): "trivial",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 496,
                                                    end: 501,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 496,
                                            end: 501,
                                            as_str(): "false",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 486,
                                    end: 507,
                                    as_str(): "{\n        false\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 472,
                                            end: 476,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 472,
                                        end: 476,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 461,
                                end: 507,
                                as_str(): "fn trivial(self) -> bool {\n        false\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 481,
                                end: 485,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 439,
                        end: 509,
                        as_str(): "impl<T> Foo2<T> {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 439,
            end: 509,
            as_str(): "impl<T> Foo2<T> {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 514,
                            end: 519,
                            as_str(): "func3",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: MethodApplication(
                                                        MethodApplicationExpression {
                                                            method_name_binding: TypeBinding {
                                                                inner: FromModule {
                                                                    method_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 558,
                                                                            end: 565,
                                                                            as_str(): "trivial",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 558,
                                                                    end: 565,
                                                                    as_str(): "trivial",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 556,
                                                                                end: 557,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 556,
                                                                        end: 557,
                                                                        as_str(): "a",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 556,
                                                        end: 567,
                                                        as_str(): "a.trivial()",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: ImplicitReturnExpression(
                                                                        Expression {
                                                                            kind: Struct(
                                                                                StructExpression {
                                                                                    call_path_binding: TypeBinding {
                                                                                        inner: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 578,
                                                                                                    end: 582,
                                                                                                    as_str(): "Foo2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 578,
                                                                                            end: 582,
                                                                                            as_str(): "Foo2",
                                                                                        },
                                                                                    },
                                                                                    fields: [
                                                                                        StructExpressionField {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 584,
                                                                                                    end: 587,
                                                                                                    as_str(): "foo",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            value: Expression {
                                                                                                kind: Literal(
                                                                                                    Boolean(
                                                                                                        false,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 589,
                                                                                                    end: 594,
                                                                                                    as_str(): "false",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 584,
                                                                                                end: 594,
                                                                                                as_str(): "foo: false",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 578,
                                                                                end: 595,
                                                                                as_str(): "Foo2 {foo: false}",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 578,
                                                                        end: 595,
                                                                        as_str(): "Foo2 {foo: false}",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 568,
                                                                end: 601,
                                                                as_str(): "{\n        Foo2 {foo: false}\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 568,
                                                        end: 601,
                                                        as_str(): "{\n        Foo2 {foo: false}\n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            Expression {
                                                                                kind: Struct(
                                                                                    StructExpression {
                                                                                        call_path_binding: TypeBinding {
                                                                                            inner: CallPath {
                                                                                                prefixes: [],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 617,
                                                                                                        end: 621,
                                                                                                        as_str(): "Foo2",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 617,
                                                                                                end: 621,
                                                                                                as_str(): "Foo2",
                                                                                            },
                                                                                        },
                                                                                        fields: [
                                                                                            StructExpressionField {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 623,
                                                                                                        end: 626,
                                                                                                        as_str(): "foo",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                value: Expression {
                                                                                                    kind: Literal(
                                                                                                        Boolean(
                                                                                                            true,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 628,
                                                                                                        end: 632,
                                                                                                        as_str(): "true",
                                                                                                    },
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 623,
                                                                                                    end: 632,
                                                                                                    as_str(): "foo: true",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 617,
                                                                                    end: 633,
                                                                                    as_str(): "Foo2 {foo: true}",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 617,
                                                                            end: 633,
                                                                            as_str(): "Foo2 {foo: true}",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 607,
                                                                    end: 639,
                                                                    as_str(): "{\n        Foo2 {foo: true}\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 607,
                                                            end: 639,
                                                            as_str(): "{\n        Foo2 {foo: true}\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 553,
                                            end: 639,
                                            as_str(): "if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 553,
                                    end: 639,
                                    as_str(): "if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 547,
                            end: 641,
                            as_str(): "{\n    if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 520,
                                    end: 521,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fe0fc01dd50,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 523,
                                        end: 527,
                                        as_str(): "Foo2",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                50,
                                            ),
                                            initial_type_id: TypeId(
                                                50,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 528,
                                                end: 530,
                                                as_str(): "u8",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 523,
                                end: 531,
                                as_str(): "Foo2<u8>",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 511,
                        end: 641,
                        as_str(): "fn func3(a: Foo2<u8>) -> Foo2<bool> {\n    if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 536,
                                end: 540,
                                as_str(): "Foo2",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        71,
                                    ),
                                    initial_type_id: TypeId(
                                        71,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 541,
                                        end: 545,
                                        as_str(): "bool",
                                    },
                                },
                            ],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 536,
                        end: 546,
                        as_str(): "Foo2<bool>",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 511,
            end: 641,
            as_str(): "fn func3(a: Foo2<u8>) -> Foo2<bool> {\n    if a.trivial() {\n        Foo2 {foo: false}\n    } else {\n        Foo2 {foo: true}\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 646,
                            end: 651,
                            as_str(): "func4",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: MethodApplication(
                                                        MethodApplicationExpression {
                                                            method_name_binding: TypeBinding {
                                                                inner: FromModule {
                                                                    method_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 690,
                                                                            end: 697,
                                                                            as_str(): "trivial",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 690,
                                                                    end: 697,
                                                                    as_str(): "trivial",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 688,
                                                                                end: 689,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 688,
                                                                        end: 689,
                                                                        as_str(): "b",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 688,
                                                        end: 699,
                                                        as_str(): "b.trivial()",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: ImplicitReturnExpression(
                                                                        Expression {
                                                                            kind: Struct(
                                                                                StructExpression {
                                                                                    call_path_binding: TypeBinding {
                                                                                        inner: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 710,
                                                                                                    end: 714,
                                                                                                    as_str(): "Foo2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 710,
                                                                                            end: 714,
                                                                                            as_str(): "Foo2",
                                                                                        },
                                                                                    },
                                                                                    fields: [
                                                                                        StructExpressionField {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 716,
                                                                                                    end: 719,
                                                                                                    as_str(): "foo",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            value: Expression {
                                                                                                kind: Literal(
                                                                                                    U8(
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 721,
                                                                                                    end: 724,
                                                                                                    as_str(): "0u8",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 716,
                                                                                                end: 724,
                                                                                                as_str(): "foo: 0u8",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 710,
                                                                                end: 725,
                                                                                as_str(): "Foo2 {foo: 0u8}",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 710,
                                                                        end: 725,
                                                                        as_str(): "Foo2 {foo: 0u8}",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 700,
                                                                end: 732,
                                                                as_str(): "{\n        Foo2 {foo: 0u8} \n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 700,
                                                        end: 732,
                                                        as_str(): "{\n        Foo2 {foo: 0u8} \n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            Expression {
                                                                                kind: Struct(
                                                                                    StructExpression {
                                                                                        call_path_binding: TypeBinding {
                                                                                            inner: CallPath {
                                                                                                prefixes: [],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 748,
                                                                                                        end: 752,
                                                                                                        as_str(): "Foo2",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 748,
                                                                                                end: 752,
                                                                                                as_str(): "Foo2",
                                                                                            },
                                                                                        },
                                                                                        fields: [
                                                                                            StructExpressionField {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 754,
                                                                                                        end: 757,
                                                                                                        as_str(): "foo",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                value: Expression {
                                                                                                    kind: Literal(
                                                                                                        U8(
                                                                                                            1,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 759,
                                                                                                        end: 762,
                                                                                                        as_str(): "1u8",
                                                                                                    },
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 754,
                                                                                                    end: 762,
                                                                                                    as_str(): "foo: 1u8",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 748,
                                                                                    end: 763,
                                                                                    as_str(): "Foo2 {foo: 1u8}",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 748,
                                                                            end: 763,
                                                                            as_str(): "Foo2 {foo: 1u8}",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 738,
                                                                    end: 769,
                                                                    as_str(): "{\n        Foo2 {foo: 1u8}\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 738,
                                                            end: 769,
                                                            as_str(): "{\n        Foo2 {foo: 1u8}\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 685,
                                            end: 769,
                                            as_str(): "if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 685,
                                    end: 769,
                                    as_str(): "if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 679,
                            end: 771,
                            as_str(): "{\n    if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 652,
                                    end: 653,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fe0fc01dd50,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 655,
                                        end: 659,
                                        as_str(): "Foo2",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                71,
                                            ),
                                            initial_type_id: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 660,
                                                end: 664,
                                                as_str(): "bool",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 655,
                                end: 665,
                                as_str(): "Foo2<bool>",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 643,
                        end: 771,
                        as_str(): "fn func4(b: Foo2<bool>) -> Foo2<u8> {\n    if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 670,
                                end: 674,
                                as_str(): "Foo2",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        50,
                                    ),
                                    initial_type_id: TypeId(
                                        50,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 675,
                                        end: 677,
                                        as_str(): "u8",
                                    },
                                },
                            ],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 670,
                        end: 678,
                        as_str(): "Foo2<u8>",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 643,
            end: 771,
            as_str(): "fn func4(b: Foo2<bool>) -> Foo2<u8> {\n    if b.trivial() {\n        Foo2 {foo: 0u8} \n    } else {\n        Foo2 {foo: 1u8}\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 807,
                            end: 813,
                            as_str(): "Rezult",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(7252),
                        E: TypeId(7253),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 826,
                                    end: 828,
                                    as_str(): "Ok",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 830,
                                        end: 831,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 830,
                                end: 831,
                                as_str(): "T",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 826,
                                end: 831,
                                as_str(): "Ok: T",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 837,
                                    end: 840,
                                    as_str(): "Err",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 842,
                                        end: 843,
                                        as_str(): "E",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 842,
                                end: 843,
                                as_str(): "E",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 837,
                                end: 843,
                                as_str(): "Err: E",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 798,
                        end: 846,
                        as_str(): "pub enum Rezult<T, E> {\n    Ok: T,\n    Err: E,\n}",
                    },
                    visibility: Public,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 798,
            end: 846,
            as_str(): "pub enum Rezult<T, E> {\n    Ok: T,\n    Err: E,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 857,
                            end: 866,
                            as_str(): "DumbError",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 873,
                                    end: 878,
                                    as_str(): "Error",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 880,
                                end: 882,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 873,
                                end: 882,
                                as_str(): "Error: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 848,
                        end: 885,
                        as_str(): "pub enum DumbError {\n    Error: (),\n}",
                    },
                    visibility: Public,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 848,
            end: 885,
            as_str(): "pub enum DumbError {\n    Error: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [
                        T: TypeId(7256),
                        E: TypeId(7257),
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 898,
                                end: 904,
                                as_str(): "Rezult",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        7254,
                                    ),
                                    initial_type_id: TypeId(
                                        7254,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 905,
                                        end: 906,
                                        as_str(): "T",
                                    },
                                },
                                TypeArgument {
                                    type_id: TypeId(
                                        7255,
                                    ),
                                    initial_type_id: TypeId(
                                        7255,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 908,
                                        end: 909,
                                        as_str(): "E",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 898,
                        end: 910,
                        as_str(): "Rezult<T, E>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 920,
                                    end: 927,
                                    as_str(): "trivial",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc38b640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                    ),
                                                    start: 952,
                                                    end: 957,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 952,
                                            end: 957,
                                            as_str(): "false",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 942,
                                    end: 963,
                                    as_str(): "{\n        false\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 928,
                                            end: 932,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 928,
                                        end: 932,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 917,
                                end: 963,
                                as_str(): "fn trivial(self) -> bool {\n        false\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 937,
                                end: 941,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 887,
                        end: 965,
                        as_str(): "impl<T, E> Rezult<T, E> {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 887,
            end: 965,
            as_str(): "impl<T, E> Rezult<T, E> {\n    fn trivial(self) -> bool {\n        false\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 974,
                            end: 979,
                            as_str(): "func5",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Public,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: MethodApplication(
                                                        MethodApplicationExpression {
                                                            method_name_binding: TypeBinding {
                                                                inner: FromModule {
                                                                    method_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1042,
                                                                            end: 1049,
                                                                            as_str(): "trivial",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1042,
                                                                    end: 1049,
                                                                    as_str(): "trivial",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1040,
                                                                                end: 1041,
                                                                                as_str(): "r",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1040,
                                                                        end: 1041,
                                                                        as_str(): "r",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 1040,
                                                        end: 1051,
                                                        as_str(): "r.trivial()",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: ImplicitReturnExpression(
                                                                        Expression {
                                                                            kind: AmbiguousPathExpression(
                                                                                AmbiguousPathExpression {
                                                                                    call_path_binding: TypeBinding {
                                                                                        inner: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: AmbiguousSuffix {
                                                                                                before: TypeBinding {
                                                                                                    inner: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1062,
                                                                                                            end: 1068,
                                                                                                            as_str(): "Rezult",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1062,
                                                                                                        end: 1068,
                                                                                                        as_str(): "Rezult",
                                                                                                    },
                                                                                                },
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1070,
                                                                                                        end: 1073,
                                                                                                        as_str(): "Err",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1062,
                                                                                            end: 1073,
                                                                                            as_str(): "Rezult::Err",
                                                                                        },
                                                                                    },
                                                                                    args: [
                                                                                        Expression {
                                                                                            kind: DelineatedPath(
                                                                                                DelineatedPathExpression {
                                                                                                    call_path_binding: TypeBinding {
                                                                                                        inner: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1074,
                                                                                                                        end: 1083,
                                                                                                                        as_str(): "DumbError",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1085,
                                                                                                                    end: 1090,
                                                                                                                    as_str(): "Error",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1085,
                                                                                                            end: 1090,
                                                                                                            as_str(): "Error",
                                                                                                        },
                                                                                                    },
                                                                                                    args: None,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 1074,
                                                                                                end: 1090,
                                                                                                as_str(): "DumbError::Error",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1062,
                                                                                end: 1091,
                                                                                as_str(): "Rezult::Err(DumbError::Error)",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1062,
                                                                        end: 1091,
                                                                        as_str(): "Rezult::Err(DumbError::Error)",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1052,
                                                                end: 1097,
                                                                as_str(): "{\n        Rezult::Err(DumbError::Error)\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 1052,
                                                        end: 1097,
                                                        as_str(): "{\n        Rezult::Err(DumbError::Error)\n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            Expression {
                                                                                kind: AmbiguousPathExpression(
                                                                                    AmbiguousPathExpression {
                                                                                        call_path_binding: TypeBinding {
                                                                                            inner: CallPath {
                                                                                                prefixes: [],
                                                                                                suffix: AmbiguousSuffix {
                                                                                                    before: TypeBinding {
                                                                                                        inner: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1113,
                                                                                                                end: 1119,
                                                                                                                as_str(): "Rezult",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1113,
                                                                                                            end: 1119,
                                                                                                            as_str(): "Rezult",
                                                                                                        },
                                                                                                    },
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1121,
                                                                                                            end: 1123,
                                                                                                            as_str(): "Ok",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 1113,
                                                                                                end: 1123,
                                                                                                as_str(): "Rezult::Ok",
                                                                                            },
                                                                                        },
                                                                                        args: [
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    U8(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1124,
                                                                                                    end: 1127,
                                                                                                    as_str(): "1u8",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 1113,
                                                                                    end: 1128,
                                                                                    as_str(): "Rezult::Ok(1u8)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1113,
                                                                            end: 1128,
                                                                            as_str(): "Rezult::Ok(1u8)",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1103,
                                                                    end: 1134,
                                                                    as_str(): "{\n        Rezult::Ok(1u8)\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1103,
                                                            end: 1134,
                                                            as_str(): "{\n        Rezult::Ok(1u8)\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1037,
                                            end: 1134,
                                            as_str(): "if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 1037,
                                    end: 1134,
                                    as_str(): "if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 1031,
                            end: 1136,
                            as_str(): "{\n    if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 980,
                                    end: 981,
                                    as_str(): "r",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fe0fc01dd50,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 983,
                                        end: 989,
                                        as_str(): "Rezult",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                50,
                                            ),
                                            initial_type_id: TypeId(
                                                50,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 990,
                                                end: 992,
                                                as_str(): "u8",
                                            },
                                        },
                                        TypeArgument {
                                            type_id: TypeId(
                                                7258,
                                            ),
                                            initial_type_id: TypeId(
                                                7258,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 994,
                                                end: 1003,
                                                as_str(): "DumbError",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 983,
                                end: 1004,
                                as_str(): "Rezult<u8, DumbError>",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 967,
                        end: 1136,
                        as_str(): "pub fn func5(r: Rezult<u8, DumbError>) -> Rezult<u8, DumbError> {\n    if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 1009,
                                end: 1015,
                                as_str(): "Rezult",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        50,
                                    ),
                                    initial_type_id: TypeId(
                                        50,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1016,
                                        end: 1018,
                                        as_str(): "u8",
                                    },
                                },
                                TypeArgument {
                                    type_id: TypeId(
                                        7259,
                                    ),
                                    initial_type_id: TypeId(
                                        7259,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1020,
                                        end: 1029,
                                        as_str(): "DumbError",
                                    },
                                },
                            ],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 1009,
                        end: 1030,
                        as_str(): "Rezult<u8, DumbError>",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 967,
            end: 1136,
            as_str(): "pub fn func5(r: Rezult<u8, DumbError>) -> Rezult<u8, DumbError> {\n    if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(1u8)\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 1145,
                            end: 1150,
                            as_str(): "func6",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Public,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: If(
                                            IfExpression {
                                                condition: Expression {
                                                    kind: MethodApplication(
                                                        MethodApplicationExpression {
                                                            method_name_binding: TypeBinding {
                                                                inner: FromModule {
                                                                    method_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1216,
                                                                            end: 1223,
                                                                            as_str(): "trivial",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1216,
                                                                    end: 1223,
                                                                    as_str(): "trivial",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1214,
                                                                                end: 1215,
                                                                                as_str(): "r",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1214,
                                                                        end: 1215,
                                                                        as_str(): "r",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 1214,
                                                        end: 1225,
                                                        as_str(): "r.trivial()",
                                                    },
                                                },
                                                then: Expression {
                                                    kind: CodeBlock(
                                                        CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: ImplicitReturnExpression(
                                                                        Expression {
                                                                            kind: AmbiguousPathExpression(
                                                                                AmbiguousPathExpression {
                                                                                    call_path_binding: TypeBinding {
                                                                                        inner: CallPath {
                                                                                            prefixes: [],
                                                                                            suffix: AmbiguousSuffix {
                                                                                                before: TypeBinding {
                                                                                                    inner: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1236,
                                                                                                            end: 1242,
                                                                                                            as_str(): "Rezult",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1236,
                                                                                                        end: 1242,
                                                                                                        as_str(): "Rezult",
                                                                                                    },
                                                                                                },
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1244,
                                                                                                        end: 1247,
                                                                                                        as_str(): "Err",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            },
                                                                                            is_absolute: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                            ),
                                                                                            start: 1236,
                                                                                            end: 1247,
                                                                                            as_str(): "Rezult::Err",
                                                                                        },
                                                                                    },
                                                                                    args: [
                                                                                        Expression {
                                                                                            kind: DelineatedPath(
                                                                                                DelineatedPathExpression {
                                                                                                    call_path_binding: TypeBinding {
                                                                                                        inner: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1248,
                                                                                                                        end: 1257,
                                                                                                                        as_str(): "DumbError",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1259,
                                                                                                                    end: 1264,
                                                                                                                    as_str(): "Error",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1259,
                                                                                                            end: 1264,
                                                                                                            as_str(): "Error",
                                                                                                        },
                                                                                                    },
                                                                                                    args: None,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 1248,
                                                                                                end: 1264,
                                                                                                as_str(): "DumbError::Error",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                ),
                                                                                start: 1236,
                                                                                end: 1265,
                                                                                as_str(): "Rezult::Err(DumbError::Error)",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc38b640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                        ),
                                                                        start: 1236,
                                                                        end: 1265,
                                                                        as_str(): "Rezult::Err(DumbError::Error)",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0fc38b640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                ),
                                                                start: 1226,
                                                                end: 1271,
                                                                as_str(): "{\n        Rezult::Err(DumbError::Error)\n    }",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc38b640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                        ),
                                                        start: 1226,
                                                        end: 1271,
                                                        as_str(): "{\n        Rezult::Err(DumbError::Error)\n    }",
                                                    },
                                                },
                                                else: Some(
                                                    Expression {
                                                        kind: CodeBlock(
                                                            CodeBlock {
                                                                contents: [
                                                                    AstNode {
                                                                        content: ImplicitReturnExpression(
                                                                            Expression {
                                                                                kind: AmbiguousPathExpression(
                                                                                    AmbiguousPathExpression {
                                                                                        call_path_binding: TypeBinding {
                                                                                            inner: CallPath {
                                                                                                prefixes: [],
                                                                                                suffix: AmbiguousSuffix {
                                                                                                    before: TypeBinding {
                                                                                                        inner: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1287,
                                                                                                                end: 1293,
                                                                                                                as_str(): "Rezult",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1287,
                                                                                                            end: 1293,
                                                                                                            as_str(): "Rezult",
                                                                                                        },
                                                                                                    },
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1295,
                                                                                                            end: 1297,
                                                                                                            as_str(): "Ok",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc38b640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                ),
                                                                                                start: 1287,
                                                                                                end: 1297,
                                                                                                as_str(): "Rezult::Ok",
                                                                                            },
                                                                                        },
                                                                                        args: [
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Boolean(
                                                                                                        true,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1298,
                                                                                                    end: 1302,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                                    ),
                                                                                    start: 1287,
                                                                                    end: 1303,
                                                                                    as_str(): "Rezult::Ok(true)",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc38b640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                            ),
                                                                            start: 1287,
                                                                            end: 1303,
                                                                            as_str(): "Rezult::Ok(true)",
                                                                        },
                                                                    },
                                                                ],
                                                                whole_block_span: Span {
                                                                    src (ptr): 0x00007fe0fc38b640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                                    ),
                                                                    start: 1277,
                                                                    end: 1309,
                                                                    as_str(): "{\n        Rezult::Ok(true)\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc38b640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                            ),
                                                            start: 1277,
                                                            end: 1309,
                                                            as_str(): "{\n        Rezult::Ok(true)\n    }",
                                                        },
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1211,
                                            end: 1309,
                                            as_str(): "if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 1211,
                                    end: 1309,
                                    as_str(): "if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 1206,
                            end: 1311,
                            as_str(): "{\n   if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 1151,
                                    end: 1152,
                                    as_str(): "r",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: false,
                            is_mutable: false,
                            mutability_span: Span {
                                src (ptr): 0x00007fe0fc01dd50,
                                path: None,
                                start: 0,
                                end: 0,
                                as_str(): "",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1154,
                                        end: 1160,
                                        as_str(): "Rezult",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                71,
                                            ),
                                            initial_type_id: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 1161,
                                                end: 1165,
                                                as_str(): "bool",
                                            },
                                        },
                                        TypeArgument {
                                            type_id: TypeId(
                                                7260,
                                            ),
                                            initial_type_id: TypeId(
                                                7260,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0fc38b640,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                                ),
                                                start: 1167,
                                                end: 1176,
                                                as_str(): "DumbError",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 1154,
                                end: 1177,
                                as_str(): "Rezult<bool, DumbError>",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 1138,
                        end: 1311,
                        as_str(): "pub fn func6(r: Rezult<bool, DumbError>) -> Rezult<bool, DumbError> {\n   if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0fc38b640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                ),
                                start: 1182,
                                end: 1188,
                                as_str(): "Rezult",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        71,
                                    ),
                                    initial_type_id: TypeId(
                                        71,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1189,
                                        end: 1193,
                                        as_str(): "bool",
                                    },
                                },
                                TypeArgument {
                                    type_id: TypeId(
                                        7261,
                                    ),
                                    initial_type_id: TypeId(
                                        7261,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fc38b640,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                        ),
                                        start: 1195,
                                        end: 1204,
                                        as_str(): "DumbError",
                                    },
                                },
                            ],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 1182,
                        end: 1205,
                        as_str(): "Rezult<bool, DumbError>",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 1138,
            end: 1311,
            as_str(): "pub fn func6(r: Rezult<bool, DumbError>) -> Rezult<bool, DumbError> {\n   if r.trivial() {\n        Rezult::Err(DumbError::Error)\n    } else {\n        Rezult::Ok(true)\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 1341,
                            end: 1345,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fc38b640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                            ),
                                            start: 1360,
                                            end: 1364,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc38b640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                                    ),
                                    start: 1360,
                                    end: 1364,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0fc38b640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                            ),
                            start: 1356,
                            end: 1366,
                            as_str(): "{\n  true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 1338,
                        end: 1366,
                        as_str(): "fn main() -> bool {\n  true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0fc38b640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
                        ),
                        start: 1351,
                        end: 1355,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0fc38b640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRiDoptb/funcs_with_generic_types/src/main.sw",
            ),
            start: 1338,
            end: 1366,
            as_str(): "fn main() -> bool {\n  true\n}",
        },
    },
]
