[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 9,
                    end: 21,
                    as_str(): "dep context;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 13,
                    end: 20,
                    as_str(): "context",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 9,
            end: 21,
            as_str(): "dep context;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 22,
                    end: 32,
                    as_str(): "dep asset;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 26,
                    end: 31,
                    as_str(): "asset",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 22,
            end: 32,
            as_str(): "dep asset;",
        },
    },
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 33,
                    end: 43,
                    as_str(): "dep utils;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe0dac0ad90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                    ),
                    start: 37,
                    end: 42,
                    as_str(): "utils",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 33,
            end: 43,
            as_str(): "dep utils;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 49,
                            end: 56,
                            as_str(): "context",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 58,
                            end: 65,
                            as_str(): "Context",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 45,
            end: 66,
            as_str(): "use context::Context;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 71,
                            end: 76,
                            as_str(): "utils",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 78,
                            end: 85,
                            as_str(): "Wrapper",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 67,
            end: 86,
            as_str(): "use utils::Wrapper;",
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
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 91,
                            end: 98,
                            as_str(): "eq_test",
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
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 110,
                                                    end: 112,
                                                    as_str(): "w1",
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
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 115,
                                                                                end: 122,
                                                                                as_str(): "Wrapper",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 115,
                                                                            end: 122,
                                                                            as_str(): "Wrapper",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 124,
                                                                            end: 127,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 115,
                                                                end: 127,
                                                                as_str(): "Wrapper::new",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                    ),
                                                                    start: 128,
                                                                    end: 129,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 115,
                                                    end: 130,
                                                    as_str(): "Wrapper::new(3)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 106,
                                    end: 131,
                                    as_str(): "let w1 = Wrapper::new(3);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 139,
                                                    end: 141,
                                                    as_str(): "w2",
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
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 144,
                                                                                end: 151,
                                                                                as_str(): "Wrapper",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 144,
                                                                            end: 151,
                                                                            as_str(): "Wrapper",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 153,
                                                                            end: 156,
                                                                            as_str(): "new",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 144,
                                                                end: 156,
                                                                as_str(): "Wrapper::new",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                    ),
                                                                    start: 157,
                                                                    end: 158,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 144,
                                                    end: 159,
                                                    as_str(): "Wrapper::new(3)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 135,
                                    end: 160,
                                    as_str(): "let w2 = Wrapper::new(3);",
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
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 165,
                                                                end: 171,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 165,
                                                        end: 171,
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
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 175,
                                                                                        end: 177,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 175,
                                                                                        end: 177,
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
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 175,
                                                                                    end: 177,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                        ),
                                                                        start: 175,
                                                                        end: 177,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 172,
                                                                                    end: 174,
                                                                                    as_str(): "w1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 172,
                                                                            end: 174,
                                                                            as_str(): "w1",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 178,
                                                                                    end: 180,
                                                                                    as_str(): "w2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 178,
                                                                            end: 180,
                                                                            as_str(): "w2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 172,
                                                            end: 180,
                                                            as_str(): "w1 == w2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 165,
                                            end: 181,
                                            as_str(): "assert(w1 == w2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 165,
                                    end: 181,
                                    as_str(): "assert(w1 == w2)",
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
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 192,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 186,
                                                        end: 192,
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
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 202,
                                                                                        end: 204,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 202,
                                                                                        end: 204,
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
                                                                                    src (ptr): 0x00007fe0dac0ad90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                    ),
                                                                                    start: 202,
                                                                                    end: 204,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                        ),
                                                                        start: 202,
                                                                        end: 204,
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
                                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                                ),
                                                                                                start: 193,
                                                                                                end: 195,
                                                                                                as_str(): "w1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 193,
                                                                                        end: 195,
                                                                                        as_str(): "w1",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 196,
                                                                                        end: 201,
                                                                                        as_str(): "asset",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 193,
                                                                            end: 201,
                                                                            as_str(): "w1.asset",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                                ),
                                                                                                start: 205,
                                                                                                end: 207,
                                                                                                as_str(): "w2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 205,
                                                                                        end: 207,
                                                                                        as_str(): "w2",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0dac0ad90,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                        ),
                                                                                        start: 208,
                                                                                        end: 213,
                                                                                        as_str(): "asset",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 205,
                                                                            end: 213,
                                                                            as_str(): "w2.asset",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dac0ad90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                            ),
                                                            start: 193,
                                                            end: 213,
                                                            as_str(): "w1.asset == w2.asset",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 214,
                                            as_str(): "assert(w1.asset == w2.asset)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 186,
                                    end: 214,
                                    as_str(): "assert(w1.asset == w2.asset)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 101,
                            end: 217,
                            as_str(): "{\n   let w1 = Wrapper::new(3);\n   let w2 = Wrapper::new(3);\n\n   assert(w1 == w2);\n   assert(w1.asset == w2.asset);\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0dac0ad90,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                        ),
                        start: 88,
                        end: 217,
                        as_str(): "fn eq_test() {\n   let w1 = Wrapper::new(3);\n   let w2 = Wrapper::new(3);\n\n   assert(w1 == w2);\n   assert(w1.asset == w2.asset);\n}",
                    },
                    return_type: Tuple(
                        [],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0dac0ad90,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                        ),
                        start: 88,
                        end: 100,
                        as_str(): "fn eq_test()",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 88,
            end: 217,
            as_str(): "fn eq_test() {\n   let w1 = Wrapper::new(3);\n   let w2 = Wrapper::new(3);\n\n   assert(w1 == w2);\n   assert(w1.asset == w2.asset);\n}",
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
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 222,
                            end: 226,
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
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 241,
                                                                end: 248,
                                                                as_str(): "eq_test",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 241,
                                                        end: 248,
                                                        as_str(): "eq_test",
                                                    },
                                                },
                                                arguments: [],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 241,
                                            end: 250,
                                            as_str(): "eq_test()",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 241,
                                    end: 250,
                                    as_str(): "eq_test()",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 260,
                                                    end: 261,
                                                    as_str(): "x",
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
                                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                                ),
                                                                                start: 264,
                                                                                end: 271,
                                                                                as_str(): "Context",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 264,
                                                                            end: 271,
                                                                            as_str(): "Context",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0dac0ad90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                            ),
                                                                            start: 273,
                                                                            end: 276,
                                                                            as_str(): "foo",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 264,
                                                                end: 276,
                                                                as_str(): "Context::foo",
                                                            },
                                                        },
                                                        args: [],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0dac0ad90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                    ),
                                                    start: 264,
                                                    end: 278,
                                                    as_str(): "Context::foo()",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 256,
                                    end: 279,
                                    as_str(): "let x = Context::foo();",
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
                                                                src (ptr): 0x00007fe0dac0ad90,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                                ),
                                                                start: 283,
                                                                end: 284,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 283,
                                                        end: 284,
                                                        as_str(): "x",
                                                    },
                                                },
                                                field_to_access: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dac0ad90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                                        ),
                                                        start: 285,
                                                        end: 294,
                                                        as_str(): "something",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0dac0ad90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                            ),
                                            start: 283,
                                            end: 294,
                                            as_str(): "x.something",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0dac0ad90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                                    ),
                                    start: 283,
                                    end: 294,
                                    as_str(): "x.something",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0dac0ad90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                            ),
                            start: 236,
                            end: 296,
                            as_str(): "{\n   eq_test();\n\n   let x = Context::foo();\n   x.something\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0dac0ad90,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                        ),
                        start: 219,
                        end: 296,
                        as_str(): "fn main() -> u64 {\n   eq_test();\n\n   let x = Context::foo();\n   x.something\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0dac0ad90,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
                        ),
                        start: 232,
                        end: 235,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0dac0ad90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRnnq3CO/import_method_from_other_file/src/main.sw",
            ),
            start: 219,
            end: 296,
            as_str(): "fn main() -> u64 {\n   eq_test();\n\n   let x = Context::foo();\n   x.something\n}",
        },
    },
]
