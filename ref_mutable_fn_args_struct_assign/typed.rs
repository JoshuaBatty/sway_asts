TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0629e5bd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                    src (ptr): 0x00007fe0629e5bd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                src (ptr): 0x00007fe0629e5bd0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                ),
                start: 26,
                end: 36,
                as_str(): "value: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0629e5bd0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
        src (ptr): 0x00007fe0629e5bd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
        ),
        start: 9,
        end: 38,
        as_str(): "struct Foo {\n    value: u64\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0629e5bd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
            ),
            start: 43,
            end: 50,
            as_str(): "mut_foo",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Reassignment(
                            TyReassignment {
                                lhs_base_name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 75,
                                        end: 78,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    5,
                                ),
                                lhs_indices: [],
                                rhs: TyExpression {
                                    expression: StructExpression {
                                        struct_name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0629e5bd0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                                        src (ptr): 0x00007fe0629e5bd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                        ),
                                                        start: 87,
                                                        end: 92,
                                                        as_str(): "value",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                value: TyExpression {
                                                    expression: Literal(
                                                        U64(
                                                            10,
                                                        ),
                                                    ),
                                                    return_type: TypeId(
                                                        2,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0629e5bd0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                        ),
                                                        start: 94,
                                                        end: 96,
                                                        as_str(): "10",
                                                    },
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 84,
                                            as_str(): "Foo",
                                        },
                                    },
                                    return_type: TypeId(
                                        5,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 81,
                                        end: 98,
                                        as_str(): "Foo { value: 10 }",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            14,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                            ),
                            start: 75,
                            end: 98,
                            as_str(): "foo = Foo { value: 10 }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0629e5bd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                    ),
                    start: 75,
                    end: 98,
                    as_str(): "foo = Foo { value: 10 }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0629e5bd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                    ),
                    start: 59,
                    end: 62,
                    as_str(): "foo",
                },
                is_raw_ident: false,
            },
            is_reference: true,
            is_mutable: true,
            mutability_span: Span {
                src (ptr): 0x00007fe0629e5bd0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                ),
                start: 51,
                end: 58,
                as_str(): "ref mut",
            },
            type_id: TypeId(
                5,
            ),
            initial_type_id: TypeId(
                4,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0629e5bd0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                ),
                start: 64,
                end: 67,
                as_str(): "Foo",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0629e5bd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
        ),
        start: 40,
        end: 101,
        as_str(): "fn mut_foo(ref mut foo: Foo) {\n    foo = Foo { value: 10 };\n}",
    },
    attributes: {},
    return_type: TypeId(
        7,
    ),
    initial_return_type: TypeId(
        6,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0629e5bd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
        ),
        start: 40,
        end: 68,
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
            src (ptr): 0x00007fe0629e5bd0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
            ),
            start: 106,
            end: 110,
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
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 134,
                                    end: 137,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                                    src (ptr): 0x00007fe0629e5bd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                    ),
                                                    start: 146,
                                                    end: 151,
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
                                                    src (ptr): 0x00007fe0629e5bd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                    ),
                                                    start: 153,
                                                    end: 154,
                                                    as_str(): "0",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 140,
                                        end: 143,
                                        as_str(): "Foo",
                                    },
                                },
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 140,
                                    end: 156,
                                    as_str(): "Foo { value: 0 }",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                5,
                            ),
                            type_ascription: TypeId(
                                17,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0629e5bd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                    ),
                    start: 126,
                    end: 157,
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
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 162,
                                        end: 169,
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
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 59,
                                            end: 62,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0629e5bd0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                    ),
                                                    start: 134,
                                                    end: 137,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0629e5bd0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                                ),
                                                start: 170,
                                                end: 173,
                                                as_str(): "foo",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            5,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 170,
                                            end: 173,
                                            as_str(): "foo",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                3,
                                Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 40,
                                    end: 101,
                                    as_str(): "fn mut_foo(ref mut foo: Foo) {\n    foo = Foo { value: 10 };\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 162,
                                        end: 169,
                                        as_str(): "mut_foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            23,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                            ),
                            start: 162,
                            end: 174,
                            as_str(): "mut_foo(foo)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0629e5bd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                    ),
                    start: 162,
                    end: 174,
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
                                            src (ptr): 0x00007fe0629e5bd0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                            ),
                                            start: 134,
                                            end: 137,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                        ),
                                        start: 180,
                                        end: 183,
                                        as_str(): "foo",
                                    },
                                    mutability: Mutable,
                                },
                                return_type: TypeId(
                                    5,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 180,
                                    end: 183,
                                    as_str(): "foo",
                                },
                            },
                            field_to_access: TyStructField {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0629e5bd0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
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
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 26,
                                    end: 36,
                                    as_str(): "value: u64",
                                },
                                type_span: Span {
                                    src (ptr): 0x00007fe0629e5bd0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                    ),
                                    start: 33,
                                    end: 36,
                                    as_str(): "u64",
                                },
                                attributes: {},
                            },
                            field_instantiation_span: Span {
                                src (ptr): 0x00007fe0629e5bd0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                                ),
                                start: 184,
                                end: 189,
                                as_str(): "value",
                            },
                            resolved_type_of_parent: TypeId(
                                5,
                            ),
                        },
                        return_type: TypeId(
                            2,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0629e5bd0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                            ),
                            start: 180,
                            end: 189,
                            as_str(): "foo.value",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0629e5bd0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
                    ),
                    start: 180,
                    end: 189,
                    as_str(): "foo.value",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0629e5bd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
        ),
        start: 103,
        end: 191,
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
        src (ptr): 0x00007fe0629e5bd0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRI9g40a/ref_mutable_fn_args_struct_assign/src/main.sw",
        ),
        start: 116,
        end: 119,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

