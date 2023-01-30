[
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 14,
                                    end: 18,
                                    as_str(): "core",
                                },
                                is_raw_ident: false,
                            },
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 20,
                                    end: 23,
                                    as_str(): "ops",
                                },
                                is_raw_ident: false,
                            },
                        ],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 25,
                                end: 27,
                                as_str(): "Eq",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Array(
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 33,
                                end: 36,
                                as_str(): "u64",
                            },
                        },
                        Length {
                            val: 2,
                            span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 38,
                                end: 39,
                                as_str(): "2",
                            },
                        },
                    ),
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 32,
                        end: 40,
                        as_str(): "[u64; 2]",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 50,
                                    end: 52,
                                    as_str(): "eq",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 99,
                                                            as_str(): "i",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    type_ascription: Unknown,
                                                    type_ascription_span: None,
                                                    body: Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                0,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 102,
                                                            end: 103,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                    is_mutable: true,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 90,
                                            end: 104,
                                            as_str(): "let mut i = 0;",
                                        },
                                    },
                                    AstNode {
                                        content: Expression(
                                            Expression {
                                                kind: WhileLoop(
                                                    WhileLoopExpression {
                                                        condition: Expression {
                                                            kind: MethodApplication(
                                                                MethodApplicationExpression {
                                                                    method_name_binding: TypeBinding {
                                                                        inner: FromTrait {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 121,
                                                                                            end: 122,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 121,
                                                                                            end: 122,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "lt",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 121,
                                                                                        end: 122,
                                                                                        as_str(): "<",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 121,
                                                                            end: 122,
                                                                            as_str(): "<",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 119,
                                                                                        end: 120,
                                                                                        as_str(): "i",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 119,
                                                                                end: 120,
                                                                                as_str(): "i",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: Literal(
                                                                                Numeric(
                                                                                    2,
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 123,
                                                                                end: 124,
                                                                                as_str(): "2",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 119,
                                                                end: 124,
                                                                as_str(): "i < 2",
                                                            },
                                                        },
                                                        body: CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: If(
                                                                                IfExpression {
                                                                                    condition: Expression {
                                                                                        kind: MethodApplication(
                                                                                            MethodApplicationExpression {
                                                                                                method_name_binding: TypeBinding {
                                                                                                    inner: FromTrait {
                                                                                                        call_path: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "core",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 150,
                                                                                                                        end: 152,
                                                                                                                        as_str(): "!=",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 150,
                                                                                                                        end: 152,
                                                                                                                        as_str(): "!=",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "neq",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 150,
                                                                                                                    end: 152,
                                                                                                                    as_str(): "!=",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 150,
                                                                                                        end: 152,
                                                                                                        as_str(): "!=",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: ArrayIndex(
                                                                                                            ArrayIndexExpression {
                                                                                                                prefix: Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 142,
                                                                                                                                end: 146,
                                                                                                                                as_str(): "self",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 142,
                                                                                                                        end: 146,
                                                                                                                        as_str(): "self",
                                                                                                                    },
                                                                                                                },
                                                                                                                index: Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 147,
                                                                                                                                end: 148,
                                                                                                                                as_str(): "i",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 147,
                                                                                                                        end: 148,
                                                                                                                        as_str(): "i",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 142,
                                                                                                            end: 149,
                                                                                                            as_str(): "self[i]",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: ArrayIndex(
                                                                                                            ArrayIndexExpression {
                                                                                                                prefix: Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 153,
                                                                                                                                end: 158,
                                                                                                                                as_str(): "other",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 153,
                                                                                                                        end: 158,
                                                                                                                        as_str(): "other",
                                                                                                                    },
                                                                                                                },
                                                                                                                index: Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 159,
                                                                                                                                end: 160,
                                                                                                                                as_str(): "i",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 159,
                                                                                                                        end: 160,
                                                                                                                        as_str(): "i",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 153,
                                                                                                            end: 161,
                                                                                                            as_str(): "other[i]",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 142,
                                                                                            end: 161,
                                                                                            as_str(): "self[i] != other[i]",
                                                                                        },
                                                                                    },
                                                                                    then: Expression {
                                                                                        kind: CodeBlock(
                                                                                            CodeBlock {
                                                                                                contents: [
                                                                                                    AstNode {
                                                                                                        content: Expression(
                                                                                                            Expression {
                                                                                                                kind: Return(
                                                                                                                    Expression {
                                                                                                                        kind: Literal(
                                                                                                                            Boolean(
                                                                                                                                false,
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 187,
                                                                                                                            end: 192,
                                                                                                                            as_str(): "false",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 180,
                                                                                                                    end: 192,
                                                                                                                    as_str(): "return false",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 180,
                                                                                                            end: 192,
                                                                                                            as_str(): "return false",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                                whole_block_span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 162,
                                                                                                    end: 207,
                                                                                                    as_str(): "{\n                return false;\n            }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 162,
                                                                                            end: 207,
                                                                                            as_str(): "{\n                return false;\n            }",
                                                                                        },
                                                                                    },
                                                                                    else: None,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 139,
                                                                                end: 207,
                                                                                as_str(): "if self[i] != other[i] {\n                return false;\n            }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 139,
                                                                        end: 207,
                                                                        as_str(): "if self[i] != other[i] {\n                return false;\n            }",
                                                                    },
                                                                },
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Reassignment(
                                                                                ReassignmentExpression {
                                                                                    lhs: VariableExpression(
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 220,
                                                                                                        end: 221,
                                                                                                        as_str(): "i",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 220,
                                                                                                end: 221,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    rhs: Expression {
                                                                                        kind: MethodApplication(
                                                                                            MethodApplicationExpression {
                                                                                                method_name_binding: TypeBinding {
                                                                                                    inner: FromTrait {
                                                                                                        call_path: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "core",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 222,
                                                                                                                        end: 224,
                                                                                                                        as_str(): "+=",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 222,
                                                                                                                        end: 224,
                                                                                                                        as_str(): "+=",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "add",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 222,
                                                                                                                    end: 224,
                                                                                                                    as_str(): "+=",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 222,
                                                                                                        end: 224,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 220,
                                                                                                                    end: 221,
                                                                                                                    as_str(): "i",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 220,
                                                                                                            end: 221,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                1,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 225,
                                                                                                            end: 226,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 220,
                                                                                            end: 226,
                                                                                            as_str(): "i += 1",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 220,
                                                                                end: 226,
                                                                                as_str(): "i += 1",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 220,
                                                                        end: 226,
                                                                        as_str(): "i += 1",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 125,
                                                                end: 237,
                                                                as_str(): "{\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 237,
                                                    as_str(): "while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 113,
                                            end: 237,
                                            as_str(): "while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }",
                                        },
                                    },
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 246,
                                                    end: 250,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 246,
                                            end: 250,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 80,
                                    end: 256,
                                    as_str(): "{\n        let mut i = 0;\n        while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 53,
                                            end: 57,
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 57,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 64,
                                            as_str(): "other",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 66,
                                        end: 70,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 47,
                                end: 256,
                                as_str(): "fn eq(self, other: Self) -> bool {\n        let mut i = 0;\n        while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 75,
                                end: 79,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 9,
                        end: 258,
                        as_str(): "impl core::ops::Eq for [u64; 2] {\n    fn eq(self, other: Self) -> bool {\n        let mut i = 0;\n        while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 9,
            end: 258,
            as_str(): "impl core::ops::Eq for [u64; 2] {\n    fn eq(self, other: Self) -> bool {\n        let mut i = 0;\n        while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 265,
                                    end: 269,
                                    as_str(): "core",
                                },
                                is_raw_ident: false,
                            },
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 271,
                                    end: 274,
                                    as_str(): "ops",
                                },
                                is_raw_ident: false,
                            },
                        ],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 276,
                                end: 278,
                                as_str(): "Eq",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 283,
                                end: 286,
                                as_str(): "Vec",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        31628,
                                    ),
                                    initial_type_id: TypeId(
                                        31628,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 287,
                                        end: 295,
                                        as_str(): "[u64; 2]",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 283,
                        end: 296,
                        as_str(): "Vec<[u64; 2]>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 306,
                                    end: 308,
                                    as_str(): "eq",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: Expression(
                                            Expression {
                                                kind: If(
                                                    IfExpression {
                                                        condition: Expression {
                                                            kind: MethodApplication(
                                                                MethodApplicationExpression {
                                                                    method_name_binding: TypeBinding {
                                                                        inner: FromTrait {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 360,
                                                                                            end: 362,
                                                                                            as_str(): "!=",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 360,
                                                                                            end: 362,
                                                                                            as_str(): "!=",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "neq",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 360,
                                                                                        end: 362,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 360,
                                                                            end: 362,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: MethodApplication(
                                                                                MethodApplicationExpression {
                                                                                    method_name_binding: TypeBinding {
                                                                                        inner: FromModule {
                                                                                            method_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 354,
                                                                                                    end: 357,
                                                                                                    as_str(): "len",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 354,
                                                                                            end: 357,
                                                                                            as_str(): "len",
                                                                                        },
                                                                                    },
                                                                                    contract_call_params: [],
                                                                                    arguments: [
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 349,
                                                                                                        end: 353,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 349,
                                                                                                end: 353,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 349,
                                                                                end: 359,
                                                                                as_str(): "self.len()",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: MethodApplication(
                                                                                MethodApplicationExpression {
                                                                                    method_name_binding: TypeBinding {
                                                                                        inner: FromModule {
                                                                                            method_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 369,
                                                                                                    end: 372,
                                                                                                    as_str(): "len",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 369,
                                                                                            end: 372,
                                                                                            as_str(): "len",
                                                                                        },
                                                                                    },
                                                                                    contract_call_params: [],
                                                                                    arguments: [
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 363,
                                                                                                        end: 368,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 363,
                                                                                                end: 368,
                                                                                                as_str(): "other",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 363,
                                                                                end: 374,
                                                                                as_str(): "other.len()",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 349,
                                                                end: 374,
                                                                as_str(): "self.len() != other.len()",
                                                            },
                                                        },
                                                        then: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [
                                                                        AstNode {
                                                                            content: Expression(
                                                                                Expression {
                                                                                    kind: Return(
                                                                                        Expression {
                                                                                            kind: Literal(
                                                                                                Boolean(
                                                                                                    false,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 396,
                                                                                                end: 401,
                                                                                                as_str(): "false",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 389,
                                                                                        end: 401,
                                                                                        as_str(): "return false",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 389,
                                                                                end: 401,
                                                                                as_str(): "return false",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 375,
                                                                        end: 412,
                                                                        as_str(): "{\n            return false;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 375,
                                                                end: 412,
                                                                as_str(): "{\n            return false;\n        }",
                                                            },
                                                        },
                                                        else: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 346,
                                                    end: 412,
                                                    as_str(): "if self.len() != other.len() {\n            return false;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 346,
                                            end: 412,
                                            as_str(): "if self.len() != other.len() {\n            return false;\n        }",
                                        },
                                    },
                                    AstNode {
                                        content: Declaration(
                                            VariableDeclaration(
                                                VariableDeclaration {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 429,
                                                            end: 430,
                                                            as_str(): "i",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    type_ascription: Unknown,
                                                    type_ascription_span: None,
                                                    body: Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                0,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 433,
                                                            end: 434,
                                                            as_str(): "0",
                                                        },
                                                    },
                                                    is_mutable: true,
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 421,
                                            end: 435,
                                            as_str(): "let mut i = 0;",
                                        },
                                    },
                                    AstNode {
                                        content: Expression(
                                            Expression {
                                                kind: WhileLoop(
                                                    WhileLoopExpression {
                                                        condition: Expression {
                                                            kind: MethodApplication(
                                                                MethodApplicationExpression {
                                                                    method_name_binding: TypeBinding {
                                                                        inner: FromTrait {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 452,
                                                                                            end: 453,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 452,
                                                                                            end: 453,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "lt",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 452,
                                                                                        end: 453,
                                                                                        as_str(): "<",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 452,
                                                                            end: 453,
                                                                            as_str(): "<",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 450,
                                                                                        end: 451,
                                                                                        as_str(): "i",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 450,
                                                                                end: 451,
                                                                                as_str(): "i",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: MethodApplication(
                                                                                MethodApplicationExpression {
                                                                                    method_name_binding: TypeBinding {
                                                                                        inner: FromModule {
                                                                                            method_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 459,
                                                                                                    end: 462,
                                                                                                    as_str(): "len",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 459,
                                                                                            end: 462,
                                                                                            as_str(): "len",
                                                                                        },
                                                                                    },
                                                                                    contract_call_params: [],
                                                                                    arguments: [
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 454,
                                                                                                        end: 458,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 454,
                                                                                                end: 458,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 454,
                                                                                end: 464,
                                                                                as_str(): "self.len()",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 450,
                                                                end: 464,
                                                                as_str(): "i < self.len()",
                                                            },
                                                        },
                                                        body: CodeBlock {
                                                            contents: [
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: If(
                                                                                IfExpression {
                                                                                    condition: Expression {
                                                                                        kind: MethodApplication(
                                                                                            MethodApplicationExpression {
                                                                                                method_name_binding: TypeBinding {
                                                                                                    inner: FromTrait {
                                                                                                        call_path: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "core",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 503,
                                                                                                                        end: 505,
                                                                                                                        as_str(): "!=",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 503,
                                                                                                                        end: 505,
                                                                                                                        as_str(): "!=",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "neq",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 503,
                                                                                                                    end: 505,
                                                                                                                    as_str(): "!=",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 503,
                                                                                                        end: 505,
                                                                                                        as_str(): "!=",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: MethodApplication(
                                                                                                            MethodApplicationExpression {
                                                                                                                method_name_binding: TypeBinding {
                                                                                                                    inner: FromModule {
                                                                                                                        method_name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 494,
                                                                                                                                end: 500,
                                                                                                                                as_str(): "unwrap",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 494,
                                                                                                                        end: 500,
                                                                                                                        as_str(): "unwrap",
                                                                                                                    },
                                                                                                                },
                                                                                                                contract_call_params: [],
                                                                                                                arguments: [
                                                                                                                    Expression {
                                                                                                                        kind: MethodApplication(
                                                                                                                            MethodApplicationExpression {
                                                                                                                                method_name_binding: TypeBinding {
                                                                                                                                    inner: FromModule {
                                                                                                                                        method_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 487,
                                                                                                                                                end: 490,
                                                                                                                                                as_str(): "get",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 487,
                                                                                                                                        end: 490,
                                                                                                                                        as_str(): "get",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                contract_call_params: [],
                                                                                                                                arguments: [
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 482,
                                                                                                                                                    end: 486,
                                                                                                                                                    as_str(): "self",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 482,
                                                                                                                                            end: 486,
                                                                                                                                            as_str(): "self",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 491,
                                                                                                                                                    end: 492,
                                                                                                                                                    as_str(): "i",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 491,
                                                                                                                                            end: 492,
                                                                                                                                            as_str(): "i",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 482,
                                                                                                                            end: 493,
                                                                                                                            as_str(): "self.get(i)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 482,
                                                                                                            end: 502,
                                                                                                            as_str(): "self.get(i).unwrap()",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: MethodApplication(
                                                                                                            MethodApplicationExpression {
                                                                                                                method_name_binding: TypeBinding {
                                                                                                                    inner: FromModule {
                                                                                                                        method_name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 519,
                                                                                                                                end: 525,
                                                                                                                                as_str(): "unwrap",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 519,
                                                                                                                        end: 525,
                                                                                                                        as_str(): "unwrap",
                                                                                                                    },
                                                                                                                },
                                                                                                                contract_call_params: [],
                                                                                                                arguments: [
                                                                                                                    Expression {
                                                                                                                        kind: MethodApplication(
                                                                                                                            MethodApplicationExpression {
                                                                                                                                method_name_binding: TypeBinding {
                                                                                                                                    inner: FromModule {
                                                                                                                                        method_name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 512,
                                                                                                                                                end: 515,
                                                                                                                                                as_str(): "get",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 512,
                                                                                                                                        end: 515,
                                                                                                                                        as_str(): "get",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                contract_call_params: [],
                                                                                                                                arguments: [
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 506,
                                                                                                                                                    end: 511,
                                                                                                                                                    as_str(): "other",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 506,
                                                                                                                                            end: 511,
                                                                                                                                            as_str(): "other",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 516,
                                                                                                                                                    end: 517,
                                                                                                                                                    as_str(): "i",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 516,
                                                                                                                                            end: 517,
                                                                                                                                            as_str(): "i",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 506,
                                                                                                                            end: 518,
                                                                                                                            as_str(): "other.get(i)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 506,
                                                                                                            end: 527,
                                                                                                            as_str(): "other.get(i).unwrap()",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 482,
                                                                                            end: 527,
                                                                                            as_str(): "self.get(i).unwrap() != other.get(i).unwrap()",
                                                                                        },
                                                                                    },
                                                                                    then: Expression {
                                                                                        kind: CodeBlock(
                                                                                            CodeBlock {
                                                                                                contents: [
                                                                                                    AstNode {
                                                                                                        content: Expression(
                                                                                                            Expression {
                                                                                                                kind: Return(
                                                                                                                    Expression {
                                                                                                                        kind: Literal(
                                                                                                                            Boolean(
                                                                                                                                false,
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 553,
                                                                                                                            end: 558,
                                                                                                                            as_str(): "false",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 546,
                                                                                                                    end: 558,
                                                                                                                    as_str(): "return false",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 546,
                                                                                                            end: 558,
                                                                                                            as_str(): "return false",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                                whole_block_span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 528,
                                                                                                    end: 573,
                                                                                                    as_str(): "{\n                return false;\n            }",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 528,
                                                                                            end: 573,
                                                                                            as_str(): "{\n                return false;\n            }",
                                                                                        },
                                                                                    },
                                                                                    else: None,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 479,
                                                                                end: 573,
                                                                                as_str(): "if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 479,
                                                                        end: 573,
                                                                        as_str(): "if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }",
                                                                    },
                                                                },
                                                                AstNode {
                                                                    content: Expression(
                                                                        Expression {
                                                                            kind: Reassignment(
                                                                                ReassignmentExpression {
                                                                                    lhs: VariableExpression(
                                                                                        Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 586,
                                                                                                        end: 587,
                                                                                                        as_str(): "i",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 586,
                                                                                                end: 587,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    rhs: Expression {
                                                                                        kind: MethodApplication(
                                                                                            MethodApplicationExpression {
                                                                                                method_name_binding: TypeBinding {
                                                                                                    inner: FromTrait {
                                                                                                        call_path: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "core",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 588,
                                                                                                                        end: 590,
                                                                                                                        as_str(): "+=",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: Some(
                                                                                                                        "ops",
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 588,
                                                                                                                        end: 590,
                                                                                                                        as_str(): "+=",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "add",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 588,
                                                                                                                    end: 590,
                                                                                                                    as_str(): "+=",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: true,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 588,
                                                                                                        end: 590,
                                                                                                        as_str(): "+=",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: Variable(
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 586,
                                                                                                                    end: 587,
                                                                                                                    as_str(): "i",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 586,
                                                                                                            end: 587,
                                                                                                            as_str(): "i",
                                                                                                        },
                                                                                                    },
                                                                                                    Expression {
                                                                                                        kind: Literal(
                                                                                                            Numeric(
                                                                                                                1,
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 591,
                                                                                                            end: 592,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 586,
                                                                                            end: 592,
                                                                                            as_str(): "i += 1",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 586,
                                                                                end: 592,
                                                                                as_str(): "i += 1",
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 586,
                                                                        end: 592,
                                                                        as_str(): "i += 1",
                                                                    },
                                                                },
                                                            ],
                                                            whole_block_span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 465,
                                                                end: 603,
                                                                as_str(): "{\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 444,
                                                    end: 603,
                                                    as_str(): "while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 444,
                                            end: 603,
                                            as_str(): "while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }",
                                        },
                                    },
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 612,
                                                    end: 616,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 612,
                                            end: 616,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 336,
                                    end: 622,
                                    as_str(): "{\n        if self.len() != other.len() {\n            return false;\n        }\n        let mut i = 0;\n        while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 309,
                                            end: 313,
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 309,
                                        end: 313,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 315,
                                            end: 320,
                                            as_str(): "other",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 322,
                                        end: 326,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 303,
                                end: 622,
                                as_str(): "fn eq(self, other: Self) -> bool {\n        if self.len() != other.len() {\n            return false;\n        }\n        let mut i = 0;\n        while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 331,
                                end: 335,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 260,
                        end: 624,
                        as_str(): "impl core::ops::Eq for Vec<[u64; 2]> {\n    fn eq(self, other: Self) -> bool {\n        if self.len() != other.len() {\n            return false;\n        }\n        let mut i = 0;\n        while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 260,
            end: 624,
            as_str(): "impl core::ops::Eq for Vec<[u64; 2]> {\n    fn eq(self, other: Self) -> bool {\n        if self.len() != other.len() {\n            return false;\n        }\n        let mut i = 0;\n        while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }\n}",
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
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 629,
                            end: 636,
                            as_str(): "tester1",
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
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 671,
                                                    end: 679,
                                                    as_str(): "expected",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 682,
                                                                                end: 685,
                                                                                as_str(): "Vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 682,
                                                                            end: 685,
                                                                            as_str(): "Vec",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 687,
                                                                            end: 690,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 682,
                                                                end: 690,
                                                                as_str(): "Vec::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 682,
                                                    end: 692,
                                                    as_str(): "Vec::new()",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 663,
                                    end: 693,
                                    as_str(): "let mut expected = Vec::new();",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 707,
                                                                end: 711,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 707,
                                                        end: 711,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 698,
                                                                    end: 706,
                                                                    as_str(): "expected",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 698,
                                                            end: 706,
                                                            as_str(): "expected",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 713,
                                                                            end: 714,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 716,
                                                                            end: 717,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 712,
                                                            end: 718,
                                                            as_str(): "[0, 1]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 698,
                                            end: 719,
                                            as_str(): "expected.push([0, 1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 698,
                                    end: 719,
                                    as_str(): "expected.push([0, 1])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 734,
                                                                end: 738,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 734,
                                                        end: 738,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 725,
                                                                    end: 733,
                                                                    as_str(): "expected",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 725,
                                                            end: 733,
                                                            as_str(): "expected",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 740,
                                                                            end: 741,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 743,
                                                                            end: 744,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 739,
                                                            end: 745,
                                                            as_str(): "[0, 1]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 725,
                                            end: 746,
                                            as_str(): "expected.push([0, 1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 725,
                                    end: 746,
                                    as_str(): "expected.push([0, 1])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 753,
                                                                end: 759,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 753,
                                                        end: 759,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 764,
                                                                                        end: 766,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 764,
                                                                                        end: 766,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 764,
                                                                                    end: 766,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 764,
                                                                        end: 766,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 760,
                                                                                    end: 763,
                                                                                    as_str(): "arg",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 760,
                                                                            end: 763,
                                                                            as_str(): "arg",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 767,
                                                                                    end: 775,
                                                                                    as_str(): "expected",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 767,
                                                                            end: 775,
                                                                            as_str(): "expected",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 760,
                                                            end: 775,
                                                            as_str(): "arg == expected",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 753,
                                            end: 776,
                                            as_str(): "assert(arg == expected)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 753,
                                    end: 776,
                                    as_str(): "assert(arg == expected)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 657,
                            end: 779,
                            as_str(): "{\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg == expected);\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 637,
                                    end: 640,
                                    as_str(): "arg",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 642,
                                        end: 645,
                                        as_str(): "Vec",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                31629,
                                            ),
                                            initial_type_id: TypeId(
                                                31629,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 646,
                                                end: 654,
                                                as_str(): "[u64; 2]",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 642,
                                end: 655,
                                as_str(): "Vec<[u64; 2]>",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 626,
                        end: 779,
                        as_str(): "fn tester1(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg == expected);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 626,
                        end: 656,
                        as_str(): "fn tester1(arg: Vec<[u64; 2]>)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 626,
            end: 779,
            as_str(): "fn tester1(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg == expected);\n}",
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
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 784,
                            end: 791,
                            as_str(): "tester2",
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
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 826,
                                                    end: 834,
                                                    as_str(): "expected",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 837,
                                                                                end: 840,
                                                                                as_str(): "Vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 837,
                                                                            end: 840,
                                                                            as_str(): "Vec",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 842,
                                                                            end: 845,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 837,
                                                                end: 845,
                                                                as_str(): "Vec::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 837,
                                                    end: 847,
                                                    as_str(): "Vec::new()",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 818,
                                    end: 848,
                                    as_str(): "let mut expected = Vec::new();",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 862,
                                                                end: 866,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 862,
                                                        end: 866,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 853,
                                                                    end: 861,
                                                                    as_str(): "expected",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 853,
                                                            end: 861,
                                                            as_str(): "expected",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 868,
                                                                            end: 869,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 871,
                                                                            end: 872,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 867,
                                                            end: 873,
                                                            as_str(): "[0, 1]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 853,
                                            end: 874,
                                            as_str(): "expected.push([0, 1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 853,
                                    end: 874,
                                    as_str(): "expected.push([0, 1])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 889,
                                                                end: 893,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 889,
                                                        end: 893,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 880,
                                                                    end: 888,
                                                                    as_str(): "expected",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 880,
                                                            end: 888,
                                                            as_str(): "expected",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 895,
                                                                            end: 896,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 898,
                                                                            end: 899,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 894,
                                                            end: 900,
                                                            as_str(): "[0, 1]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 880,
                                            end: 901,
                                            as_str(): "expected.push([0, 1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 880,
                                    end: 901,
                                    as_str(): "expected.push([0, 1])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 908,
                                                                end: 914,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 908,
                                                        end: 914,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 919,
                                                                                        end: 921,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 919,
                                                                                        end: 921,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "neq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 919,
                                                                                    end: 921,
                                                                                    as_str(): "!=",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 919,
                                                                        end: 921,
                                                                        as_str(): "!=",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 915,
                                                                                    end: 918,
                                                                                    as_str(): "arg",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 915,
                                                                            end: 918,
                                                                            as_str(): "arg",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 922,
                                                                                    end: 930,
                                                                                    as_str(): "expected",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 922,
                                                                            end: 930,
                                                                            as_str(): "expected",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 915,
                                                            end: 930,
                                                            as_str(): "arg != expected",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 908,
                                            end: 931,
                                            as_str(): "assert(arg != expected)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 908,
                                    end: 931,
                                    as_str(): "assert(arg != expected)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 812,
                            end: 934,
                            as_str(): "{\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg != expected);\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 792,
                                    end: 795,
                                    as_str(): "arg",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 797,
                                        end: 800,
                                        as_str(): "Vec",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                31630,
                                            ),
                                            initial_type_id: TypeId(
                                                31630,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 801,
                                                end: 809,
                                                as_str(): "[u64; 2]",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0c6ce4c30,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                ),
                                start: 797,
                                end: 810,
                                as_str(): "Vec<[u64; 2]>",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 781,
                        end: 934,
                        as_str(): "fn tester2(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg != expected);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 781,
                        end: 811,
                        as_str(): "fn tester2(arg: Vec<[u64; 2]>)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 781,
            end: 934,
            as_str(): "fn tester2(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg != expected);\n}",
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
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 939,
                            end: 943,
                            as_str(): "main",
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
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 966,
                                                    end: 970,
                                                    as_str(): "arg1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 973,
                                                                                end: 976,
                                                                                as_str(): "Vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 973,
                                                                            end: 976,
                                                                            as_str(): "Vec",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 978,
                                                                            end: 981,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 973,
                                                                end: 981,
                                                                as_str(): "Vec::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 973,
                                                    end: 983,
                                                    as_str(): "Vec::new()",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 958,
                                    end: 984,
                                    as_str(): "let mut arg1 = Vec::new();",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 992,
                                                                end: 996,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 992,
                                                        end: 996,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 987,
                                                                    end: 991,
                                                                    as_str(): "arg1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 987,
                                                            end: 991,
                                                            as_str(): "arg1",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 998,
                                                                            end: 999,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1001,
                                                                            end: 1002,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 997,
                                                            end: 1003,
                                                            as_str(): "[0, 1]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 987,
                                            end: 1004,
                                            as_str(): "arg1.push([0, 1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 987,
                                    end: 1004,
                                    as_str(): "arg1.push([0, 1])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1013,
                                                                end: 1017,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1013,
                                                        end: 1017,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 1008,
                                                                    end: 1012,
                                                                    as_str(): "arg1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1008,
                                                            end: 1012,
                                                            as_str(): "arg1",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1019,
                                                                            end: 1020,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1022,
                                                                            end: 1023,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1018,
                                                            end: 1024,
                                                            as_str(): "[0, 1]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1008,
                                            end: 1025,
                                            as_str(): "arg1.push([0, 1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1008,
                                    end: 1025,
                                    as_str(): "arg1.push([0, 1])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1029,
                                                                end: 1036,
                                                                as_str(): "tester1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1029,
                                                        end: 1036,
                                                        as_str(): "tester1",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 1037,
                                                                    end: 1041,
                                                                    as_str(): "arg1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1037,
                                                            end: 1041,
                                                            as_str(): "arg1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1029,
                                            end: 1042,
                                            as_str(): "tester1(arg1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1029,
                                    end: 1042,
                                    as_str(): "tester1(arg1)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 1055,
                                                    end: 1059,
                                                    as_str(): "arg2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
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
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 1062,
                                                                                end: 1065,
                                                                                as_str(): "Vec",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1062,
                                                                            end: 1065,
                                                                            as_str(): "Vec",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1067,
                                                                            end: 1070,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1062,
                                                                end: 1070,
                                                                as_str(): "Vec::new",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 1062,
                                                    end: 1072,
                                                    as_str(): "Vec::new()",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1047,
                                    end: 1073,
                                    as_str(): "let mut arg2 = Vec::new();",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1081,
                                                                end: 1085,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1081,
                                                        end: 1085,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 1076,
                                                                    end: 1080,
                                                                    as_str(): "arg2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1076,
                                                            end: 1080,
                                                            as_str(): "arg2",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1087,
                                                                            end: 1088,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1090,
                                                                            end: 1091,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1086,
                                                            end: 1092,
                                                            as_str(): "[0, 1]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1076,
                                            end: 1093,
                                            as_str(): "arg2.push([0, 1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1076,
                                    end: 1093,
                                    as_str(): "arg2.push([0, 1])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1102,
                                                                end: 1106,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1102,
                                                        end: 1106,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 1097,
                                                                    end: 1101,
                                                                    as_str(): "arg2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1097,
                                                            end: 1101,
                                                            as_str(): "arg2",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1108,
                                                                            end: 1109,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1111,
                                                                            end: 1112,
                                                                            as_str(): "2",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1107,
                                                            end: 1113,
                                                            as_str(): "[0, 2]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1097,
                                            end: 1114,
                                            as_str(): "arg2.push([0, 2])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1097,
                                    end: 1114,
                                    as_str(): "arg2.push([0, 2])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1118,
                                                                end: 1125,
                                                                as_str(): "tester2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1118,
                                                        end: 1125,
                                                        as_str(): "tester2",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 1126,
                                                                    end: 1130,
                                                                    as_str(): "arg2",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1126,
                                                            end: 1130,
                                                            as_str(): "arg2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1118,
                                            end: 1131,
                                            as_str(): "tester2(arg2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1118,
                                    end: 1131,
                                    as_str(): "tester2(arg2)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: MethodApplication(
                                            MethodApplicationExpression {
                                                method_name_binding: TypeBinding {
                                                    inner: FromModule {
                                                        method_name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1141,
                                                                end: 1145,
                                                                as_str(): "push",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1141,
                                                        end: 1145,
                                                        as_str(): "push",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 1136,
                                                                    end: 1140,
                                                                    as_str(): "arg1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1136,
                                                            end: 1140,
                                                            as_str(): "arg1",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Array(
                                                            ArrayExpression {
                                                                contents: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1147,
                                                                            end: 1148,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1150,
                                                                            end: 1151,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                ],
                                                                length_span: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1146,
                                                            end: 1152,
                                                            as_str(): "[0, 1]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1136,
                                            end: 1153,
                                            as_str(): "arg1.push([0, 1])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1136,
                                    end: 1153,
                                    as_str(): "arg1.push([0, 1])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1157,
                                                                end: 1164,
                                                                as_str(): "tester2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1157,
                                                        end: 1164,
                                                        as_str(): "tester2",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 1165,
                                                                    end: 1169,
                                                                    as_str(): "arg1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1165,
                                                            end: 1169,
                                                            as_str(): "arg1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1157,
                                            end: 1170,
                                            as_str(): "tester2(arg1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1157,
                                    end: 1170,
                                    as_str(): "tester2(arg1)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1175,
                                            end: 1176,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1175,
                                    end: 1176,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 953,
                            end: 1178,
                            as_str(): "{\n\n  let mut arg1 = Vec::new();\n  arg1.push([0, 1]);\n  arg1.push([0, 1]);\n  tester1(arg1);\n\n  let mut arg2 = Vec::new();\n  arg2.push([0, 1]);\n  arg2.push([0, 2]);\n  tester2(arg2);\n\n  arg1.push([0, 1]);\n  tester2(arg1);\n\n  1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 936,
                        end: 1178,
                        as_str(): "fn main() -> u64 {\n\n  let mut arg1 = Vec::new();\n  arg1.push([0, 1]);\n  arg1.push([0, 1]);\n  tester1(arg1);\n\n  let mut arg2 = Vec::new();\n  arg2.push([0, 1]);\n  arg2.push([0, 2]);\n  tester2(arg2);\n\n  arg1.push([0, 1]);\n  tester2(arg1);\n\n  1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 949,
                        end: 952,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 936,
            end: 1178,
            as_str(): "fn main() -> u64 {\n\n  let mut arg1 = Vec::new();\n  arg1.push([0, 1]);\n  arg1.push([0, 1]);\n  tester1(arg1);\n\n  let mut arg2 = Vec::new();\n  arg2.push([0, 1]);\n  arg2.push([0, 2]);\n  tester2(arg2);\n\n  arg1.push([0, 1]);\n  tester2(arg1);\n\n  1\n}",
        },
    },
]
