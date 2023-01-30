


TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007f8a1d1ebd20,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
            ),
            start: 67,
            end: 71,
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
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 91,
                                    end: 92,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a124e2460,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/a_dependency.sw",
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
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 114,
                                                    end: 117,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d1ebd20,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 123,
                                                    as_str(): "1u32",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 100,
                                        end: 103,
                                        as_str(): "Foo",
                                    },
                                },
                                return_type: TypeId(
                                    7261,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 130,
                                    as_str(): "foo::Foo {\n        foo: 1u32,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7261,
                            ),
                            type_ascription: TypeId(
                                7259,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 87,
                    end: 131,
                    as_str(): "let x = foo::Foo {\n        foo: 1u32,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 140,
                                    end: 141,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007f8a1e1b3da0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                ),
                                                start: 19,
                                                end: 22,
                                                as_str(): "Bar",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1e1b3da0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                        ),
                                                        start: 29,
                                                        end: 32,
                                                        as_str(): "Baz",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    71,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007f8a1e1b3da0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                    ),
                                                    start: 34,
                                                    end: 38,
                                                    as_str(): "bool",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1e1b3da0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                    ),
                                                    start: 29,
                                                    end: 38,
                                                    as_str(): "Baz: bool",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007f8a1e1b3da0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                            ),
                                            start: 14,
                                            end: 41,
                                            as_str(): "enum Bar {\n    Baz: bool,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1e1b3da0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                            ),
                                            start: 29,
                                            end: 32,
                                            as_str(): "Baz",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                Boolean(
                                                    true,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a1d1ebd20,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                ),
                                                start: 158,
                                                end: 162,
                                                as_str(): "true",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 144,
                                        end: 147,
                                        as_str(): "bar",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 154,
                                        end: 157,
                                        as_str(): "Baz",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 144,
                                            end: 157,
                                            as_str(): "bar::Bar::Baz",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7264,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 154,
                                    end: 157,
                                    as_str(): "Baz",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7264,
                            ),
                            type_ascription: TypeId(
                                7263,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 136,
                    end: 164,
                    as_str(): "let y = bar::Bar::Baz(true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 173,
                                    end: 174,
                                    as_str(): "z",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007f8a1e1b3da0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                ),
                                                start: 19,
                                                end: 22,
                                                as_str(): "Bar",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1e1b3da0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                        ),
                                                        start: 29,
                                                        end: 32,
                                                        as_str(): "Baz",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    71,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007f8a1e1b3da0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                    ),
                                                    start: 34,
                                                    end: 38,
                                                    as_str(): "bool",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007f8a1e1b3da0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                                    ),
                                                    start: 29,
                                                    end: 38,
                                                    as_str(): "Baz: bool",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007f8a1e1b3da0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                            ),
                                            start: 14,
                                            end: 41,
                                            as_str(): "enum Bar {\n    Baz: bool,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1e1b3da0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/b_dependency.sw",
                                            ),
                                            start: 29,
                                            end: 32,
                                            as_str(): "Baz",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                Boolean(
                                                    false,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007f8a1d1ebd20,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                                ),
                                                start: 193,
                                                end: 198,
                                                as_str(): "false",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 179,
                                        end: 182,
                                        as_str(): "bar",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 189,
                                        end: 192,
                                        as_str(): "Baz",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 179,
                                            end: 192,
                                            as_str(): "bar::Bar::Baz",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7264,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d1ebd20,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                    ),
                                    start: 189,
                                    end: 192,
                                    as_str(): "Baz",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7264,
                            ),
                            type_ascription: TypeId(
                                7266,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 169,
                    end: 200,
                    as_str(): "let z = ::bar::Bar::Baz(false);",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d1ebd20,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                            ),
                                            start: 205,
                                            end: 208,
                                            as_str(): "baz",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 210,
                                        end: 218,
                                        as_str(): "return_1",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                551,
                                Span {
                                    src (ptr): 0x00007f8a1933faa0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/c_dependency.sw",
                                    ),
                                    start: 14,
                                    end: 47,
                                    as_str(): "fn return_1() -> u32 {\n    1u32\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007f8a1d1ebd20,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                                        ),
                                        start: 205,
                                        end: 218,
                                        as_str(): "baz::return_1",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            33,
                        ),
                        span: Span {
                            src (ptr): 0x00007f8a1d1ebd20,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                            ),
                            start: 205,
                            end: 220,
                            as_str(): "baz::return_1()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007f8a1d1ebd20,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
                    ),
                    start: 205,
                    end: 220,
                    as_str(): "baz::return_1()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007f8a1d1ebd20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
        ),
        start: 64,
        end: 222,
        as_str(): "fn main() -> u64 {\n    let x = foo::Foo {\n        foo: 1u32,\n    };\n    let y = bar::Bar::Baz(true);\n    let z = ::bar::Bar::Baz(false);\n    baz::return_1()\n}",
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
        src (ptr): 0x00007f8a1d1ebd20,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRCzAbi2/use_full_path_names/src/main.sw",
        ),
        start: 77,
        end: 80,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

