TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f9b4a470,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
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
                    src (ptr): 0x00007fe0f9b4a470,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                    ),
                    start: 27,
                    end: 28,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7254,
            ),
            initial_type_id: TypeId(
                7255,
            ),
            span: Span {
                src (ptr): 0x00007fe0f9b4a470,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                ),
                start: 27,
                end: 31,
                as_str(): "a: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0f9b4a470,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                ),
                start: 30,
                end: 31,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7254),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0f9b4a470,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
        ),
        start: 9,
        end: 34,
        as_str(): "struct Foo<T> {\n  a: T,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f9b4a470,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
            ),
            start: 39,
            end: 44,
            as_str(): "get_a",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructFieldAccess {
                            prefix: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 51,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 70,
                                        end: 73,
                                        as_str(): "foo",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    7259,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 70,
                                    end: 73,
                                    as_str(): "foo",
                                },
                            },
                            field_to_access: TyStructField {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 28,
                                        as_str(): "a",
                                    },
                                    is_raw_ident: false,
                                },
                                type_id: TypeId(
                                    7257,
                                ),
                                initial_type_id: TypeId(
                                    7255,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 27,
                                    end: 31,
                                    as_str(): "a: T",
                                },
                                type_span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 30,
                                    end: 31,
                                    as_str(): "T",
                                },
                                attributes: {},
                            },
                            field_instantiation_span: Span {
                                src (ptr): 0x00007fe0f9b4a470,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                ),
                                start: 74,
                                end: 75,
                                as_str(): "a",
                            },
                            resolved_type_of_parent: TypeId(
                                7259,
                            ),
                        },
                        return_type: TypeId(
                            7257,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f9b4a470,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                            ),
                            start: 70,
                            end: 75,
                            as_str(): "foo.a",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f9b4a470,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                    ),
                    start: 70,
                    end: 75,
                    as_str(): "foo.a",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f9b4a470,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                    ),
                    start: 48,
                    end: 51,
                    as_str(): "foo",
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
            type_id: TypeId(
                7259,
            ),
            initial_type_id: TypeId(
                7258,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f9b4a470,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                ),
                start: 53,
                end: 59,
                as_str(): "Foo<V>",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f9b4a470,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
        ),
        start: 36,
        end: 77,
        as_str(): "fn get_a<V>(foo: Foo<V>) -> V {\n  foo.a\n}",
    },
    attributes: {},
    return_type: TypeId(
        7257,
    ),
    initial_return_type: TypeId(
        7260,
    ),
    type_parameters: [
        V: TypeId(7257),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0f9b4a470,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
        ),
        start: 64,
        end: 65,
        as_str(): "V",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f9b4a470,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
            ),
            start: 82,
            end: 86,
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
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 105,
                                    end: 108,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
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
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 117,
                                                    end: 118,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 120,
                                                    end: 124,
                                                    as_str(): "true",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 111,
                                        end: 114,
                                        as_str(): "Foo",
                                    },
                                },
                                return_type: TypeId(
                                    7266,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 111,
                                    end: 126,
                                    as_str(): "Foo { a: true }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7266,
                            ),
                            type_ascription: TypeId(
                                7263,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f9b4a470,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                    ),
                    start: 101,
                    end: 127,
                    as_str(): "let foo = Foo { a: true };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 134,
                                    end: 137,
                                    as_str(): "bar",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
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
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 146,
                                                    end: 147,
                                                    as_str(): "a",
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
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 149,
                                                    end: 151,
                                                    as_str(): "10",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 140,
                                        end: 143,
                                        as_str(): "Foo",
                                    },
                                },
                                return_type: TypeId(
                                    7271,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 140,
                                    end: 153,
                                    as_str(): "Foo { a: 10 }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7271,
                            ),
                            type_ascription: TypeId(
                                7268,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f9b4a470,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                    ),
                    start: 130,
                    end: 154,
                    as_str(): "let bar = Foo { a: 10 };",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 163,
                                        as_str(): "get_a",
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
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 51,
                                            as_str(): "foo",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f9b4a470,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 108,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f9b4a470,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                                ),
                                                start: 164,
                                                end: 167,
                                                as_str(): "foo",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7266,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f9b4a470,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                            ),
                                            start: 164,
                                            end: 167,
                                            as_str(): "foo",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                547,
                                Span {
                                    src (ptr): 0x00007fe0f9b4a470,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 77,
                                    as_str(): "fn get_a<V>(foo: Foo<V>) -> V {\n  foo.a\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f9b4a470,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 163,
                                        as_str(): "get_a",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7274,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f9b4a470,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                            ),
                            start: 158,
                            end: 168,
                            as_str(): "get_a(foo)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f9b4a470,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
                    ),
                    start: 158,
                    end: 168,
                    as_str(): "get_a(foo)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f9b4a470,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
        ),
        start: 79,
        end: 170,
        as_str(): "fn main() -> bool {\n  let foo = Foo { a: true };\n  let bar = Foo { a: 10 };\n\n  get_a(foo)\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0f9b4a470,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6KRo5n/generic_struct/src/main.sw",
        ),
        start: 92,
        end: 96,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

