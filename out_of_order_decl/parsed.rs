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
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 93,
                            end: 97,
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
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 118,
                                                    end: 119,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        42,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 124,
                                                    as_str(): "42",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 125,
                                    as_str(): "let a = 42;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 157,
                                                    end: 158,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: FunctionApplication(
                                                    FunctionApplicationExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 161,
                                                                        end: 176,
                                                                        as_str(): "the_number_five",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 161,
                                                                end: 176,
                                                                as_str(): "the_number_five",
                                                            },
                                                        },
                                                        arguments: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 161,
                                                    end: 178,
                                                    as_str(): "the_number_five()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 153,
                                    end: 179,
                                    as_str(): "let x = the_number_five();",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 213,
                                                    end: 214,
                                                    as_str(): "z",
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 217,
                                                                            end: 223,
                                                                            as_str(): "AnEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 225,
                                                                        end: 232,
                                                                        as_str(): "Variant",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 225,
                                                                end: 232,
                                                                as_str(): "Variant",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 217,
                                                    end: 232,
                                                    as_str(): "AnEnum::Variant",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 209,
                                    end: 233,
                                    as_str(): "let z = AnEnum::Variant;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 269,
                                                    end: 270,
                                                    as_str(): "y",
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
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 273,
                                                                        end: 283,
                                                                        as_str(): "FuelStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 273,
                                                                end: 283,
                                                                as_str(): "FuelStruct",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 294,
                                                                        end: 295,
                                                                        as_str(): "a",
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
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 297,
                                                                        end: 301,
                                                                        as_str(): "true",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 294,
                                                                    end: 301,
                                                                    as_str(): "a: true",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 311,
                                                                        end: 312,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        Boolean(
                                                                            false,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 314,
                                                                        end: 319,
                                                                        as_str(): "false",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 311,
                                                                    end: 319,
                                                                    as_str(): "b: false",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 273,
                                                    end: 326,
                                                    as_str(): "FuelStruct {\n        a: true,\n        b: false,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 265,
                                    end: 327,
                                    as_str(): "let y = FuelStruct {\n        a: true,\n        b: false,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 395,
                                                    end: 396,
                                                    as_str(): "u",
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
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 399,
                                                                        end: 410,
                                                                        as_str(): "FuelWrapper",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 399,
                                                                end: 410,
                                                                as_str(): "FuelWrapper",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 421,
                                                                        end: 422,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 424,
                                                                                end: 425,
                                                                                as_str(): "y",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 424,
                                                                        end: 425,
                                                                        as_str(): "y",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 421,
                                                                    end: 425,
                                                                    as_str(): "a: y",
                                                                },
                                                            },
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 435,
                                                                        end: 436,
                                                                        as_str(): "b",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 438,
                                                                                end: 439,
                                                                                as_str(): "z",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 438,
                                                                        end: 439,
                                                                        as_str(): "z",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 435,
                                                                    end: 439,
                                                                    as_str(): "b: z",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 399,
                                                    end: 446,
                                                    as_str(): "FuelWrapper {\n        a: y,\n        b: z,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 391,
                                    end: 447,
                                    as_str(): "let u = FuelWrapper {\n        a: y,\n        b: z,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 457,
                                                    end: 458,
                                                    as_str(): "v",
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
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 461,
                                                                                end: 472,
                                                                                as_str(): "WrapperEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 461,
                                                                            end: 472,
                                                                            as_str(): "WrapperEnum",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 474,
                                                                            end: 481,
                                                                            as_str(): "Variant",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 461,
                                                                end: 481,
                                                                as_str(): "WrapperEnum::Variant",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 482,
                                                                            end: 483,
                                                                            as_str(): "u",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 482,
                                                                    end: 483,
                                                                    as_str(): "u",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 461,
                                                    end: 484,
                                                    as_str(): "WrapperEnum::Variant(u)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 453,
                                    end: 485,
                                    as_str(): "let v = WrapperEnum::Variant(u);",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Match(
                                            MatchExpression {
                                                value: Expression {
                                                    kind: DelineatedPath(
                                                        DelineatedPathExpression {
                                                            call_path_binding: TypeBinding {
                                                                inner: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 516,
                                                                                end: 522,
                                                                                as_str(): "AnEnum",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 524,
                                                                            end: 531,
                                                                            as_str(): "Variant",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 524,
                                                                    end: 531,
                                                                    as_str(): "Variant",
                                                                },
                                                            },
                                                            args: None,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 516,
                                                        end: 531,
                                                        as_str(): "AnEnum::Variant",
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 498,
                                                                            end: 504,
                                                                            as_str(): "AnEnum",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 506,
                                                                        end: 513,
                                                                        as_str(): "Variant",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            value: CatchAll {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 498,
                                                                    end: 513,
                                                                    as_str(): "AnEnum::Variant",
                                                                },
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 498,
                                                                end: 513,
                                                                as_str(): "AnEnum::Variant",
                                                            },
                                                        },
                                                        result: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
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
                                                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 542,
                                                                                                            end: 546,
                                                                                                            as_str(): "void",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                                    ),
                                                                                                    start: 542,
                                                                                                    end: 546,
                                                                                                    as_str(): "void",
                                                                                                },
                                                                                            },
                                                                                            arguments: [],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 542,
                                                                                        end: 548,
                                                                                        as_str(): "void()",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 542,
                                                                                end: 548,
                                                                                as_str(): "void()",
                                                                            },
                                                                        },
                                                                    ],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 532,
                                                                        end: 555,
                                                                        as_str(): "{\n        void();\n    }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 532,
                                                                end: 555,
                                                                as_str(): "{\n        void();\n    }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 498,
                                                            end: 555,
                                                            as_str(): "AnEnum::Variant = AnEnum::Variant {\n        void();\n    }",
                                                        },
                                                    },
                                                    MatchBranch {
                                                        scrutinee: CatchAll {
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 532,
                                                                end: 555,
                                                                as_str(): "{\n        void();\n    }",
                                                            },
                                                        },
                                                        result: Expression {
                                                            kind: CodeBlock(
                                                                CodeBlock {
                                                                    contents: [],
                                                                    whole_block_span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 532,
                                                                        end: 555,
                                                                        as_str(): "{\n        void();\n    }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 532,
                                                                end: 555,
                                                                as_str(): "{\n        void();\n    }",
                                                            },
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 532,
                                                            end: 555,
                                                            as_str(): "{\n        void();\n    }",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 491,
                                            end: 555,
                                            as_str(): "if let AnEnum::Variant = AnEnum::Variant {\n        void();\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 491,
                                    end: 555,
                                    as_str(): "if let AnEnum::Variant = AnEnum::Variant {\n        void();\n    }",
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
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 561,
                                            end: 565,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 561,
                                    end: 565,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 108,
                            end: 567,
                            as_str(): "{\n    let a = 42;\n\n    // fn before decl\n    let x = the_number_five();\n\n    // enum before decl\n    let z = AnEnum::Variant;\n\n    // struct before decl\n    let y = FuelStruct {\n        a: true,\n        b: false,\n    };\n\n    // struct and enum with complex members, out of order\n    let u = FuelWrapper {\n        a: y,\n        b: z,\n    };\n\n    let v = WrapperEnum::Variant(u);\n\n    if let AnEnum::Variant = AnEnum::Variant {\n        void();\n    }\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 90,
                        end: 567,
                        as_str(): "fn main() -> bool {\n    let a = 42;\n\n    // fn before decl\n    let x = the_number_five();\n\n    // enum before decl\n    let z = AnEnum::Variant;\n\n    // struct before decl\n    let y = FuelStruct {\n        a: true,\n        b: false,\n    };\n\n    // struct and enum with complex members, out of order\n    let u = FuelWrapper {\n        a: y,\n        b: z,\n    };\n\n    let v = WrapperEnum::Variant(u);\n\n    if let AnEnum::Variant = AnEnum::Variant {\n        void();\n    }\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 103,
                        end: 107,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 90,
            end: 567,
            as_str(): "fn main() -> bool {\n    let a = 42;\n\n    // fn before decl\n    let x = the_number_five();\n\n    // enum before decl\n    let z = AnEnum::Variant;\n\n    // struct before decl\n    let y = FuelStruct {\n        a: true,\n        b: false,\n    };\n\n    // struct and enum with complex members, out of order\n    let u = FuelWrapper {\n        a: y,\n        b: z,\n    };\n\n    let v = WrapperEnum::Variant(u);\n\n    if let AnEnum::Variant = AnEnum::Variant {\n        void();\n    }\n\n    true\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 576,
                            end: 587,
                            as_str(): "FuelWrapper",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 594,
                                    end: 595,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 597,
                                        end: 607,
                                        as_str(): "FuelStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 594,
                                end: 607,
                                as_str(): "a: FuelStruct",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 597,
                                end: 607,
                                as_str(): "FuelStruct",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 613,
                                    end: 614,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 616,
                                        end: 622,
                                        as_str(): "AnEnum",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 613,
                                end: 622,
                                as_str(): "b: AnEnum",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 616,
                                end: 622,
                                as_str(): "AnEnum",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 569,
                        end: 625,
                        as_str(): "struct FuelWrapper {\n    a: FuelStruct,\n    b: AnEnum,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 569,
            end: 625,
            as_str(): "struct FuelWrapper {\n    a: FuelStruct,\n    b: AnEnum,\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 632,
                            end: 643,
                            as_str(): "WrapperEnum",
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
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 650,
                                    end: 657,
                                    as_str(): "Variant",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 659,
                                        end: 670,
                                        as_str(): "FuelWrapper",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 659,
                                end: 670,
                                as_str(): "FuelWrapper",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 650,
                                end: 670,
                                as_str(): "Variant: FuelWrapper",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 627,
                        end: 673,
                        as_str(): "enum WrapperEnum {\n    Variant: FuelWrapper,\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 627,
            end: 673,
            as_str(): "enum WrapperEnum {\n    Variant: FuelWrapper,\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 682,
                            end: 692,
                            as_str(): "FuelStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 699,
                                    end: 700,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Boolean,
                            span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 699,
                                end: 706,
                                as_str(): "a: bool",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 702,
                                end: 706,
                                as_str(): "bool",
                            },
                        },
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 712,
                                    end: 713,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Boolean,
                            span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 712,
                                end: 719,
                                as_str(): "b: bool",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 715,
                                end: 719,
                                as_str(): "bool",
                            },
                        },
                    ],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 675,
                        end: 722,
                        as_str(): "struct FuelStruct {\n    a: bool,\n    b: bool,\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 675,
            end: 722,
            as_str(): "struct FuelStruct {\n    a: bool,\n    b: bool,\n}",
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
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 727,
                            end: 742,
                            as_str(): "the_number_five",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                5,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 758,
                                            end: 759,
                                            as_str(): "5",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 758,
                                    end: 759,
                                    as_str(): "5",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 752,
                            end: 761,
                            as_str(): "{\n    5\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 724,
                        end: 761,
                        as_str(): "fn the_number_five() -> u64 {\n    5\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 748,
                        end: 751,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 724,
            end: 761,
            as_str(): "fn the_number_five() -> u64 {\n    5\n}",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 768,
                            end: 774,
                            as_str(): "AnEnum",
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
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 781,
                                    end: 788,
                                    as_str(): "Variant",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 790,
                                end: 792,
                                as_str(): "()",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 781,
                                end: 792,
                                as_str(): "Variant: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 763,
                        end: 795,
                        as_str(): "enum AnEnum {\n    Variant: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 763,
            end: 795,
            as_str(): "enum AnEnum {\n    Variant: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 823,
                                end: 832,
                                as_str(): "FuelTrait",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 837,
                        end: 840,
                        as_str(): "u64",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 850,
                                    end: 853,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 874,
                                                    end: 878,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 874,
                                            end: 878,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 864,
                                    end: 884,
                                    as_str(): "{\n        true\n    }",
                                },
                            },
                            parameters: [],
                            span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 847,
                                end: 884,
                                as_str(): "fn foo() -> bool {\n        true\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 859,
                                end: 863,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 818,
                        end: 886,
                        as_str(): "impl FuelTrait for u64 {\n    fn foo() -> bool {\n        true\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 818,
            end: 886,
            as_str(): "impl FuelTrait for u64 {\n    fn foo() -> bool {\n        true\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            TraitDeclaration(
                TraitDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 894,
                            end: 903,
                            as_str(): "FuelTrait",
                        },
                        is_raw_ident: false,
                    },
                    type_parameters: [],
                    attributes: {},
                    interface_surface: [
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 913,
                                    end: 916,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [],
                            return_type: Boolean,
                            return_type_span: Span {
                                src (ptr): 0x00007fe06cba43c0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                ),
                                start: 922,
                                end: 926,
                                as_str(): "bool",
                            },
                        },
                    ],
                    methods: [],
                    supertraits: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 888,
                        end: 929,
                        as_str(): "trait FuelTrait {\n    fn foo() -> bool;\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 888,
            end: 929,
            as_str(): "trait FuelTrait {\n    fn foo() -> bool;\n}",
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
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 934,
                            end: 938,
                            as_str(): "void",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 941,
                            end: 944,
                            as_str(): "{\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 931,
                        end: 944,
                        as_str(): "fn void() {\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe06cba43c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                        ),
                        start: 931,
                        end: 940,
                        as_str(): "fn void()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 931,
            end: 944,
            as_str(): "fn void() {\n}",
        },
    },
]
