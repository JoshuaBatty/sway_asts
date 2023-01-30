TyStructDeclaration {
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
    fields: [
        TyStructField {
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
            type_id: TypeId(
                2,
            ),
            initial_type_id: TypeId(
                2,
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
            attributes: {},
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
    attributes: {},
}
ImplTrait(
    DeclId(
        2,
        Span {
            src (ptr): 0x00007fe05c5b18a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
            ),
            start: 40,
            end: 155,
            as_str(): "impl Foo {\n    pub fn set(ref mut self, value: u64) -> u64 {\n        self.value = value;\n        self.value\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
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
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: VariableExpression {
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
                                            span: Span {
                                                src (ptr): 0x00007fe05c5b18a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                ),
                                                start: 192,
                                                end: 195,
                                                as_str(): "foo",
                                            },
                                            mutability: RefMutable,
                                        },
                                        return_type: TypeId(
                                            4,
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
                                ),
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: Literal(
                                            U64(
                                                10,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            2,
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
                                ),
                            ],
                            function_decl_id: DeclId(
                                3,
                                Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 55,
                                    end: 153,
                                    as_str(): "pub fn set(ref mut self, value: u64) -> u64 {\n        self.value = value;\n        self.value\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            2,
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
    },
    parameters: [
        TyFunctionParameter {
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
            type_id: TypeId(
                4,
            ),
            initial_type_id: TypeId(
                14,
            ),
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
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05c5b18a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
        ),
        start: 157,
        end: 206,
        as_str(): "fn mut_foo(ref mut foo: Foo) {\n    foo.set(10);\n}",
    },
    attributes: {},
    return_type: TypeId(
        16,
    ),
    initial_return_type: TypeId(
        15,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
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
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
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
                                    fields: [
                                        TyStructExpressionField {
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
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    2,
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
                                        },
                                    ],
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
                                return_type: TypeId(
                                    4,
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
                            mutability: Mutable,
                            return_type: TypeId(
                                4,
                            ),
                            type_ascription: TypeId(
                                23,
                            ),
                            type_ascription_span: None,
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
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
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
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
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
                                    TyExpression {
                                        expression: VariableExpression {
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
                                            span: Span {
                                                src (ptr): 0x00007fe05c5b18a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                ),
                                                start: 275,
                                                end: 278,
                                                as_str(): "foo",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            4,
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
                                ),
                            ],
                            function_decl_id: DeclId(
                                6,
                                Span {
                                    src (ptr): 0x00007fe05c5b18a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                    ),
                                    start: 157,
                                    end: 206,
                                    as_str(): "fn mut_foo(ref mut foo: Foo) {\n    foo.set(10);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
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
                            ),
                        },
                        return_type: TypeId(
                            29,
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructFieldAccess {
                            prefix: TyExpression {
                                expression: VariableExpression {
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
                                    span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 285,
                                        end: 288,
                                        as_str(): "foo",
                                    },
                                    mutability: Mutable,
                                },
                                return_type: TypeId(
                                    4,
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
                            field_to_access: TyStructField {
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
                                type_id: TypeId(
                                    2,
                                ),
                                initial_type_id: TypeId(
                                    2,
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
                                attributes: {},
                            },
                            field_instantiation_span: Span {
                                src (ptr): 0x00007fe05c5b18a0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                ),
                                start: 289,
                                end: 294,
                                as_str(): "value",
                            },
                            resolved_type_of_parent: TypeId(
                                4,
                            ),
                        },
                        return_type: TypeId(
                            2,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe05c5b18a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
        ),
        start: 208,
        end: 296,
        as_str(): "fn main() -> u64 {\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
    },
    attributes: {},
    return_type: TypeId(
        2,
    ),
    initial_return_type: TypeId(
        2,
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
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

