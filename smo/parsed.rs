[
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe056083bb0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                            ),
                            start: 12,
                            end: 15,
                            as_str(): "smo",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 83,
                                                        end: 88,
                                                        as_str(): "__smo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Smo,
                                                    type_arguments: [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                0,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                0,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 91,
                                                                end: 92,
                                                                as_str(): "T",
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 83,
                                                        end: 132,
                                                        as_str(): "__smo::<T>(recipient, value, output_index, coins)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 94,
                                                                    end: 103,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 94,
                                                            end: 103,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 105,
                                                                    end: 110,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 105,
                                                            end: 110,
                                                            as_str(): "value",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 112,
                                                                    end: 124,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 112,
                                                            end: 124,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 131,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 126,
                                                            end: 131,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 83,
                                            end: 132,
                                            as_str(): "__smo::<T>(recipient, value, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 83,
                                    end: 132,
                                    as_str(): "__smo::<T>(recipient, value, output_index, coins)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe056083bb0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                            ),
                            start: 77,
                            end: 135,
                            as_str(): "{\n    __smo::<T>(recipient, value, output_index, coins);\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 19,
                                    end: 28,
                                    as_str(): "recipient",
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
                            type_info: B256,
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 30,
                                end: 34,
                                as_str(): "b256",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 36,
                                    end: 41,
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
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 44,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 43,
                                end: 44,
                                as_str(): "T",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 46,
                                    end: 58,
                                    as_str(): "output_index",
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
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 60,
                                end: 63,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 70,
                                    as_str(): "coins",
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
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 72,
                                end: 75,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe056083bb0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                        ),
                        start: 9,
                        end: 135,
                        as_str(): "fn smo<T>(recipient: b256, value: T, output_index: u64, coins: u64) {\n    __smo::<T>(recipient, value, output_index, coins);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [
                        T: TypeId(1),
                    ],
                    return_type_span: Span {
                        src (ptr): 0x00007fe056083bb0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                        ),
                        start: 9,
                        end: 76,
                        as_str(): "fn smo<T>(recipient: b256, value: T, output_index: u64, coins: u64)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe056083bb0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
            ),
            start: 9,
            end: 135,
            as_str(): "fn smo<T>(recipient: b256, value: T, output_index: u64, coins: u64) {\n    __smo::<T>(recipient, value, output_index, coins);\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe056083bb0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                            ),
                            start: 144,
                            end: 154,
                            as_str(): "TestStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 164,
                                    end: 171,
                                    as_str(): "field_1",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Boolean,
                            span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 164,
                                end: 177,
                                as_str(): "field_1: bool",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 173,
                                end: 177,
                                as_str(): "bool",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 183,
                                    end: 190,
                                    as_str(): "field_2",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 192,
                                        end: 193,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 183,
                                end: 193,
                                as_str(): "field_2: T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 192,
                                end: 193,
                                as_str(): "T",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 199,
                                    end: 206,
                                    as_str(): "field_3",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 199,
                                end: 211,
                                as_str(): "field_3: u64",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 208,
                                end: 211,
                                as_str(): "u64",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(2),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe056083bb0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                        ),
                        start: 137,
                        end: 214,
                        as_str(): "struct TestStruct<T> {\n    field_1: bool,\n    field_2: T,\n    field_3: u64,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe056083bb0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
            ),
            start: 137,
            end: 214,
            as_str(): "struct TestStruct<T> {\n    field_1: bool,\n    field_2: T,\n    field_3: u64,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe056083bb0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                            ),
                            start: 221,
                            end: 229,
                            as_str(): "TestEnum",
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
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 236,
                                    end: 246,
                                    as_str(): "VariantOne",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 248,
                                end: 250,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 236,
                                end: 250,
                                as_str(): "VariantOne: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 256,
                                    end: 266,
                                    as_str(): "VariantTwo",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 268,
                                end: 270,
                                as_str(): "()",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 256,
                                end: 270,
                                as_str(): "VariantTwo: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe056083bb0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                        ),
                        start: 216,
                        end: 273,
                        as_str(): "enum TestEnum {\n    VariantOne: (),\n    VariantTwo: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe056083bb0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
            ),
            start: 216,
            end: 273,
            as_str(): "enum TestEnum {\n    VariantOne: (),\n    VariantTwo: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe056083bb0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                            ),
                            start: 284,
                            end: 290,
                            as_str(): "Option",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [
                        T: TypeId(3),
                    ],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 300,
                                    end: 304,
                                    as_str(): "None",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 306,
                                end: 308,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 300,
                                end: 308,
                                as_str(): "None: ()",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 314,
                                    end: 318,
                                    as_str(): "Some",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe056083bb0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                        ),
                                        start: 320,
                                        end: 321,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 320,
                                end: 321,
                                as_str(): "T",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fe056083bb0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                ),
                                start: 314,
                                end: 321,
                                as_str(): "Some: T",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe056083bb0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                        ),
                        start: 275,
                        end: 324,
                        as_str(): "pub enum Option<T> {\n    None: (),\n    Some: T,\n}",
                    },
                    visibility: Public,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe056083bb0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
            ),
            start: 275,
            end: 324,
            as_str(): "pub enum Option<T> {\n    None: (),\n    Some: T,\n}",
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
                            src (ptr): 0x00007fe056083bb0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                            ),
                            start: 329,
                            end: 333,
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
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 354,
                                                    end: 363,
                                                    as_str(): "recipient",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    B256(
                                                        [
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                            1,
                                                        ],
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 366,
                                                    end: 432,
                                                    as_str(): "0x0101010101010101010101010101010101010101010101010101010101010101",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 350,
                                    end: 433,
                                    as_str(): "let recipient = 0x0101010101010101010101010101010101010101010101010101010101010101;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 442,
                                                    end: 454,
                                                    as_str(): "output_index",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        3,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 457,
                                                    end: 458,
                                                    as_str(): "3",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 438,
                                    end: 459,
                                    as_str(): "let output_index = 3;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 468,
                                                    end: 473,
                                                    as_str(): "coins",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        24,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 476,
                                                    end: 478,
                                                    as_str(): "24",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 464,
                                    end: 479,
                                    as_str(): "let coins = 24;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 548,
                                                    end: 549,
                                                    as_str(): "k",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: B256,
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 551,
                                                    end: 555,
                                                    as_str(): "b256",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    B256(
                                                        [
                                                            239,
                                                            134,
                                                            175,
                                                            169,
                                                            105,
                                                            108,
                                                            240,
                                                            220,
                                                            99,
                                                            133,
                                                            226,
                                                            196,
                                                            7,
                                                            166,
                                                            225,
                                                            89,
                                                            161,
                                                            16,
                                                            60,
                                                            239,
                                                            183,
                                                            226,
                                                            174,
                                                            6,
                                                            54,
                                                            251,
                                                            51,
                                                            211,
                                                            203,
                                                            42,
                                                            158,
                                                            74,
                                                        ],
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 558,
                                                    end: 624,
                                                    as_str(): "0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 544,
                                    end: 625,
                                    as_str(): "let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 634,
                                                    end: 635,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Str(
                                                Length {
                                                    val: 4,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 641,
                                                        end: 642,
                                                        as_str(): "4",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 637,
                                                    end: 643,
                                                    as_str(): "str[4]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    String(
                                                        Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 647,
                                                            end: 651,
                                                            as_str(): "Fuel",
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 646,
                                                    end: 652,
                                                    as_str(): "\"Fuel\"",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 630,
                                    end: 653,
                                    as_str(): "let a: str[4] = \"Fuel\";",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 662,
                                                    end: 663,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Array(
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        4,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        4,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 666,
                                                        end: 668,
                                                        as_str(): "u8",
                                                    },
                                                },
                                                Length {
                                                    val: 3,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 670,
                                                        end: 671,
                                                        as_str(): "3",
                                                    },
                                                },
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 665,
                                                    end: 672,
                                                    as_str(): "[u8; 3]",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 676,
                                                                    end: 679,
                                                                    as_str(): "1u8",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        2,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 681,
                                                                    end: 684,
                                                                    as_str(): "2u8",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    U8(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 686,
                                                                    end: 689,
                                                                    as_str(): "3u8",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 675,
                                                    end: 690,
                                                    as_str(): "[1u8, 2u8, 3u8]",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 658,
                                    end: 691,
                                    as_str(): "let b: [u8; 3] = [1u8, 2u8, 3u8];",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 700,
                                                    end: 711,
                                                    as_str(): "test_struct",
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 714,
                                                                        end: 724,
                                                                        as_str(): "TestStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 714,
                                                                end: 724,
                                                                as_str(): "TestStruct",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 735,
                                                                        end: 742,
                                                                        as_str(): "field_1",
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
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 744,
                                                                        end: 748,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 735,
                                                                    end: 748,
                                                                    as_str(): "field_1: true",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 765,
                                                                        as_str(): "field_2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                ),
                                                                                start: 767,
                                                                                end: 768,
                                                                                as_str(): "k",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 767,
                                                                        end: 768,
                                                                        as_str(): "k",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 758,
                                                                    end: 768,
                                                                    as_str(): "field_2: k",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 778,
                                                                        end: 785,
                                                                        as_str(): "field_3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Numeric(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 787,
                                                                        end: 789,
                                                                        as_str(): "11",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 778,
                                                                    end: 789,
                                                                    as_str(): "field_3: 11",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 714,
                                                    end: 796,
                                                    as_str(): "TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 696,
                                    end: 797,
                                    as_str(): "let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 807,
                                                    end: 816,
                                                    as_str(): "test_enum",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: DelineatedPath(
                                                    DelineatedPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 819,
                                                                            end: 827,
                                                                            as_str(): "TestEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 829,
                                                                        end: 839,
                                                                        as_str(): "VariantTwo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 829,
                                                                end: 839,
                                                                as_str(): "VariantTwo",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe056083bb0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                    ),
                                                    start: 819,
                                                    end: 839,
                                                    as_str(): "TestEnum::VariantTwo",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 803,
                                    end: 840,
                                    as_str(): "let test_enum = TestEnum::VariantTwo;",
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 845,
                                                                end: 848,
                                                                as_str(): "smo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 845,
                                                        end: 848,
                                                        as_str(): "smo",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 849,
                                                                    end: 858,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 849,
                                                            end: 858,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 860,
                                                                    end: 861,
                                                                    as_str(): "k",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 860,
                                                            end: 861,
                                                            as_str(): "k",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 863,
                                                                    end: 875,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 863,
                                                            end: 875,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 877,
                                                                    end: 882,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 877,
                                                            end: 882,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 845,
                                            end: 883,
                                            as_str(): "smo(recipient, k, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 845,
                                    end: 883,
                                    as_str(): "smo(recipient, k, output_index, coins)",
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 889,
                                                                end: 892,
                                                                as_str(): "smo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 889,
                                                        end: 892,
                                                        as_str(): "smo",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 893,
                                                                    end: 902,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 893,
                                                            end: 902,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            Numeric(
                                                                42,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 904,
                                                            end: 906,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 908,
                                                                    end: 920,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 908,
                                                            end: 920,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 922,
                                                                    end: 927,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 922,
                                                            end: 927,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 889,
                                            end: 928,
                                            as_str(): "smo(recipient, 42, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 889,
                                    end: 928,
                                    as_str(): "smo(recipient, 42, output_index, coins)",
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 934,
                                                                end: 937,
                                                                as_str(): "smo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 934,
                                                        end: 937,
                                                        as_str(): "smo",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 938,
                                                                    end: 947,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 938,
                                                            end: 947,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            U32(
                                                                42,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 949,
                                                            end: 954,
                                                            as_str(): "42u32",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 956,
                                                                    end: 968,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 956,
                                                            end: 968,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 970,
                                                                    end: 975,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 970,
                                                            end: 975,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 934,
                                            end: 976,
                                            as_str(): "smo(recipient, 42u32, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 934,
                                    end: 976,
                                    as_str(): "smo(recipient, 42u32, output_index, coins)",
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 982,
                                                                end: 985,
                                                                as_str(): "smo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 982,
                                                        end: 985,
                                                        as_str(): "smo",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 986,
                                                                    end: 995,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 986,
                                                            end: 995,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            U16(
                                                                42,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 997,
                                                            end: 1002,
                                                            as_str(): "42u16",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1004,
                                                                    end: 1016,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1004,
                                                            end: 1016,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1018,
                                                                    end: 1023,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1018,
                                                            end: 1023,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 982,
                                            end: 1024,
                                            as_str(): "smo(recipient, 42u16, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 982,
                                    end: 1024,
                                    as_str(): "smo(recipient, 42u16, output_index, coins)",
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
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 1030,
                                                                end: 1033,
                                                                as_str(): "smo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1030,
                                                        end: 1033,
                                                        as_str(): "smo",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1034,
                                                                    end: 1043,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1034,
                                                            end: 1043,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Literal(
                                                            U8(
                                                                42,
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1045,
                                                            end: 1049,
                                                            as_str(): "42u8",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1051,
                                                                    end: 1063,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1051,
                                                            end: 1063,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1065,
                                                                    end: 1070,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1065,
                                                            end: 1070,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1030,
                                            end: 1071,
                                            as_str(): "smo(recipient, 42u8, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1030,
                                    end: 1071,
                                    as_str(): "smo(recipient, 42u8, output_index, coins)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1077,
                                                        end: 1082,
                                                        as_str(): "__smo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Smo,
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1077,
                                                        end: 1117,
                                                        as_str(): "__smo(recipient, a, output_index, coins)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1083,
                                                                    end: 1092,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1083,
                                                            end: 1092,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1094,
                                                                    end: 1095,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1094,
                                                            end: 1095,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1097,
                                                                    end: 1109,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1097,
                                                            end: 1109,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1111,
                                                                    end: 1116,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1111,
                                                            end: 1116,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1077,
                                            end: 1117,
                                            as_str(): "__smo(recipient, a, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1077,
                                    end: 1117,
                                    as_str(): "__smo(recipient, a, output_index, coins)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1123,
                                                        end: 1128,
                                                        as_str(): "__smo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Smo,
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1123,
                                                        end: 1163,
                                                        as_str(): "__smo(recipient, b, output_index, coins)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1129,
                                                                    end: 1138,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1129,
                                                            end: 1138,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1140,
                                                                    end: 1141,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1140,
                                                            end: 1141,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1143,
                                                                    end: 1155,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1143,
                                                            end: 1155,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1157,
                                                                    end: 1162,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1157,
                                                            end: 1162,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1123,
                                            end: 1163,
                                            as_str(): "__smo(recipient, b, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1123,
                                    end: 1163,
                                    as_str(): "__smo(recipient, b, output_index, coins)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1169,
                                                        end: 1174,
                                                        as_str(): "__smo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Smo,
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1169,
                                                        end: 1219,
                                                        as_str(): "__smo(recipient, test_struct, output_index, coins)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1175,
                                                                    end: 1184,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1175,
                                                            end: 1184,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1186,
                                                                    end: 1197,
                                                                    as_str(): "test_struct",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1186,
                                                            end: 1197,
                                                            as_str(): "test_struct",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1199,
                                                                    end: 1211,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1199,
                                                            end: 1211,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1213,
                                                                    end: 1218,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1213,
                                                            end: 1218,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1169,
                                            end: 1219,
                                            as_str(): "__smo(recipient, test_struct, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1169,
                                    end: 1219,
                                    as_str(): "__smo(recipient, test_struct, output_index, coins)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1225,
                                                        end: 1230,
                                                        as_str(): "__smo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Smo,
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1225,
                                                        end: 1273,
                                                        as_str(): "__smo(recipient, test_enum, output_index, coins)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1231,
                                                                    end: 1240,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1231,
                                                            end: 1240,
                                                            as_str(): "recipient",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1242,
                                                                    end: 1251,
                                                                    as_str(): "test_enum",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1242,
                                                            end: 1251,
                                                            as_str(): "test_enum",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1253,
                                                                    end: 1265,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1253,
                                                            end: 1265,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1267,
                                                                    end: 1272,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1267,
                                                            end: 1272,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1225,
                                            end: 1273,
                                            as_str(): "__smo(recipient, test_enum, output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1225,
                                    end: 1273,
                                    as_str(): "__smo(recipient, test_enum, output_index, coins)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1279,
                                                        end: 1284,
                                                        as_str(): "__smo",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Smo,
                                                    type_arguments: [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                7,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                7,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe056083bb0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                ),
                                                                start: 1287,
                                                                end: 1312,
                                                                as_str(): "Option::<TestStruct<u64>>",
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1279,
                                                        end: 1444,
                                                        as_str(): "__smo::<Option::<TestStruct<u64>>>(recipient, Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }), output_index, coins)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1314,
                                                                    end: 1323,
                                                                    as_str(): "recipient",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1314,
                                                            end: 1323,
                                                            as_str(): "recipient",
                                                        },
                                                    },
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
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1325,
                                                                                        end: 1331,
                                                                                        as_str(): "Option",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1325,
                                                                                    end: 1331,
                                                                                    as_str(): "Option",
                                                                                },
                                                                            },
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe056083bb0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                    ),
                                                                                    start: 1333,
                                                                                    end: 1337,
                                                                                    as_str(): "Some",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        is_absolute: false,
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe056083bb0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                        ),
                                                                        start: 1325,
                                                                        end: 1337,
                                                                        as_str(): "Option::Some",
                                                                    },
                                                                },
                                                                args: [
                                                                    Expression {
                                                                        kind: Struct(
                                                                            StructExpression {
                                                                                call_path_binding: TypeBinding {
                                                                                    inner: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1338,
                                                                                                end: 1348,
                                                                                                as_str(): "TestStruct",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe056083bb0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                        ),
                                                                                        start: 1338,
                                                                                        end: 1348,
                                                                                        as_str(): "TestStruct",
                                                                                    },
                                                                                },
                                                                                fields: [
                                                                                    StructExpressionField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1359,
                                                                                                end: 1366,
                                                                                                as_str(): "field_1",
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
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1368,
                                                                                                end: 1372,
                                                                                                as_str(): "true",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 1359,
                                                                                            end: 1372,
                                                                                            as_str(): "field_1: true",
                                                                                        },
                                                                                    },
                                                                                    StructExpressionField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1382,
                                                                                                end: 1389,
                                                                                                as_str(): "field_2",
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
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1391,
                                                                                                end: 1393,
                                                                                                as_str(): "42",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 1382,
                                                                                            end: 1393,
                                                                                            as_str(): "field_2: 42",
                                                                                        },
                                                                                    },
                                                                                    StructExpressionField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1403,
                                                                                                end: 1410,
                                                                                                as_str(): "field_3",
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
                                                                                                src (ptr): 0x00007fe056083bb0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                                ),
                                                                                                start: 1412,
                                                                                                end: 1414,
                                                                                                as_str(): "42",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe056083bb0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                                            ),
                                                                                            start: 1403,
                                                                                            end: 1414,
                                                                                            as_str(): "field_3: 42",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe056083bb0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                            ),
                                                                            start: 1338,
                                                                            end: 1421,
                                                                            as_str(): "TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1325,
                                                            end: 1422,
                                                            as_str(): "Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    })",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1424,
                                                                    end: 1436,
                                                                    as_str(): "output_index",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1424,
                                                            end: 1436,
                                                            as_str(): "output_index",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1438,
                                                                    end: 1443,
                                                                    as_str(): "coins",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1438,
                                                            end: 1443,
                                                            as_str(): "coins",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1279,
                                            end: 1444,
                                            as_str(): "__smo::<Option::<TestStruct<u64>>>(recipient, Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }), output_index, coins)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1279,
                                    end: 1444,
                                    as_str(): "__smo::<Option::<TestStruct<u64>>>(recipient, Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }), output_index, coins)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1517,
                                                        end: 1522,
                                                        as_str(): "__log",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Log,
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1517,
                                                        end: 1525,
                                                        as_str(): "__log(a)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1523,
                                                                    end: 1524,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1523,
                                                            end: 1524,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1517,
                                            end: 1525,
                                            as_str(): "__log(a)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1517,
                                    end: 1525,
                                    as_str(): "__log(a)",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: IntrinsicFunction(
                                            IntrinsicFunctionExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1531,
                                                        end: 1536,
                                                        as_str(): "__log",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                kind_binding: TypeBinding {
                                                    inner: Log,
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe056083bb0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                        ),
                                                        start: 1531,
                                                        end: 1539,
                                                        as_str(): "__log(b)",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe056083bb0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                                    ),
                                                                    start: 1537,
                                                                    end: 1538,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe056083bb0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                                            ),
                                                            start: 1537,
                                                            end: 1538,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1531,
                                            end: 1539,
                                            as_str(): "__log(b)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1531,
                                    end: 1539,
                                    as_str(): "__log(b)",
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
                                            src (ptr): 0x00007fe056083bb0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                            ),
                                            start: 1546,
                                            end: 1550,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe056083bb0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                                    ),
                                    start: 1546,
                                    end: 1550,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe056083bb0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                            ),
                            start: 344,
                            end: 1552,
                            as_str(): "{\n    let recipient = 0x0101010101010101010101010101010101010101010101010101010101010101;\n    let output_index = 3;\n    let coins = 24;\n\n    // Check various data types as message data in `__smo`\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    smo(recipient, k, output_index, coins);\n    smo(recipient, 42, output_index, coins);\n    smo(recipient, 42u32, output_index, coins);\n    smo(recipient, 42u16, output_index, coins);\n    smo(recipient, 42u8, output_index, coins);\n    __smo(recipient, a, output_index, coins);\n    __smo(recipient, b, output_index, coins);\n    __smo(recipient, test_struct, output_index, coins);\n    __smo(recipient, test_enum, output_index, coins);\n    __smo::<Option::<TestStruct<u64>>>(recipient, Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }), output_index, coins);\n\n    // Make sure that logs don't clobber messages in the JSON ABI\n    __log(a);\n    __log(b);\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe056083bb0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                        ),
                        start: 326,
                        end: 1552,
                        as_str(): "fn main() -> bool {\n    let recipient = 0x0101010101010101010101010101010101010101010101010101010101010101;\n    let output_index = 3;\n    let coins = 24;\n\n    // Check various data types as message data in `__smo`\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    smo(recipient, k, output_index, coins);\n    smo(recipient, 42, output_index, coins);\n    smo(recipient, 42u32, output_index, coins);\n    smo(recipient, 42u16, output_index, coins);\n    smo(recipient, 42u8, output_index, coins);\n    __smo(recipient, a, output_index, coins);\n    __smo(recipient, b, output_index, coins);\n    __smo(recipient, test_struct, output_index, coins);\n    __smo(recipient, test_enum, output_index, coins);\n    __smo::<Option::<TestStruct<u64>>>(recipient, Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }), output_index, coins);\n\n    // Make sure that logs don't clobber messages in the JSON ABI\n    __log(a);\n    __log(b);\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe056083bb0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
                        ),
                        start: 339,
                        end: 343,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe056083bb0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXynQs3/smo/src/main.sw",
            ),
            start: 326,
            end: 1552,
            as_str(): "fn main() -> bool {\n    let recipient = 0x0101010101010101010101010101010101010101010101010101010101010101;\n    let output_index = 3;\n    let coins = 24;\n\n    // Check various data types as message data in `__smo`\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    smo(recipient, k, output_index, coins);\n    smo(recipient, 42, output_index, coins);\n    smo(recipient, 42u32, output_index, coins);\n    smo(recipient, 42u16, output_index, coins);\n    smo(recipient, 42u8, output_index, coins);\n    __smo(recipient, a, output_index, coins);\n    __smo(recipient, b, output_index, coins);\n    __smo(recipient, test_struct, output_index, coins);\n    __smo(recipient, test_enum, output_index, coins);\n    __smo::<Option::<TestStruct<u64>>>(recipient, Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }), output_index, coins);\n\n    // Make sure that logs don't clobber messages in the JSON ABI\n    __log(a);\n    __log(b);\n\n    true\n}",
        },
    },
]
