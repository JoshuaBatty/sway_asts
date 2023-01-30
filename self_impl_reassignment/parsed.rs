[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 19,
                            end: 25,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 27,
                            end: 33,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 9,
            end: 51,
            as_str(): "use std::{assert::assert, revert::revert};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 35,
                            end: 41,
                            as_str(): "revert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 43,
                            end: 49,
                            as_str(): "revert",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 9,
            end: 51,
            as_str(): "use std::{assert::assert, revert::revert};",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 60,
                            end: 61,
                            as_str(): "A",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 68,
                                    end: 69,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 68,
                                end: 74,
                                as_str(): "a: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 71,
                                end: 74,
                                as_str(): "u64",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 80,
                                    end: 81,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 80,
                                end: 86,
                                as_str(): "b: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 83,
                                end: 86,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 53,
                        end: 89,
                        as_str(): "struct A {\n    a: u64,\n    b: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 53,
            end: 89,
            as_str(): "struct A {\n    a: u64,\n    b: u64,\n}",
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
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 96,
                                end: 97,
                                as_str(): "A",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 96,
                        end: 97,
                        as_str(): "A",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 107,
                                    end: 108,
                                    as_str(): "f",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 133,
                                                                                        end: 137,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 133,
                                                                                end: 137,
                                                                                as_str(): "self",
                                                                            },
                                                                        },
                                                                        field_to_access: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 138,
                                                                                end: 139,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 133,
                                                                    end: 139,
                                                                    as_str(): "self.a",
                                                                },
                                                            },
                                                        ),
                                                        rhs: Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    42,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 142,
                                                                end: 144,
                                                                as_str(): "42",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 144,
                                                    as_str(): "self.a = 42",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 133,
                                            end: 144,
                                            as_str(): "self.a = 42",
                                        },
                                    },
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 154,
                                                                                        end: 158,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 154,
                                                                                end: 158,
                                                                                as_str(): "self",
                                                                            },
                                                                        },
                                                                        field_to_access: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 159,
                                                                                end: 160,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 154,
                                                                    end: 160,
                                                                    as_str(): "self.b",
                                                                },
                                                            },
                                                        ),
                                                        rhs: Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    77,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 163,
                                                                end: 165,
                                                                as_str(): "77",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 154,
                                                    end: 165,
                                                    as_str(): "self.b = 77",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 154,
                                            end: 165,
                                            as_str(): "self.b = 77",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 123,
                                    end: 172,
                                    as_str(): "{\n        self.a = 42;\n        self.b = 77;\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 121,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: true,
                                    is_mutable: true,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 109,
                                        end: 116,
                                        as_str(): "ref mut",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 117,
                                        end: 121,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 104,
                                end: 172,
                                as_str(): "fn f(ref mut self) {\n        self.a = 42;\n        self.b = 77;\n    }",
                            },
                            return_type: Tuple(
                                [],
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 104,
                                end: 122,
                                as_str(): "fn f(ref mut self)",
                            },
                        },
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 181,
                                    end: 182,
                                    as_str(): "g",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 217,
                                                                                        end: 221,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 217,
                                                                                end: 221,
                                                                                as_str(): "self",
                                                                            },
                                                                        },
                                                                        field_to_access: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 222,
                                                                                end: 223,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 217,
                                                                    end: 223,
                                                                    as_str(): "self.a",
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
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 233,
                                                                                            end: 234,
                                                                                            as_str(): "+",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 233,
                                                                                            end: 234,
                                                                                            as_str(): "+",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "add",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 233,
                                                                                        end: 234,
                                                                                        as_str(): "+",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 233,
                                                                            end: 234,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Subfield(
                                                                                SubfieldExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 226,
                                                                                                    end: 230,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 226,
                                                                                            end: 230,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 231,
                                                                                            end: 232,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 226,
                                                                                end: 232,
                                                                                as_str(): "self.a",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 235,
                                                                                        end: 238,
                                                                                        as_str(): "inc",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 235,
                                                                                end: 238,
                                                                                as_str(): "inc",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 226,
                                                                end: 238,
                                                                as_str(): "self.a + inc",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 217,
                                                    end: 238,
                                                    as_str(): "self.a = self.a + inc",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 217,
                                            end: 238,
                                            as_str(): "self.a = self.a + inc",
                                        },
                                    },
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 248,
                                                                                        end: 252,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 248,
                                                                                end: 252,
                                                                                as_str(): "self",
                                                                            },
                                                                        },
                                                                        field_to_access: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 253,
                                                                                end: 254,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 248,
                                                                    end: 254,
                                                                    as_str(): "self.b",
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
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 264,
                                                                                            end: 265,
                                                                                            as_str(): "+",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 264,
                                                                                            end: 265,
                                                                                            as_str(): "+",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "add",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 264,
                                                                                        end: 265,
                                                                                        as_str(): "+",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 264,
                                                                            end: 265,
                                                                            as_str(): "+",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: Subfield(
                                                                                SubfieldExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 257,
                                                                                                    end: 261,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 257,
                                                                                            end: 261,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 262,
                                                                                            end: 263,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 257,
                                                                                end: 263,
                                                                                as_str(): "self.b",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 266,
                                                                                        end: 269,
                                                                                        as_str(): "inc",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 266,
                                                                                end: 269,
                                                                                as_str(): "inc",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 257,
                                                                end: 269,
                                                                as_str(): "self.b + inc",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 248,
                                                    end: 269,
                                                    as_str(): "self.b = self.b + inc",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 248,
                                            end: 269,
                                            as_str(): "self.b = self.b + inc",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 207,
                                    end: 276,
                                    as_str(): "{\n        self.a = self.a + inc;\n        self.b = self.b + inc;\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 191,
                                            end: 195,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: true,
                                    is_mutable: true,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 183,
                                        end: 190,
                                        as_str(): "ref mut",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 191,
                                        end: 195,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 197,
                                            end: 200,
                                            as_str(): "inc",
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
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 202,
                                        end: 205,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 178,
                                end: 276,
                                as_str(): "fn g(ref mut self, inc: u64) {\n        self.a = self.a + inc;\n        self.b = self.b + inc;\n    }",
                            },
                            return_type: Tuple(
                                [],
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 178,
                                end: 206,
                                as_str(): "fn g(ref mut self, inc: u64)",
                            },
                        },
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 285,
                                    end: 286,
                                    as_str(): "h",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Reassignment(
                                                    ReassignmentExpression {
                                                        lhs: VariableExpression(
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 311,
                                                                            end: 315,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 311,
                                                                    end: 315,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                        ),
                                                        rhs: Expression {
                                                            kind: Struct(
                                                                StructExpression {
                                                                    call_path_binding: TypeBinding {
                                                                        inner: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 318,
                                                                                    end: 319,
                                                                                    as_str(): "A",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 318,
                                                                            end: 319,
                                                                            as_str(): "A",
                                                                        },
                                                                    },
                                                                    fields: [
                                                                        StructExpressionField {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 334,
                                                                                    end: 335,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        100,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 337,
                                                                                    end: 340,
                                                                                    as_str(): "100",
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 334,
                                                                                end: 340,
                                                                                as_str(): "a: 100",
                                                                            },
                                                                        },
                                                                        StructExpressionField {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 354,
                                                                                    end: 355,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            value: Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        200,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 357,
                                                                                    end: 360,
                                                                                    as_str(): "200",
                                                                                },
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 354,
                                                                                end: 360,
                                                                                as_str(): "b: 200",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 318,
                                                                end: 371,
                                                                as_str(): "A {\n            a: 100,\n            b: 200,\n        }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 311,
                                                    end: 371,
                                                    as_str(): "self = A {\n            a: 100,\n            b: 200,\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 311,
                                            end: 371,
                                            as_str(): "self = A {\n            a: 100,\n            b: 200,\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 301,
                                    end: 377,
                                    as_str(): "{\n        self = A {\n            a: 100,\n            b: 200,\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 295,
                                            end: 299,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: true,
                                    is_mutable: true,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 287,
                                        end: 294,
                                        as_str(): "ref mut",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 295,
                                        end: 299,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 282,
                                end: 377,
                                as_str(): "fn h(ref mut self) {\n        self = A {\n            a: 100,\n            b: 200,\n        }\n    }",
                            },
                            return_type: Tuple(
                                [],
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 282,
                                end: 300,
                                as_str(): "fn h(ref mut self)",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 91,
                        end: 379,
                        as_str(): "impl A {\n    fn f(ref mut self) {\n        self.a = 42;\n        self.b = 77;\n    }\n\n    fn g(ref mut self, inc: u64) {\n        self.a = self.a + inc;\n        self.b = self.b + inc;\n    }\n\n    fn h(ref mut self) {\n        self = A {\n            a: 100,\n            b: 200,\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 91,
            end: 379,
            as_str(): "impl A {\n    fn f(ref mut self) {\n        self.a = 42;\n        self.b = 77;\n    }\n\n    fn g(ref mut self, inc: u64) {\n        self.a = self.a + inc;\n        self.b = self.b + inc;\n    }\n\n    fn h(ref mut self) {\n        self = A {\n            a: 100,\n            b: 200,\n        }\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 386,
                            end: 387,
                            as_str(): "E",
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
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 394,
                                    end: 395,
                                    as_str(): "X",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 397,
                                end: 400,
                                as_str(): "u64",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 394,
                                end: 400,
                                as_str(): "X: u64",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 406,
                                    end: 407,
                                    as_str(): "Y",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 409,
                                end: 412,
                                as_str(): "u64",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 406,
                                end: 412,
                                as_str(): "Y: u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 381,
                        end: 415,
                        as_str(): "enum E {\n    X: u64,\n    Y: u64,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 381,
            end: 415,
            as_str(): "enum E {\n    X: u64,\n    Y: u64,\n}",
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
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 422,
                                end: 423,
                                as_str(): "E",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 422,
                        end: 423,
                        as_str(): "E",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 433,
                                    end: 434,
                                    as_str(): "j",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Reassignment(
                                                    ReassignmentExpression {
                                                        lhs: VariableExpression(
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 469,
                                                                            end: 473,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 469,
                                                                    end: 473,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                        ),
                                                        rhs: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [
                                                                        AstNode {
                                                                            content: Declaration(
                                                                                VariableDeclaration(
                                                                                    VariableDeclaration {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_1",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 482,
                                                                                                end: 486,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_ascription: Unknown,
                                                                                        type_ascription_span: None,
                                                                                        body: Expression {
                                                                                            kind: Variable(
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 482,
                                                                                                        end: 486,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 482,
                                                                                                end: 486,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                        is_mutable: false,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 476,
                                                                                end: 582,
                                                                                as_str(): "match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }",
                                                                            },
                                                                        },
                                                                        AstNode {
                                                                            content: ImplicitReturnExpression(
                                                                                Expression {
                                                                                    kind: Match(
                                                                                        MatchExpression {
                                                                                            value: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "__match_return_var_name_1",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 482,
                                                                                                            end: 486,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 482,
                                                                                                    end: 486,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                            },
                                                                                            branches: [
                                                                                                MatchBranch {
                                                                                                    scrutinee: EnumScrutinee {
                                                                                                        call_path: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 501,
                                                                                                                        end: 502,
                                                                                                                        as_str(): "E",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 504,
                                                                                                                    end: 505,
                                                                                                                    as_str(): "X",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        value: Variable {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 506,
                                                                                                                    end: 509,
                                                                                                                    as_str(): "val",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 506,
                                                                                                                end: 509,
                                                                                                                as_str(): "val",
                                                                                                            },
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 501,
                                                                                                            end: 510,
                                                                                                            as_str(): "E::X(val)",
                                                                                                        },
                                                                                                    },
                                                                                                    result: Expression {
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
                                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 514,
                                                                                                                                        end: 515,
                                                                                                                                        as_str(): "E",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                type_arguments: [],
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 514,
                                                                                                                                    end: 515,
                                                                                                                                    as_str(): "E",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            suffix: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 517,
                                                                                                                                    end: 518,
                                                                                                                                    as_str(): "Y",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        is_absolute: false,
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 514,
                                                                                                                        end: 518,
                                                                                                                        as_str(): "E::Y",
                                                                                                                    },
                                                                                                                },
                                                                                                                args: [
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
                                                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 523,
                                                                                                                                                        end: 524,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                        "ops",
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 523,
                                                                                                                                                        end: 524,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                            suffix: BaseIdent {
                                                                                                                                                name_override_opt: Some(
                                                                                                                                                    "add",
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 523,
                                                                                                                                                    end: 524,
                                                                                                                                                    as_str(): "+",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            is_absolute: true,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 523,
                                                                                                                                        end: 524,
                                                                                                                                        as_str(): "+",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                contract_call_params: [],
                                                                                                                                arguments: [
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 519,
                                                                                                                                                    end: 522,
                                                                                                                                                    as_str(): "val",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 519,
                                                                                                                                            end: 522,
                                                                                                                                            as_str(): "val",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 525,
                                                                                                                                                    end: 528,
                                                                                                                                                    as_str(): "inc",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 525,
                                                                                                                                            end: 528,
                                                                                                                                            as_str(): "inc",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 519,
                                                                                                                            end: 528,
                                                                                                                            as_str(): "val + inc",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 514,
                                                                                                            end: 529,
                                                                                                            as_str(): "E::Y(val + inc)",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 501,
                                                                                                        end: 530,
                                                                                                        as_str(): "E::X(val) => E::Y(val + inc),",
                                                                                                    },
                                                                                                },
                                                                                                MatchBranch {
                                                                                                    scrutinee: EnumScrutinee {
                                                                                                        call_path: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 543,
                                                                                                                        end: 544,
                                                                                                                        as_str(): "E",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 546,
                                                                                                                    end: 547,
                                                                                                                    as_str(): "Y",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        value: Variable {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 548,
                                                                                                                    end: 551,
                                                                                                                    as_str(): "val",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 548,
                                                                                                                end: 551,
                                                                                                                as_str(): "val",
                                                                                                            },
                                                                                                        },
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 543,
                                                                                                            end: 552,
                                                                                                            as_str(): "E::Y(val)",
                                                                                                        },
                                                                                                    },
                                                                                                    result: Expression {
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
                                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 556,
                                                                                                                                        end: 557,
                                                                                                                                        as_str(): "E",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                type_arguments: [],
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 556,
                                                                                                                                    end: 557,
                                                                                                                                    as_str(): "E",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            suffix: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 559,
                                                                                                                                    end: 560,
                                                                                                                                    as_str(): "X",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        },
                                                                                                                        is_absolute: false,
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 556,
                                                                                                                        end: 560,
                                                                                                                        as_str(): "E::X",
                                                                                                                    },
                                                                                                                },
                                                                                                                args: [
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
                                                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 565,
                                                                                                                                                        end: 566,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                                BaseIdent {
                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                        "ops",
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 565,
                                                                                                                                                        end: 566,
                                                                                                                                                        as_str(): "+",
                                                                                                                                                    },
                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                },
                                                                                                                                            ],
                                                                                                                                            suffix: BaseIdent {
                                                                                                                                                name_override_opt: Some(
                                                                                                                                                    "add",
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 565,
                                                                                                                                                    end: 566,
                                                                                                                                                    as_str(): "+",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            is_absolute: true,
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 565,
                                                                                                                                        end: 566,
                                                                                                                                        as_str(): "+",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                contract_call_params: [],
                                                                                                                                arguments: [
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 561,
                                                                                                                                                    end: 564,
                                                                                                                                                    as_str(): "val",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 561,
                                                                                                                                            end: 564,
                                                                                                                                            as_str(): "val",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    Expression {
                                                                                                                                        kind: Variable(
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 567,
                                                                                                                                                    end: 570,
                                                                                                                                                    as_str(): "inc",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 567,
                                                                                                                                            end: 570,
                                                                                                                                            as_str(): "inc",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                ],
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 561,
                                                                                                                            end: 570,
                                                                                                                            as_str(): "val + inc",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 556,
                                                                                                            end: 571,
                                                                                                            as_str(): "E::X(val + inc)",
                                                                                                        },
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 543,
                                                                                                        end: 572,
                                                                                                        as_str(): "E::Y(val) => E::X(val + inc),",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 476,
                                                                                        end: 582,
                                                                                        as_str(): "match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 476,
                                                                                end: 582,
                                                                                as_str(): "match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 476,
                                                                        end: 582,
                                                                        as_str(): "match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 476,
                                                                end: 582,
                                                                as_str(): "match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 469,
                                                    end: 582,
                                                    as_str(): "self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 469,
                                            end: 582,
                                            as_str(): "self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 459,
                                    end: 588,
                                    as_str(): "{\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 443,
                                            end: 447,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: true,
                                    is_mutable: true,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 435,
                                        end: 442,
                                        as_str(): "ref mut",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 443,
                                        end: 447,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 449,
                                            end: 452,
                                            as_str(): "inc",
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
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 454,
                                        end: 457,
                                        as_str(): "u64",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 430,
                                end: 588,
                                as_str(): "fn j(ref mut self, inc: u64) {\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }",
                            },
                            return_type: Tuple(
                                [],
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe04ae0ff10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                ),
                                start: 430,
                                end: 458,
                                as_str(): "fn j(ref mut self, inc: u64)",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 417,
                        end: 590,
                        as_str(): "impl E {\n    fn j(ref mut self, inc: u64) {\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 417,
            end: 590,
            as_str(): "impl E {\n    fn j(ref mut self, inc: u64) {\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }\n}",
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
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 595,
                            end: 599,
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
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 624,
                                                    end: 625,
                                                    as_str(): "a",
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
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 628,
                                                                        end: 629,
                                                                        as_str(): "A",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 628,
                                                                end: 629,
                                                                as_str(): "A",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 640,
                                                                        end: 641,
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
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 643,
                                                                        end: 644,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 640,
                                                                    end: 644,
                                                                    as_str(): "a: 0",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 654,
                                                                        end: 655,
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
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 657,
                                                                        end: 658,
                                                                        as_str(): "0",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 654,
                                                                    end: 658,
                                                                    as_str(): "b: 0",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 628,
                                                    end: 665,
                                                    as_str(): "A {\n        a: 0,\n        b: 0,\n    }",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 616,
                                    end: 666,
                                    as_str(): "let mut a = A {\n        a: 0,\n        b: 0,\n    };",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 674,
                                                                end: 675,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 674,
                                                        end: 675,
                                                        as_str(): "f",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 672,
                                                                    end: 673,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 672,
                                                            end: 673,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 672,
                                            end: 677,
                                            as_str(): "a.f()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 672,
                                    end: 677,
                                    as_str(): "a.f()",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 683,
                                                                end: 689,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 683,
                                                        end: 689,
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 694,
                                                                                        end: 696,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 694,
                                                                                        end: 696,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 694,
                                                                                    end: 696,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 694,
                                                                        end: 696,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 690,
                                                                                                end: 691,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 690,
                                                                                        end: 691,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 692,
                                                                                        end: 693,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 690,
                                                                            end: 693,
                                                                            as_str(): "a.a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                42,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 697,
                                                                            end: 699,
                                                                            as_str(): "42",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 690,
                                                            end: 699,
                                                            as_str(): "a.a == 42",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 683,
                                            end: 700,
                                            as_str(): "assert(a.a == 42)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 683,
                                    end: 700,
                                    as_str(): "assert(a.a == 42)",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 706,
                                                                end: 712,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 706,
                                                        end: 712,
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 717,
                                                                                        end: 719,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 717,
                                                                                        end: 719,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 717,
                                                                                    end: 719,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 717,
                                                                        end: 719,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 713,
                                                                                                end: 714,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 713,
                                                                                        end: 714,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 715,
                                                                                        end: 716,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 713,
                                                                            end: 716,
                                                                            as_str(): "a.b",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                77,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 720,
                                                                            end: 722,
                                                                            as_str(): "77",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 713,
                                                            end: 722,
                                                            as_str(): "a.b == 77",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 706,
                                            end: 723,
                                            as_str(): "assert(a.b == 77)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 706,
                                    end: 723,
                                    as_str(): "assert(a.b == 77)",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 732,
                                                                end: 733,
                                                                as_str(): "g",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 732,
                                                        end: 733,
                                                        as_str(): "g",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 730,
                                                                    end: 731,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 730,
                                                            end: 731,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                1,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 734,
                                                            end: 735,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 730,
                                            end: 736,
                                            as_str(): "a.g(1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 730,
                                    end: 736,
                                    as_str(): "a.g(1)",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 742,
                                                                end: 748,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 742,
                                                        end: 748,
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 753,
                                                                                        end: 755,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 753,
                                                                                        end: 755,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 753,
                                                                                    end: 755,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 753,
                                                                        end: 755,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 749,
                                                                                                end: 750,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 749,
                                                                                        end: 750,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 751,
                                                                                        end: 752,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 749,
                                                                            end: 752,
                                                                            as_str(): "a.a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                43,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 756,
                                                                            end: 758,
                                                                            as_str(): "43",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 749,
                                                            end: 758,
                                                            as_str(): "a.a == 43",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 742,
                                            end: 759,
                                            as_str(): "assert(a.a == 43)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 742,
                                    end: 759,
                                    as_str(): "assert(a.a == 43)",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 765,
                                                                end: 771,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 765,
                                                        end: 771,
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 776,
                                                                                        end: 778,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 776,
                                                                                        end: 778,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 776,
                                                                                    end: 778,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 776,
                                                                        end: 778,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 772,
                                                                                                end: 773,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 772,
                                                                                        end: 773,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 774,
                                                                                        end: 775,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 772,
                                                                            end: 775,
                                                                            as_str(): "a.b",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                78,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 779,
                                                                            end: 781,
                                                                            as_str(): "78",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 772,
                                                            end: 781,
                                                            as_str(): "a.b == 78",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 765,
                                            end: 782,
                                            as_str(): "assert(a.b == 78)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 765,
                                    end: 782,
                                    as_str(): "assert(a.b == 78)",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 791,
                                                                end: 792,
                                                                as_str(): "h",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 791,
                                                        end: 792,
                                                        as_str(): "h",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 789,
                                                                    end: 790,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 789,
                                                            end: 790,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 789,
                                            end: 794,
                                            as_str(): "a.h()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 789,
                                    end: 794,
                                    as_str(): "a.h()",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 800,
                                                                end: 806,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 800,
                                                        end: 806,
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 811,
                                                                                        end: 813,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 811,
                                                                                        end: 813,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 811,
                                                                                    end: 813,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 811,
                                                                        end: 813,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 807,
                                                                                                end: 808,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 807,
                                                                                        end: 808,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 809,
                                                                                        end: 810,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 807,
                                                                            end: 810,
                                                                            as_str(): "a.a",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                100,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 814,
                                                                            end: 817,
                                                                            as_str(): "100",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 807,
                                                            end: 817,
                                                            as_str(): "a.a == 100",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 800,
                                            end: 818,
                                            as_str(): "assert(a.a == 100)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 800,
                                    end: 818,
                                    as_str(): "assert(a.a == 100)",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 824,
                                                                end: 830,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 824,
                                                        end: 830,
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
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 835,
                                                                                        end: 837,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 835,
                                                                                        end: 837,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 835,
                                                                                    end: 837,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 835,
                                                                        end: 837,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 831,
                                                                                                end: 832,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 831,
                                                                                        end: 832,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 833,
                                                                                        end: 834,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 831,
                                                                            end: 834,
                                                                            as_str(): "a.b",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                200,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 838,
                                                                            end: 841,
                                                                            as_str(): "200",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 831,
                                                            end: 841,
                                                            as_str(): "a.b == 200",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 824,
                                            end: 842,
                                            as_str(): "assert(a.b == 200)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 824,
                                    end: 842,
                                    as_str(): "assert(a.b == 200)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 857,
                                                    end: 858,
                                                    as_str(): "e",
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
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 861,
                                                                                end: 862,
                                                                                as_str(): "E",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 861,
                                                                            end: 862,
                                                                            as_str(): "E",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 864,
                                                                            end: 865,
                                                                            as_str(): "X",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 861,
                                                                end: 865,
                                                                as_str(): "E::X",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        42,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 866,
                                                                    end: 868,
                                                                    as_str(): "42",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 861,
                                                    end: 869,
                                                    as_str(): "E::X(42)",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 849,
                                    end: 870,
                                    as_str(): "let mut e = E::X(42);",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "__match_return_var_name_2",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 881,
                                                                            end: 882,
                                                                            as_str(): "e",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 881,
                                                                                    end: 882,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 881,
                                                                            end: 882,
                                                                            as_str(): "e",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 875,
                                                            end: 938,
                                                            as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "__match_return_var_name_2",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 881,
                                                                                        end: 882,
                                                                                        as_str(): "e",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 881,
                                                                                end: 882,
                                                                                as_str(): "e",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 893,
                                                                                                    end: 894,
                                                                                                    as_str(): "E",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 896,
                                                                                                end: 897,
                                                                                                as_str(): "X",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Literal {
                                                                                        value: Numeric(
                                                                                            42,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 898,
                                                                                            end: 900,
                                                                                            as_str(): "42",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 893,
                                                                                        end: 901,
                                                                                        as_str(): "E::X(42)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 905,
                                                                                                end: 907,
                                                                                                as_str(): "{}",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 905,
                                                                                        end: 907,
                                                                                        as_str(): "{}",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 893,
                                                                                    end: 908,
                                                                                    as_str(): "E::X(42) => {},",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: CatchAll {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 917,
                                                                                        end: 918,
                                                                                        as_str(): "_",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 922,
                                                                                                            end: 928,
                                                                                                            as_str(): "revert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 922,
                                                                                                    end: 928,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            0,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 929,
                                                                                                        end: 930,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 922,
                                                                                        end: 931,
                                                                                        as_str(): "revert(0)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 917,
                                                                                    end: 932,
                                                                                    as_str(): "_ => revert(0),",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 875,
                                                                    end: 938,
                                                                    as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 875,
                                                            end: 938,
                                                            as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 875,
                                                    end: 938,
                                                    as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 875,
                                            end: 938,
                                            as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 875,
                                    end: 938,
                                    as_str(): "match e {\n        E::X(42) => {},\n        _ => revert(0),\n    }",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 951,
                                                                end: 952,
                                                                as_str(): "j",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 951,
                                                        end: 952,
                                                        as_str(): "j",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 949,
                                                                    end: 950,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 949,
                                                            end: 950,
                                                            as_str(): "e",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                4,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 953,
                                                            end: 954,
                                                            as_str(): "4",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 949,
                                            end: 955,
                                            as_str(): "e.j(4)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 949,
                                    end: 955,
                                    as_str(): "e.j(4)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "__match_return_var_name_3",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 967,
                                                                            end: 968,
                                                                            as_str(): "e",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 967,
                                                                                    end: 968,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 967,
                                                                            end: 968,
                                                                            as_str(): "e",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 961,
                                                            end: 1024,
                                                            as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "__match_return_var_name_3",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 967,
                                                                                        end: 968,
                                                                                        as_str(): "e",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 967,
                                                                                end: 968,
                                                                                as_str(): "e",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 979,
                                                                                                    end: 980,
                                                                                                    as_str(): "E",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 982,
                                                                                                end: 983,
                                                                                                as_str(): "Y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Literal {
                                                                                        value: Numeric(
                                                                                            46,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 984,
                                                                                            end: 986,
                                                                                            as_str(): "46",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 979,
                                                                                        end: 987,
                                                                                        as_str(): "E::Y(46)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 991,
                                                                                                end: 993,
                                                                                                as_str(): "{}",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 991,
                                                                                        end: 993,
                                                                                        as_str(): "{}",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 979,
                                                                                    end: 994,
                                                                                    as_str(): "E::Y(46) => {},",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: CatchAll {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1003,
                                                                                        end: 1004,
                                                                                        as_str(): "_",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1008,
                                                                                                            end: 1014,
                                                                                                            as_str(): "revert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1008,
                                                                                                    end: 1014,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            0,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1015,
                                                                                                        end: 1016,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1008,
                                                                                        end: 1017,
                                                                                        as_str(): "revert(0)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1003,
                                                                                    end: 1018,
                                                                                    as_str(): "_ => revert(0),",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 961,
                                                                    end: 1024,
                                                                    as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 961,
                                                            end: 1024,
                                                            as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 961,
                                                    end: 1024,
                                                    as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 961,
                                            end: 1024,
                                            as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 961,
                                    end: 1024,
                                    as_str(): "match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 1033,
                                                                end: 1034,
                                                                as_str(): "j",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 1033,
                                                        end: 1034,
                                                        as_str(): "j",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 1031,
                                                                    end: 1032,
                                                                    as_str(): "e",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1031,
                                                            end: 1032,
                                                            as_str(): "e",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                5,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1035,
                                                            end: 1036,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 1031,
                                            end: 1037,
                                            as_str(): "e.j(5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 1031,
                                    end: 1037,
                                    as_str(): "e.j(5)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "__match_return_var_name_4",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 1049,
                                                                            end: 1050,
                                                                            as_str(): "e",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1049,
                                                                                    end: 1050,
                                                                                    as_str(): "e",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 1049,
                                                                            end: 1050,
                                                                            as_str(): "e",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1043,
                                                            end: 1106,
                                                            as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "__match_return_var_name_4",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1049,
                                                                                        end: 1050,
                                                                                        as_str(): "e",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1049,
                                                                                end: 1050,
                                                                                as_str(): "e",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1061,
                                                                                                    end: 1062,
                                                                                                    as_str(): "E",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 1064,
                                                                                                end: 1065,
                                                                                                as_str(): "X",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Literal {
                                                                                        value: Numeric(
                                                                                            51,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 1066,
                                                                                            end: 1068,
                                                                                            as_str(): "51",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1061,
                                                                                        end: 1069,
                                                                                        as_str(): "E::X(51)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 1073,
                                                                                                end: 1075,
                                                                                                as_str(): "{}",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1073,
                                                                                        end: 1075,
                                                                                        as_str(): "{}",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1061,
                                                                                    end: 1076,
                                                                                    as_str(): "E::X(51) => {},",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: CatchAll {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1085,
                                                                                        end: 1086,
                                                                                        as_str(): "_",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1090,
                                                                                                            end: 1096,
                                                                                                            as_str(): "revert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1090,
                                                                                                    end: 1096,
                                                                                                    as_str(): "revert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Numeric(
                                                                                                            0,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1097,
                                                                                                        end: 1098,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1090,
                                                                                        end: 1099,
                                                                                        as_str(): "revert(0)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1085,
                                                                                    end: 1100,
                                                                                    as_str(): "_ => revert(0),",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 1043,
                                                                    end: 1106,
                                                                    as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1043,
                                                            end: 1106,
                                                            as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 1043,
                                                    end: 1106,
                                                    as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 1043,
                                            end: 1106,
                                            as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 1043,
                                    end: 1106,
                                    as_str(): "match e {\n        E::X(51) => {},\n        _ => revert(0),\n    }",
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
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 1116,
                                            end: 1120,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe04ae0ff10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                    ),
                                    start: 1116,
                                    end: 1120,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 610,
                            end: 1122,
                            as_str(): "{\n    let mut a = A {\n        a: 0,\n        b: 0,\n    };\n\n    a.f();\n    assert(a.a == 42);\n    assert(a.b == 77);\n\n    a.g(1);\n    assert(a.a == 43);\n    assert(a.b == 78);\n\n    a.h();\n    assert(a.a == 100);\n    assert(a.b == 200);\n\n    let mut e = E::X(42);\n    match e {\n        E::X(42) => {},\n        _ => revert(0),\n    };\n    \n    e.j(4);\n    match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    };\n\n    e.j(5);\n    match e {\n        E::X(51) => {},\n        _ => revert(0),\n    };\n   \n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 592,
                        end: 1122,
                        as_str(): "fn main() -> bool {\n    let mut a = A {\n        a: 0,\n        b: 0,\n    };\n\n    a.f();\n    assert(a.a == 42);\n    assert(a.b == 77);\n\n    a.g(1);\n    assert(a.a == 43);\n    assert(a.b == 78);\n\n    a.h();\n    assert(a.a == 100);\n    assert(a.b == 200);\n\n    let mut e = E::X(42);\n    match e {\n        E::X(42) => {},\n        _ => revert(0),\n    };\n    \n    e.j(4);\n    match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    };\n\n    e.j(5);\n    match e {\n        E::X(51) => {},\n        _ => revert(0),\n    };\n   \n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 605,
                        end: 609,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe04ae0ff10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
            ),
            start: 592,
            end: 1122,
            as_str(): "fn main() -> bool {\n    let mut a = A {\n        a: 0,\n        b: 0,\n    };\n\n    a.f();\n    assert(a.a == 42);\n    assert(a.b == 77);\n\n    a.g(1);\n    assert(a.a == 43);\n    assert(a.b == 78);\n\n    a.h();\n    assert(a.a == 100);\n    assert(a.b == 200);\n\n    let mut e = E::X(42);\n    match e {\n        E::X(42) => {},\n        _ => revert(0),\n    };\n    \n    e.j(4);\n    match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    };\n\n    e.j(5);\n    match e {\n        E::X(51) => {},\n        _ => revert(0),\n    };\n   \n    true\n}",
        },
    },
]
