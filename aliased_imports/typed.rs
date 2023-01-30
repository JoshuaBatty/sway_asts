

TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb13db7f560,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
            ),
            start: 92,
            end: 96,
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
                                    src (ptr): 0x00007fb13db7f560,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                    ),
                                    start: 116,
                                    end: 119,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13c982ff0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                            ),
                                            start: 25,
                                            end: 28,
                                            as_str(): "Foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13db7f560,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                    ),
                                                    start: 138,
                                                    end: 141,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        42,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13db7f560,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                                    ),
                                                    start: 143,
                                                    end: 145,
                                                    as_str(): "42",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb13db7f560,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                        ),
                                        start: 122,
                                        end: 127,
                                        as_str(): "MyFoo",
                                    },
                                },
                                return_type: TypeId(
                                    31632,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13db7f560,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                    ),
                                    start: 122,
                                    end: 152,
                                    as_str(): "MyFoo {\n        foo: 42,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31632,
                            ),
                            type_ascription: TypeId(
                                31634,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13db7f560,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                    ),
                    start: 112,
                    end: 153,
                    as_str(): "let foo = MyFoo {\n        foo: 42,\n    };",
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
                                            src (ptr): 0x00007fb13db7f560,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                            ),
                                            start: 116,
                                            end: 119,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb13db7f560,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 161,
                                        as_str(): "foo",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    31632,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13db7f560,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                    ),
                                    start: 158,
                                    end: 161,
                                    as_str(): "foo",
                                },
                            },
                            field_to_access: TyStructField {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb13c982ff0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                        ),
                                        start: 35,
                                        end: 38,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13c982ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                    ),
                                    start: 35,
                                    end: 43,
                                    as_str(): "foo: u64",
                                },
                                type_span: Span {
                                    src (ptr): 0x00007fb13c982ff0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/a_dependency.sw",
                                    ),
                                    start: 40,
                                    end: 43,
                                    as_str(): "u64",
                                },
                                attributes: {},
                            },
                            field_instantiation_span: Span {
                                src (ptr): 0x00007fb13db7f560,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                                ),
                                start: 162,
                                end: 165,
                                as_str(): "foo",
                            },
                            resolved_type_of_parent: TypeId(
                                31632,
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb13db7f560,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                            ),
                            start: 158,
                            end: 165,
                            as_str(): "foo.foo",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb13db7f560,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
                    ),
                    start: 158,
                    end: 165,
                    as_str(): "foo.foo",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb13db7f560,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
        ),
        start: 89,
        end: 167,
        as_str(): "fn main() -> u64 {\n    let foo = MyFoo {\n        foo: 42,\n    };\n    foo.foo\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb13db7f560,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRIgAdMI/aliased_imports/src/main.sw",
        ),
        start: 102,
        end: 105,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

