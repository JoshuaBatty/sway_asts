[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fe08b1f4f00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                    ),
                    start: 9,
                    end: 17,
                    as_str(): "dep bar;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fe08b1f4f00,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                    ),
                    start: 13,
                    end: 16,
                    as_str(): "bar",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fe08b1f4f00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
            ),
            start: 9,
            end: 17,
            as_str(): "dep bar;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 25,
                            end: 28,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 31,
                            end: 35,
                            as_str(): "Bar1",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: true,
                alias: Some(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 39,
                            end: 45,
                            as_str(): "MyBar1",
                        },
                        is_raw_ident: false,
                    },
                ),
            },
        ),
        span: Span {
            src (ptr): 0x00007fe08b1f4f00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
            ),
            start: 19,
            end: 151,
            as_str(): "use ::bar::{Bar1 as MyBar1, Bar2, double_bar::{DoubleBar1::{self as MyDoubleBar1}, DoubleBar2::{self as MyDoubleBar2}, DoubleBar3}};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 25,
                            end: 28,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 47,
                            end: 51,
                            as_str(): "Bar2",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: true,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe08b1f4f00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
            ),
            start: 19,
            end: 151,
            as_str(): "use ::bar::{Bar1 as MyBar1, Bar2, double_bar::{DoubleBar1::{self as MyDoubleBar1}, DoubleBar2::{self as MyDoubleBar2}, DoubleBar3}};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 25,
                            end: 28,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 53,
                            end: 63,
                            as_str(): "double_bar",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 66,
                            end: 76,
                            as_str(): "DoubleBar1",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: SelfImport,
                is_absolute: true,
                alias: Some(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 87,
                            end: 99,
                            as_str(): "MyDoubleBar1",
                        },
                        is_raw_ident: false,
                    },
                ),
            },
        ),
        span: Span {
            src (ptr): 0x00007fe08b1f4f00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
            ),
            start: 19,
            end: 151,
            as_str(): "use ::bar::{Bar1 as MyBar1, Bar2, double_bar::{DoubleBar1::{self as MyDoubleBar1}, DoubleBar2::{self as MyDoubleBar2}, DoubleBar3}};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 25,
                            end: 28,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 53,
                            end: 63,
                            as_str(): "double_bar",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 102,
                            end: 112,
                            as_str(): "DoubleBar2",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: SelfImport,
                is_absolute: true,
                alias: Some(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 123,
                            end: 135,
                            as_str(): "MyDoubleBar2",
                        },
                        is_raw_ident: false,
                    },
                ),
            },
        ),
        span: Span {
            src (ptr): 0x00007fe08b1f4f00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
            ),
            start: 19,
            end: 151,
            as_str(): "use ::bar::{Bar1 as MyBar1, Bar2, double_bar::{DoubleBar1::{self as MyDoubleBar1}, DoubleBar2::{self as MyDoubleBar2}, DoubleBar3}};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 25,
                            end: 28,
                            as_str(): "bar",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 53,
                            end: 63,
                            as_str(): "double_bar",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 138,
                            end: 148,
                            as_str(): "DoubleBar3",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: true,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe08b1f4f00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
            ),
            start: 19,
            end: 151,
            as_str(): "use ::bar::{Bar1 as MyBar1, Bar2, double_bar::{DoubleBar1::{self as MyDoubleBar1}, DoubleBar2::{self as MyDoubleBar2}, DoubleBar3}};",
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
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 156,
                            end: 160,
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
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 181,
                                                    end: 185,
                                                    as_str(): "bar1",
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 188,
                                                                        end: 194,
                                                                        as_str(): "MyBar1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 188,
                                                                end: 194,
                                                                as_str(): "MyBar1",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 205,
                                                                        end: 206,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U32(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 208,
                                                                        end: 212,
                                                                        as_str(): "5u32",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 205,
                                                                    end: 212,
                                                                    as_str(): "a: 5u32",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 188,
                                                    end: 219,
                                                    as_str(): "MyBar1 {\n        a: 5u32,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 177,
                                    end: 220,
                                    as_str(): "let bar1 = MyBar1 {\n        a: 5u32,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 229,
                                                    end: 233,
                                                    as_str(): "bar2",
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 236,
                                                                        end: 240,
                                                                        as_str(): "Bar2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 236,
                                                                end: 240,
                                                                as_str(): "Bar2",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 251,
                                                                        end: 252,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 254,
                                                                        end: 258,
                                                                        as_str(): "5u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 251,
                                                                    end: 258,
                                                                    as_str(): "a: 5u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 236,
                                                    end: 265,
                                                    as_str(): "Bar2 {\n        a: 5u64,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 225,
                                    end: 266,
                                    as_str(): "let bar2 = Bar2 {\n        a: 5u64,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 275,
                                                    end: 278,
                                                    as_str(): "db1",
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 281,
                                                                        end: 293,
                                                                        as_str(): "MyDoubleBar1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 281,
                                                                end: 293,
                                                                as_str(): "MyDoubleBar1",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 304,
                                                                        end: 305,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U32(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 307,
                                                                        end: 311,
                                                                        as_str(): "5u32",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 304,
                                                                    end: 311,
                                                                    as_str(): "a: 5u32",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 281,
                                                    end: 318,
                                                    as_str(): "MyDoubleBar1 {\n        a: 5u32,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 271,
                                    end: 319,
                                    as_str(): "let db1 = MyDoubleBar1 {\n        a: 5u32,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 328,
                                                    end: 331,
                                                    as_str(): "db2",
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 334,
                                                                        end: 346,
                                                                        as_str(): "MyDoubleBar2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 334,
                                                                end: 346,
                                                                as_str(): "MyDoubleBar2",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 357,
                                                                        end: 358,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 360,
                                                                        end: 364,
                                                                        as_str(): "5u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 357,
                                                                    end: 364,
                                                                    as_str(): "a: 5u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 334,
                                                    end: 371,
                                                    as_str(): "MyDoubleBar2 {\n        a: 5u64,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 324,
                                    end: 372,
                                    as_str(): "let db2 = MyDoubleBar2 {\n        a: 5u64,\n    };",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 381,
                                                    end: 384,
                                                    as_str(): "db3",
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
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 387,
                                                                        end: 397,
                                                                        as_str(): "DoubleBar3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe08b1f4f00,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                ),
                                                                start: 387,
                                                                end: 397,
                                                                as_str(): "DoubleBar3",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 408,
                                                                        end: 409,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: Literal(
                                                                        U64(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe08b1f4f00,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                        ),
                                                                        start: 411,
                                                                        end: 415,
                                                                        as_str(): "5u64",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe08b1f4f00,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                                    ),
                                                                    start: 408,
                                                                    end: 415,
                                                                    as_str(): "a: 5u64",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe08b1f4f00,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                                    ),
                                                    start: 387,
                                                    end: 422,
                                                    as_str(): "DoubleBar3 {\n        a: 5u64,\n    }",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 377,
                                    end: 423,
                                    as_str(): "let db3 = DoubleBar3 {\n        a: 5u64,\n    };",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                false,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe08b1f4f00,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                            ),
                                            start: 428,
                                            end: 433,
                                            as_str(): "false",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe08b1f4f00,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                                    ),
                                    start: 428,
                                    end: 433,
                                    as_str(): "false",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe08b1f4f00,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                            ),
                            start: 171,
                            end: 435,
                            as_str(): "{\n    let bar1 = MyBar1 {\n        a: 5u32,\n    };\n    let bar2 = Bar2 {\n        a: 5u64,\n    };\n    let db1 = MyDoubleBar1 {\n        a: 5u32,\n    };\n    let db2 = MyDoubleBar2 {\n        a: 5u64,\n    };\n    let db3 = DoubleBar3 {\n        a: 5u64,\n    };\n    false\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe08b1f4f00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                        ),
                        start: 153,
                        end: 435,
                        as_str(): "fn main() -> bool {\n    let bar1 = MyBar1 {\n        a: 5u32,\n    };\n    let bar2 = Bar2 {\n        a: 5u64,\n    };\n    let db1 = MyDoubleBar1 {\n        a: 5u32,\n    };\n    let db2 = MyDoubleBar2 {\n        a: 5u64,\n    };\n    let db3 = DoubleBar3 {\n        a: 5u64,\n    };\n    false\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe08b1f4f00,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
                        ),
                        start: 166,
                        end: 170,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe08b1f4f00,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHfiYR1/multi_item_import/src/main.sw",
            ),
            start: 153,
            end: 435,
            as_str(): "fn main() -> bool {\n    let bar1 = MyBar1 {\n        a: 5u32,\n    };\n    let bar2 = Bar2 {\n        a: 5u64,\n    };\n    let db1 = MyDoubleBar1 {\n        a: 5u32,\n    };\n    let db2 = MyDoubleBar2 {\n        a: 5u64,\n    };\n    let db3 = DoubleBar3 {\n        a: 5u64,\n    };\n    false\n}",
        },
    },
]
