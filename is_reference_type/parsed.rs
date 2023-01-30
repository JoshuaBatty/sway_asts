[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
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
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 18,
                            end: 24,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 26,
                            end: 32,
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
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 9,
            end: 33,
            as_str(): "use std::assert::assert;",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 42,
                            end: 43,
                            as_str(): "S",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 50,
                                    end: 51,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe0c08fd1d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                ),
                                start: 50,
                                end: 56,
                                as_str(): "a: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0c08fd1d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                ),
                                start: 53,
                                end: 56,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0c08fd1d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                        ),
                        start: 35,
                        end: 59,
                        as_str(): "struct S {\n    a: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 35,
            end: 59,
            as_str(): "struct S {\n    a: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 66,
                            end: 67,
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
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 74,
                                    end: 81,
                                    as_str(): "Variant",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe0c08fd1d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                ),
                                start: 83,
                                end: 85,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe0c08fd1d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                ),
                                start: 74,
                                end: 85,
                                as_str(): "Variant: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0c08fd1d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                        ),
                        start: 61,
                        end: 88,
                        as_str(): "enum E {\n    Variant: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 61,
            end: 88,
            as_str(): "enum E {\n    Variant: (),\n}",
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
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 93,
                            end: 109,
                            as_str(): "arg_is_reference",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 133,
                                                        end: 152,
                                                        as_str(): "__is_reference_type",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: IsReferenceType,
                                                    type_arguments: [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                31628,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                31628,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 155,
                                                                end: 156,
                                                                as_str(): "T",
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 133,
                                                        end: 159,
                                                        as_str(): "__is_reference_type::<T>()",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 133,
                                            end: 159,
                                            as_str(): "__is_reference_type::<T>()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 133,
                                    end: 159,
                                    as_str(): "__is_reference_type::<T>()",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 127,
                            end: 161,
                            as_str(): "{\n    __is_reference_type::<T>()\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 113,
                                    end: 114,
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
                                        src (ptr): 0x00007fe0c08fd1d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                        ),
                                        start: 116,
                                        end: 117,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0c08fd1d0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                ),
                                start: 116,
                                end: 117,
                                as_str(): "T",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0c08fd1d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                        ),
                        start: 90,
                        end: 161,
                        as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [
                        T: TypeId(31629),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c08fd1d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                        ),
                        start: 122,
                        end: 126,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 90,
            end: 161,
            as_str(): "fn arg_is_reference<T>(a: T) -> bool {\n    __is_reference_type::<T>()\n}",
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
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 166,
                            end: 170,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 187,
                                                                end: 193,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 187,
                                                        end: 193,
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
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 194,
                                                                                        end: 195,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 194,
                                                                                        end: 195,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "not",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 194,
                                                                                    end: 195,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 194,
                                                                        end: 195,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 195,
                                                                                        end: 214,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: IsReferenceType,
                                                                                    type_arguments: [
                                                                                        TypeArgument {
                                                                                            type_id: TypeId(
                                                                                                31630,
                                                                                            ),
                                                                                            initial_type_id: TypeId(
                                                                                                31630,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 217,
                                                                                                end: 219,
                                                                                                as_str(): "()",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 195,
                                                                                        end: 222,
                                                                                        as_str(): "__is_reference_type::<()>()",
                                                                                    },
                                                                                },
                                                                                arguments: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 195,
                                                                            end: 222,
                                                                            as_str(): "__is_reference_type::<()>()",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 194,
                                                            end: 222,
                                                            as_str(): "!__is_reference_type::<()>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 187,
                                            end: 223,
                                            as_str(): "assert(!__is_reference_type::<()>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 187,
                                    end: 223,
                                    as_str(): "assert(!__is_reference_type::<()>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 259,
                                                                end: 265,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 259,
                                                        end: 265,
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
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 266,
                                                                                        end: 267,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 266,
                                                                                        end: 267,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "not",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 266,
                                                                                    end: 267,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 266,
                                                                        end: 267,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 286,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: IsReferenceType,
                                                                                    type_arguments: [
                                                                                        TypeArgument {
                                                                                            type_id: TypeId(
                                                                                                71,
                                                                                            ),
                                                                                            initial_type_id: TypeId(
                                                                                                71,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 289,
                                                                                                end: 293,
                                                                                                as_str(): "bool",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 267,
                                                                                        end: 296,
                                                                                        as_str(): "__is_reference_type::<bool>()",
                                                                                    },
                                                                                },
                                                                                arguments: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 267,
                                                                            end: 296,
                                                                            as_str(): "__is_reference_type::<bool>()",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 266,
                                                            end: 296,
                                                            as_str(): "!__is_reference_type::<bool>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 259,
                                            end: 297,
                                            as_str(): "assert(!__is_reference_type::<bool>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 259,
                                    end: 297,
                                    as_str(): "assert(!__is_reference_type::<bool>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 303,
                                                                end: 309,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 303,
                                                        end: 309,
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
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 310,
                                                                                        end: 311,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 310,
                                                                                        end: 311,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "not",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 310,
                                                                                    end: 311,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 310,
                                                                        end: 311,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: IntrinsicFunction(
                                                                            IntrinsicFunctionExpression {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 311,
                                                                                        end: 330,
                                                                                        as_str(): "__is_reference_type",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                kind_binding: TypeBinding {
                                                                                    inner: IsReferenceType,
                                                                                    type_arguments: [
                                                                                        TypeArgument {
                                                                                            type_id: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            initial_type_id: TypeId(
                                                                                                21,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 333,
                                                                                                end: 336,
                                                                                                as_str(): "u64",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 311,
                                                                                        end: 339,
                                                                                        as_str(): "__is_reference_type::<u64>()",
                                                                                    },
                                                                                },
                                                                                arguments: [],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 311,
                                                                            end: 339,
                                                                            as_str(): "__is_reference_type::<u64>()",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 310,
                                                            end: 339,
                                                            as_str(): "!__is_reference_type::<u64>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 303,
                                            end: 340,
                                            as_str(): "assert(!__is_reference_type::<u64>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 303,
                                    end: 340,
                                    as_str(): "assert(!__is_reference_type::<u64>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 347,
                                                                end: 353,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 347,
                                                        end: 353,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: IntrinsicFunction(
                                                            IntrinsicFunctionExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 354,
                                                                        end: 373,
                                                                        as_str(): "__is_reference_type",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                kind_binding: TypeBinding {
                                                                    inner: IsReferenceType,
                                                                    type_arguments: [
                                                                        TypeArgument {
                                                                            type_id: TypeId(
                                                                                31631,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                31631,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 376,
                                                                                end: 382,
                                                                                as_str(): "str[1]",
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 354,
                                                                        end: 385,
                                                                        as_str(): "__is_reference_type::<str[1]>()",
                                                                    },
                                                                },
                                                                arguments: [],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 354,
                                                            end: 385,
                                                            as_str(): "__is_reference_type::<str[1]>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 347,
                                            end: 386,
                                            as_str(): "assert(__is_reference_type::<str[1]>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 347,
                                    end: 386,
                                    as_str(): "assert(__is_reference_type::<str[1]>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 392,
                                                                end: 398,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 392,
                                                        end: 398,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: IntrinsicFunction(
                                                            IntrinsicFunctionExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 399,
                                                                        end: 418,
                                                                        as_str(): "__is_reference_type",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                kind_binding: TypeBinding {
                                                                    inner: IsReferenceType,
                                                                    type_arguments: [
                                                                        TypeArgument {
                                                                            type_id: TypeId(
                                                                                59,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                59,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 421,
                                                                                end: 425,
                                                                                as_str(): "b256",
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 399,
                                                                        end: 428,
                                                                        as_str(): "__is_reference_type::<b256>()",
                                                                    },
                                                                },
                                                                arguments: [],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 399,
                                                            end: 428,
                                                            as_str(): "__is_reference_type::<b256>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 392,
                                            end: 429,
                                            as_str(): "assert(__is_reference_type::<b256>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 392,
                                    end: 429,
                                    as_str(): "assert(__is_reference_type::<b256>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 435,
                                                                end: 441,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 435,
                                                        end: 441,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: IntrinsicFunction(
                                                            IntrinsicFunctionExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 442,
                                                                        end: 461,
                                                                        as_str(): "__is_reference_type",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                kind_binding: TypeBinding {
                                                                    inner: IsReferenceType,
                                                                    type_arguments: [
                                                                        TypeArgument {
                                                                            type_id: TypeId(
                                                                                31632,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                31632,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 464,
                                                                                end: 465,
                                                                                as_str(): "S",
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 442,
                                                                        end: 468,
                                                                        as_str(): "__is_reference_type::<S>()",
                                                                    },
                                                                },
                                                                arguments: [],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 442,
                                                            end: 468,
                                                            as_str(): "__is_reference_type::<S>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 435,
                                            end: 469,
                                            as_str(): "assert(__is_reference_type::<S>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 435,
                                    end: 469,
                                    as_str(): "assert(__is_reference_type::<S>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 475,
                                                                end: 481,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 475,
                                                        end: 481,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: IntrinsicFunction(
                                                            IntrinsicFunctionExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 482,
                                                                        end: 501,
                                                                        as_str(): "__is_reference_type",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                kind_binding: TypeBinding {
                                                                    inner: IsReferenceType,
                                                                    type_arguments: [
                                                                        TypeArgument {
                                                                            type_id: TypeId(
                                                                                31633,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                31633,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 504,
                                                                                end: 505,
                                                                                as_str(): "E",
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 482,
                                                                        end: 508,
                                                                        as_str(): "__is_reference_type::<E>()",
                                                                    },
                                                                },
                                                                arguments: [],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 482,
                                                            end: 508,
                                                            as_str(): "__is_reference_type::<E>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 475,
                                            end: 509,
                                            as_str(): "assert(__is_reference_type::<E>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 475,
                                    end: 509,
                                    as_str(): "assert(__is_reference_type::<E>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 515,
                                                                end: 521,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 515,
                                                        end: 521,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: IntrinsicFunction(
                                                            IntrinsicFunctionExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 522,
                                                                        end: 541,
                                                                        as_str(): "__is_reference_type",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                kind_binding: TypeBinding {
                                                                    inner: IsReferenceType,
                                                                    type_arguments: [
                                                                        TypeArgument {
                                                                            type_id: TypeId(
                                                                                31634,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                31634,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 544,
                                                                                end: 556,
                                                                                as_str(): "(bool, bool)",
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 522,
                                                                        end: 559,
                                                                        as_str(): "__is_reference_type::<(bool, bool)>()",
                                                                    },
                                                                },
                                                                arguments: [],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 522,
                                                            end: 559,
                                                            as_str(): "__is_reference_type::<(bool, bool)>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 515,
                                            end: 560,
                                            as_str(): "assert(__is_reference_type::<(bool, bool)>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 515,
                                    end: 560,
                                    as_str(): "assert(__is_reference_type::<(bool, bool)>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 566,
                                                                end: 572,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 566,
                                                        end: 572,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: IntrinsicFunction(
                                                            IntrinsicFunctionExpression {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 573,
                                                                        end: 592,
                                                                        as_str(): "__is_reference_type",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                kind_binding: TypeBinding {
                                                                    inner: IsReferenceType,
                                                                    type_arguments: [
                                                                        TypeArgument {
                                                                            type_id: TypeId(
                                                                                31635,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                31635,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 595,
                                                                                end: 603,
                                                                                as_str(): "[u64; 2]",
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 573,
                                                                        end: 606,
                                                                        as_str(): "__is_reference_type::<[u64; 2]>()",
                                                                    },
                                                                },
                                                                arguments: [],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 573,
                                                            end: 606,
                                                            as_str(): "__is_reference_type::<[u64; 2]>()",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 566,
                                            end: 607,
                                            as_str(): "assert(__is_reference_type::<[u64; 2]>())",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 566,
                                    end: 607,
                                    as_str(): "assert(__is_reference_type::<[u64; 2]>())",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 614,
                                                                end: 620,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 614,
                                                        end: 620,
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
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 621,
                                                                                        end: 622,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 621,
                                                                                        end: 622,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "not",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 621,
                                                                                    end: 622,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 621,
                                                                        end: 622,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 622,
                                                                                                end: 638,
                                                                                                as_str(): "arg_is_reference",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 622,
                                                                                        end: 638,
                                                                                        as_str(): "arg_is_reference",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Tuple(
                                                                                            [],
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 639,
                                                                                            end: 641,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 622,
                                                                            end: 642,
                                                                            as_str(): "arg_is_reference(())",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 621,
                                                            end: 642,
                                                            as_str(): "!arg_is_reference(())",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 614,
                                            end: 643,
                                            as_str(): "assert(!arg_is_reference(()))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 614,
                                    end: 643,
                                    as_str(): "assert(!arg_is_reference(()))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 649,
                                                                end: 655,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 649,
                                                        end: 655,
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
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 656,
                                                                                        end: 657,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 656,
                                                                                        end: 657,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "not",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 656,
                                                                                    end: 657,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 656,
                                                                        end: 657,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 657,
                                                                                                end: 673,
                                                                                                as_str(): "arg_is_reference",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 657,
                                                                                        end: 673,
                                                                                        as_str(): "arg_is_reference",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Boolean(
                                                                                                false,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 674,
                                                                                            end: 679,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 657,
                                                                            end: 680,
                                                                            as_str(): "arg_is_reference(false)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 656,
                                                            end: 680,
                                                            as_str(): "!arg_is_reference(false)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 649,
                                            end: 681,
                                            as_str(): "assert(!arg_is_reference(false))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 649,
                                    end: 681,
                                    as_str(): "assert(!arg_is_reference(false))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 687,
                                                                end: 693,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 687,
                                                        end: 693,
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
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 694,
                                                                                        end: 695,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 694,
                                                                                        end: 695,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "not",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 694,
                                                                                    end: 695,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 694,
                                                                        end: 695,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 695,
                                                                                                end: 711,
                                                                                                as_str(): "arg_is_reference",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 695,
                                                                                        end: 711,
                                                                                        as_str(): "arg_is_reference",
                                                                                    },
                                                                                },
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U64(
                                                                                                43,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 712,
                                                                                            end: 716,
                                                                                            as_str(): "0x2b",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 695,
                                                                            end: 717,
                                                                            as_str(): "arg_is_reference(0x2b)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 694,
                                                            end: 717,
                                                            as_str(): "!arg_is_reference(0x2b)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 687,
                                            end: 718,
                                            as_str(): "assert(!arg_is_reference(0x2b))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 687,
                                    end: 718,
                                    as_str(): "assert(!arg_is_reference(0x2b))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 724,
                                                                end: 730,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 724,
                                                        end: 730,
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
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 731,
                                                                                        end: 732,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 731,
                                                                                        end: 732,
                                                                                        as_str(): "!",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "not",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 731,
                                                                                    end: 732,
                                                                                    as_str(): "!",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 731,
                                                                        end: 732,
                                                                        as_str(): "!",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: FunctionApplication(
                                                                            FunctionApplicationExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 732,
                                                                                                end: 748,
                                                                                                as_str(): "arg_is_reference",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 732,
                                                                                        end: 748,
                                                                                        as_str(): "arg_is_reference",
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
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 749,
                                                                                            end: 750,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 732,
                                                                            end: 751,
                                                                            as_str(): "arg_is_reference(0)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 731,
                                                            end: 751,
                                                            as_str(): "!arg_is_reference(0)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 724,
                                            end: 752,
                                            as_str(): "assert(!arg_is_reference(0))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 724,
                                    end: 752,
                                    as_str(): "assert(!arg_is_reference(0))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 759,
                                                                end: 765,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 759,
                                                        end: 765,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: FunctionApplication(
                                                            FunctionApplicationExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 766,
                                                                                end: 782,
                                                                                as_str(): "arg_is_reference",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 766,
                                                                        end: 782,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            String(
                                                                                Span {
                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                    ),
                                                                                    start: 784,
                                                                                    end: 793,
                                                                                    as_str(): "breakfast",
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 783,
                                                                            end: 794,
                                                                            as_str(): "\"breakfast\"",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 766,
                                                            end: 795,
                                                            as_str(): "arg_is_reference(\"breakfast\")",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 759,
                                            end: 796,
                                            as_str(): "assert(arg_is_reference(\"breakfast\"))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 759,
                                    end: 796,
                                    as_str(): "assert(arg_is_reference(\"breakfast\"))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 802,
                                                                end: 808,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 802,
                                                        end: 808,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: FunctionApplication(
                                                            FunctionApplicationExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 809,
                                                                                end: 825,
                                                                                as_str(): "arg_is_reference",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 809,
                                                                        end: 825,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Literal(
                                                                            B256(
                                                                                [
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                    254,
                                                                                ],
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 826,
                                                                            end: 892,
                                                                            as_str(): "0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 809,
                                                            end: 893,
                                                            as_str(): "arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 802,
                                            end: 894,
                                            as_str(): "assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 802,
                                    end: 894,
                                    as_str(): "assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 900,
                                                                end: 906,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 900,
                                                        end: 906,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: FunctionApplication(
                                                            FunctionApplicationExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 907,
                                                                                end: 923,
                                                                                as_str(): "arg_is_reference",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 907,
                                                                        end: 923,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Struct(
                                                                            StructExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 924,
                                                                                                end: 925,
                                                                                                as_str(): "S",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 924,
                                                                                        end: 925,
                                                                                        as_str(): "S",
                                                                                    },
                                                                                },
                                                                                fields: [
                                                                                    StructExpressionField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 928,
                                                                                                end: 929,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value: Expression {
                                                                                            kind: Literal(
                                                                                                Numeric(
                                                                                                    42,
                                                                                                ),
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 931,
                                                                                                end: 933,
                                                                                                as_str(): "42",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 928,
                                                                                            end: 933,
                                                                                            as_str(): "a: 42",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 924,
                                                                            end: 935,
                                                                            as_str(): "S { a: 42 }",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 907,
                                                            end: 936,
                                                            as_str(): "arg_is_reference(S { a: 42 })",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 900,
                                            end: 937,
                                            as_str(): "assert(arg_is_reference(S { a: 42 }))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 900,
                                    end: 937,
                                    as_str(): "assert(arg_is_reference(S { a: 42 }))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 943,
                                                                end: 949,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 943,
                                                        end: 949,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: FunctionApplication(
                                                            FunctionApplicationExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 950,
                                                                                end: 966,
                                                                                as_str(): "arg_is_reference",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 950,
                                                                        end: 966,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                                arguments: [
                                                                    Expression {
                                                                        kind: DelineatedPath(
                                                                            DelineatedPathExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c08fd1d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                    ),
                                                                                                    start: 967,
                                                                                                    end: 968,
                                                                                                    as_str(): "E",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                                ),
                                                                                                start: 970,
                                                                                                end: 977,
                                                                                                as_str(): "Variant",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 970,
                                                                                        end: 977,
                                                                                        as_str(): "Variant",
                                                                                    },
                                                                                },
                                                                                args: None,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 967,
                                                                            end: 977,
                                                                            as_str(): "E::Variant",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 950,
                                                            end: 978,
                                                            as_str(): "arg_is_reference(E::Variant)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 943,
                                            end: 979,
                                            as_str(): "assert(arg_is_reference(E::Variant))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 943,
                                    end: 979,
                                    as_str(): "assert(arg_is_reference(E::Variant))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 985,
                                                                end: 991,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 985,
                                                        end: 991,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: FunctionApplication(
                                                            FunctionApplicationExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 992,
                                                                                end: 1008,
                                                                                as_str(): "arg_is_reference",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 992,
                                                                        end: 1008,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Tuple(
                                                                            [
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            true,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 1010,
                                                                                        end: 1014,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Boolean(
                                                                                            true,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                        ),
                                                                                        start: 1016,
                                                                                        end: 1020,
                                                                                        as_str(): "true",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 1009,
                                                                            end: 1021,
                                                                            as_str(): "(true, true)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 992,
                                                            end: 1022,
                                                            as_str(): "arg_is_reference((true, true))",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 985,
                                            end: 1023,
                                            as_str(): "assert(arg_is_reference((true, true)))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 985,
                                    end: 1023,
                                    as_str(): "assert(arg_is_reference((true, true)))",
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
                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                ),
                                                                start: 1029,
                                                                end: 1035,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                        ),
                                                        start: 1029,
                                                        end: 1035,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: FunctionApplication(
                                                            FunctionApplicationExpression {
                                                                call_path_binding: TypeBinding {
                                                                    inner: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c08fd1d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                ),
                                                                                start: 1036,
                                                                                end: 1052,
                                                                                as_str(): "arg_is_reference",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0c08fd1d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                        ),
                                                                        start: 1036,
                                                                        end: 1052,
                                                                        as_str(): "arg_is_reference",
                                                                    },
                                                                },
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Array(
                                                                            ArrayExpression {
                                                                                contents: [
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                5,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1054,
                                                                                            end: 1055,
                                                                                            as_str(): "5",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                4,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1057,
                                                                                            end: 1058,
                                                                                            as_str(): "4",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                3,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1060,
                                                                                            end: 1061,
                                                                                            as_str(): "3",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1063,
                                                                                            end: 1064,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            Numeric(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                                            ),
                                                                                            start: 1066,
                                                                                            end: 1067,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                length_span: None,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                                            ),
                                                                            start: 1053,
                                                                            end: 1068,
                                                                            as_str(): "[5, 4, 3, 2, 1]",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c08fd1d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                                            ),
                                                            start: 1036,
                                                            end: 1069,
                                                            as_str(): "arg_is_reference([5, 4, 3, 2, 1])",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 1029,
                                            end: 1070,
                                            as_str(): "assert(arg_is_reference([5, 4, 3, 2, 1]))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 1029,
                                    end: 1070,
                                    as_str(): "assert(arg_is_reference([5, 4, 3, 2, 1]))",
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
                                            src (ptr): 0x00007fe0c08fd1d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                            ),
                                            start: 1077,
                                            end: 1081,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c08fd1d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                                    ),
                                    start: 1077,
                                    end: 1081,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0c08fd1d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                            ),
                            start: 181,
                            end: 1083,
                            as_str(): "{\n    assert(!__is_reference_type::<()>());        // Is Unit ref or not?\n    assert(!__is_reference_type::<bool>());\n    assert(!__is_reference_type::<u64>());\n\n    assert(__is_reference_type::<str[1]>());\n    assert(__is_reference_type::<b256>());\n    assert(__is_reference_type::<S>());\n    assert(__is_reference_type::<E>());\n    assert(__is_reference_type::<(bool, bool)>());\n    assert(__is_reference_type::<[u64; 2]>());\n\n    assert(!arg_is_reference(()));\n    assert(!arg_is_reference(false));\n    assert(!arg_is_reference(0x2b));\n    assert(!arg_is_reference(0));\n\n    assert(arg_is_reference(\"breakfast\"));\n    assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe));\n    assert(arg_is_reference(S { a: 42 }));\n    assert(arg_is_reference(E::Variant));\n    assert(arg_is_reference((true, true)));\n    assert(arg_is_reference([5, 4, 3, 2, 1]));\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0c08fd1d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                        ),
                        start: 163,
                        end: 1083,
                        as_str(): "fn main() -> bool {\n    assert(!__is_reference_type::<()>());        // Is Unit ref or not?\n    assert(!__is_reference_type::<bool>());\n    assert(!__is_reference_type::<u64>());\n\n    assert(__is_reference_type::<str[1]>());\n    assert(__is_reference_type::<b256>());\n    assert(__is_reference_type::<S>());\n    assert(__is_reference_type::<E>());\n    assert(__is_reference_type::<(bool, bool)>());\n    assert(__is_reference_type::<[u64; 2]>());\n\n    assert(!arg_is_reference(()));\n    assert(!arg_is_reference(false));\n    assert(!arg_is_reference(0x2b));\n    assert(!arg_is_reference(0));\n\n    assert(arg_is_reference(\"breakfast\"));\n    assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe));\n    assert(arg_is_reference(S { a: 42 }));\n    assert(arg_is_reference(E::Variant));\n    assert(arg_is_reference((true, true)));\n    assert(arg_is_reference([5, 4, 3, 2, 1]));\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0c08fd1d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
                        ),
                        start: 176,
                        end: 180,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0c08fd1d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRH8jSt4/is_reference_type/src/main.sw",
            ),
            start: 163,
            end: 1083,
            as_str(): "fn main() -> bool {\n    assert(!__is_reference_type::<()>());        // Is Unit ref or not?\n    assert(!__is_reference_type::<bool>());\n    assert(!__is_reference_type::<u64>());\n\n    assert(__is_reference_type::<str[1]>());\n    assert(__is_reference_type::<b256>());\n    assert(__is_reference_type::<S>());\n    assert(__is_reference_type::<E>());\n    assert(__is_reference_type::<(bool, bool)>());\n    assert(__is_reference_type::<[u64; 2]>());\n\n    assert(!arg_is_reference(()));\n    assert(!arg_is_reference(false));\n    assert(!arg_is_reference(0x2b));\n    assert(!arg_is_reference(0));\n\n    assert(arg_is_reference(\"breakfast\"));\n    assert(arg_is_reference(0xfefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe));\n    assert(arg_is_reference(S { a: 42 }));\n    assert(arg_is_reference(E::Variant));\n    assert(arg_is_reference((true, true)));\n    assert(arg_is_reference([5, 4, 3, 2, 1]));\n\n    true\n}",
        },
    },
]
