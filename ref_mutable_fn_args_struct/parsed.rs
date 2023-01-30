[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe05c5b18a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                            ),
                            start: 16,
                            end: 19,
                            as_str(): "Foo",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 26,
                                    end: 31,
                                    as_str(): "value",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe05c5b18a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                ),
                                start: 26,
                                end: 36,
                                as_str(): "value: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe05c5b18a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                ),
                                start: 33,
                                end: 36,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe05c5b18a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                        ),
                        start: 9,
                        end: 38,
                        as_str(): "struct Foo {\n    value: u64\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05c5b18a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
            ),
            start: 9,
            end: 38,
            as_str(): "struct Foo {\n    value: u64\n}",
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
                                src (ptr): 0x00007fe05c5b18a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                ),
                                start: 45,
                                end: 48,
                                as_str(): "Foo",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe05c5b18a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                        ),
                        start: 45,
                        end: 48,
                        as_str(): "Foo",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 62,
                                    end: 65,
                                    as_str(): "set",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Public,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: Expression(
                                            Expression {
                                                kind: Reassignment(
                                                    ReassignmentExpression {
                                                        lhs: VariableExpression(
                                                            Expression {
                                                                kind: Subfield(
                                                                    SubfieldExpression {
                                                                        prefix: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 109,
                                                                                        end: 113,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 113,
                                                                                as_str(): "self",
                                                                            },
                                                                        },
                                                                        field_to_access: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                ),
                                                                                start: 114,
                                                                                end: 119,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 109,
                                                                    end: 119,
                                                                    as_str(): "self.value",
                                                                },
                                                            },
                                                        ),
                                                        rhs: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 122,
                                                                        end: 127,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 122,
                                                                end: 127,
                                                                as_str(): "value",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05c5b18a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                    ),
                                                    start: 109,
                                                    end: 127,
                                                    as_str(): "self.value = value",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 109,
                                            end: 127,
                                            as_str(): "self.value = value",
                                        },
                                    },
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Subfield(
                                                    SubfieldExpression {
                                                        prefix: Expression {
                                                            kind: Variable(
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 137,
                                                                        end: 141,
                                                                        as_str(): "self",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 137,
                                                                end: 141,
                                                                as_str(): "self",
                                                            },
                                                        },
                                                        field_to_access: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 142,
                                                                end: 147,
                                                                as_str(): "value",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05c5b18a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                    ),
                                                    start: 137,
                                                    end: 147,
                                                    as_str(): "self.value",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 137,
                                            end: 147,
                                            as_str(): "self.value",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 99,
                                    end: 153,
                                    as_str(): "{\n        self.value = value;\n        self.value\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 74,
                                            end: 78,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: true,
                                    is_mutable: true,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 66,
                                        end: 73,
                                        as_str(): "ref mut",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 74,
                                        end: 78,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 80,
                                            end: 85,
                                            as_str(): "value",
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
                                    type_info: UnsignedInteger(
                                        SixtyFour,
                                    ),
                                    type_span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 87,
                                        end: 90,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe05c5b18a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                ),
                                start: 55,
                                end: 153,
                                as_str(): "pub fn set(ref mut self, value: u64) -> u64 {\n        self.value = value;\n        self.value\n    }",
                            },
                            return_type: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe05c5b18a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                ),
                                start: 95,
                                end: 98,
                                as_str(): "u64",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe05c5b18a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                        ),
                        start: 40,
                        end: 155,
                        as_str(): "impl Foo {\n    pub fn set(ref mut self, value: u64) -> u64 {\n        self.value = value;\n        self.value\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05c5b18a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
            ),
            start: 40,
            end: 155,
            as_str(): "impl Foo {\n    pub fn set(ref mut self, value: u64) -> u64 {\n        self.value = value;\n        self.value\n    }\n}",
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
                            src (ptr): 0x00007fe05c5b18a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                            ),
                            start: 160,
                            end: 167,
                            as_str(): "mut_foo",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 199,
                                                                as_str(): "set",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5b18a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                        ),
                                                        start: 196,
                                                        end: 199,
                                                        as_str(): "set",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 192,
                                                                    end: 195,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 192,
                                                            end: 195,
                                                            as_str(): "foo",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                10,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 200,
                                                            end: 202,
                                                            as_str(): "10",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 192,
                                            end: 203,
                                            as_str(): "foo.set(10)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 192,
                                    end: 203,
                                    as_str(): "foo.set(10)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05c5b18a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                            ),
                            start: 186,
                            end: 206,
                            as_str(): "{\n    foo.set(10);\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 176,
                                    end: 179,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            is_reference: true,
                            is_mutable: true,
                            mutability_span: Span {
                                src (ptr): 0x00007fe05c5b18a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                ),
                                start: 168,
                                end: 175,
                                as_str(): "ref mut",
                            },
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 181,
                                        end: 184,
                                        as_str(): "Foo",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe05c5b18a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                ),
                                start: 181,
                                end: 184,
                                as_str(): "Foo",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe05c5b18a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                        ),
                        start: 157,
                        end: 206,
                        as_str(): "fn mut_foo(ref mut foo: Foo) {\n    foo.set(10);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05c5b18a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                        ),
                        start: 157,
                        end: 185,
                        as_str(): "fn mut_foo(ref mut foo: Foo)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05c5b18a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
            ),
            start: 157,
            end: 206,
            as_str(): "fn mut_foo(ref mut foo: Foo) {\n    foo.set(10);\n}",
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
                            src (ptr): 0x00007fe05c5b18a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                            ),
                            start: 211,
                            end: 215,
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
                                                    src (ptr): 0x00007fe05c5b18a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                    ),
                                                    start: 239,
                                                    end: 242,
                                                    as_str(): "foo",
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
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 245,
                                                                        end: 248,
                                                                        as_str(): "Foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 245,
                                                                end: 248,
                                                                as_str(): "Foo",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 251,
                                                                        end: 256,
                                                                        as_str(): "value",
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
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 258,
                                                                        end: 259,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 251,
                                                                    end: 259,
                                                                    as_str(): "value: 0",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe05c5b18a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                    ),
                                                    start: 245,
                                                    end: 261,
                                                    as_str(): "Foo { value: 0 }",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 231,
                                    end: 262,
                                    as_str(): "let mut foo = Foo { value: 0 };",
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
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 267,
                                                                end: 274,
                                                                as_str(): "mut_foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5b18a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                        ),
                                                        start: 267,
                                                        end: 274,
                                                        as_str(): "mut_foo",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 275,
                                                                    end: 278,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 275,
                                                            end: 278,
                                                            as_str(): "foo",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 267,
                                            end: 279,
                                            as_str(): "mut_foo(foo)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 267,
                                    end: 279,
                                    as_str(): "mut_foo(foo)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Subfield(
                                            SubfieldExpression {
                                                prefix: Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 285,
                                                                end: 288,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5b18a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                        ),
                                                        start: 285,
                                                        end: 288,
                                                        as_str(): "foo",
                                                    },
                                                },
                                                field_to_access: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5b18a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                        ),
                                                        start: 289,
                                                        end: 294,
                                                        as_str(): "value",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 285,
                                            end: 294,
                                            as_str(): "foo.value",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 285,
                                    end: 294,
                                    as_str(): "foo.value",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe05c5b18a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                            ),
                            start: 225,
                            end: 296,
                            as_str(): "{\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe05c5b18a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                        ),
                        start: 208,
                        end: 296,
                        as_str(): "fn main() -> u64 {\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe05c5b18a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                        ),
                        start: 221,
                        end: 224,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe05c5b18a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
            ),
            start: 208,
            end: 296,
            as_str(): "fn main() -> u64 {\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
        },
    },
]
