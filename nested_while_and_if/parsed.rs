[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 13,
                            end: 17,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe07c8bc640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
            ),
            start: 9,
            end: 21,
            as_str(): "use core::*;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 26,
                            end: 29,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 31,
                            end: 37,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 39,
                            end: 45,
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
            src (ptr): 0x00007fe07c8bc640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
            ),
            start: 22,
            end: 46,
            as_str(): "use std::assert::assert;",
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
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 51,
                            end: 54,
                            as_str(): "foo",
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
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 95,
                                                    end: 100,
                                                    as_str(): "index",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 103,
                                                    end: 104,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 87,
                                    end: 105,
                                    as_str(): "let mut index = 0;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 118,
                                                    end: 121,
                                                    as_str(): "sum",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        0,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 124,
                                                    end: 125,
                                                    as_str(): "0",
                                                },
                                            },
                                            is_mutable: true,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 110,
                                    end: 126,
                                    as_str(): "let mut sum = 0;",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: WhileLoop(
                                            WhileLoopExpression {
                                                condition: Expression {
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
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 143,
                                                                                    end: 144,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 143,
                                                                                    end: 144,
                                                                                    as_str(): "<",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: Some(
                                                                                "lt",
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 143,
                                                                                end: 144,
                                                                                as_str(): "<",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 143,
                                                                    end: 144,
                                                                    as_str(): "<",
                                                                },
                                                            },
                                                            contract_call_params: [],
                                                            arguments: [
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 137,
                                                                                end: 142,
                                                                                as_str(): "index",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 137,
                                                                        end: 142,
                                                                        as_str(): "index",
                                                                    },
                                                                },
                                                                Expression {
                                                                    kind: Variable(
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                ),
                                                                                start: 145,
                                                                                end: 146,
                                                                                as_str(): "n",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 145,
                                                                        end: 146,
                                                                        as_str(): "n",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 137,
                                                        end: 146,
                                                        as_str(): "index < n",
                                                    },
                                                },
                                                body: CodeBlock {
                                                    contents: [
                                                        AstNode {
                                                            content: Expression(
                                                                Expression {
                                                                    kind: If(
                                                                        IfExpression {
                                                                            condition: Expression {
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
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 170,
                                                                                                                end: 172,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 170,
                                                                                                                end: 172,
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
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 170,
                                                                                                            end: 172,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                ),
                                                                                                start: 170,
                                                                                                end: 172,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
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
                                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 166,
                                                                                                                                end: 167,
                                                                                                                                as_str(): "%",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: Some(
                                                                                                                                "ops",
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 166,
                                                                                                                                end: 167,
                                                                                                                                as_str(): "%",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ],
                                                                                                                    suffix: BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "modulo",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 166,
                                                                                                                            end: 167,
                                                                                                                            as_str(): "%",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    is_absolute: true,
                                                                                                                },
                                                                                                            },
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 166,
                                                                                                                end: 167,
                                                                                                                as_str(): "%",
                                                                                                            },
                                                                                                        },
                                                                                                        contract_call_params: [],
                                                                                                        arguments: [
                                                                                                            Expression {
                                                                                                                kind: Variable(
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 160,
                                                                                                                            end: 165,
                                                                                                                            as_str(): "index",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 160,
                                                                                                                    end: 165,
                                                                                                                    as_str(): "index",
                                                                                                                },
                                                                                                            },
                                                                                                            Expression {
                                                                                                                kind: Literal(
                                                                                                                    Numeric(
                                                                                                                        2,
                                                                                                                    ),
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 168,
                                                                                                                    end: 169,
                                                                                                                    as_str(): "2",
                                                                                                                },
                                                                                                            },
                                                                                                        ],
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                    ),
                                                                                                    start: 160,
                                                                                                    end: 169,
                                                                                                    as_str(): "index % 2",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                    ),
                                                                                                    start: 173,
                                                                                                    end: 174,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 160,
                                                                                    end: 174,
                                                                                    as_str(): "index % 2 == 0",
                                                                                },
                                                                            },
                                                                            then: Expression {
                                                                                kind: CodeBlock(
                                                                                    CodeBlock {
                                                                                        contents: [
                                                                                            AstNode {
                                                                                                content: Expression(
                                                                                                    Expression {
                                                                                                        kind: Reassignment(
                                                                                                            ReassignmentExpression {
                                                                                                                lhs: VariableExpression(
                                                                                                                    Expression {
                                                                                                                        kind: Variable(
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 189,
                                                                                                                                    end: 192,
                                                                                                                                    as_str(): "sum",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 189,
                                                                                                                            end: 192,
                                                                                                                            as_str(): "sum",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                rhs: Expression {
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
                                                                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 199,
                                                                                                                                                    end: 200,
                                                                                                                                                    as_str(): "+",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: Some(
                                                                                                                                                    "ops",
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 199,
                                                                                                                                                    end: 200,
                                                                                                                                                    as_str(): "+",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                        ],
                                                                                                                                        suffix: BaseIdent {
                                                                                                                                            name_override_opt: Some(
                                                                                                                                                "add",
                                                                                                                                            ),
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 199,
                                                                                                                                                end: 200,
                                                                                                                                                as_str(): "+",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        is_absolute: true,
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                type_arguments: [],
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 199,
                                                                                                                                    end: 200,
                                                                                                                                    as_str(): "+",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            contract_call_params: [],
                                                                                                                            arguments: [
                                                                                                                                Expression {
                                                                                                                                    kind: Variable(
                                                                                                                                        BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 195,
                                                                                                                                                end: 198,
                                                                                                                                                as_str(): "sum",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 195,
                                                                                                                                        end: 198,
                                                                                                                                        as_str(): "sum",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                Expression {
                                                                                                                                    kind: Variable(
                                                                                                                                        BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 201,
                                                                                                                                                end: 206,
                                                                                                                                                as_str(): "index",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 201,
                                                                                                                                        end: 206,
                                                                                                                                        as_str(): "index",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ],
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 195,
                                                                                                                        end: 206,
                                                                                                                        as_str(): "sum + index",
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 189,
                                                                                                            end: 206,
                                                                                                            as_str(): "sum = sum + index",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                    ),
                                                                                                    start: 189,
                                                                                                    end: 206,
                                                                                                    as_str(): "sum = sum + index",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        whole_block_span: Span {
                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                            ),
                                                                                            start: 175,
                                                                                            end: 217,
                                                                                            as_str(): "{\n            sum = sum + index;\n        }",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 175,
                                                                                    end: 217,
                                                                                    as_str(): "{\n            sum = sum + index;\n        }",
                                                                                },
                                                                            },
                                                                            else: None,
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 157,
                                                                        end: 217,
                                                                        as_str(): "if index % 2 == 0 {\n            sum = sum + index;\n        }",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 217,
                                                                as_str(): "if index % 2 == 0 {\n            sum = sum + index;\n        }",
                                                            },
                                                        },
                                                        AstNode {
                                                            content: Expression(
                                                                Expression {
                                                                    kind: Reassignment(
                                                                        ReassignmentExpression {
                                                                            lhs: VariableExpression(
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                ),
                                                                                                start: 227,
                                                                                                end: 232,
                                                                                                as_str(): "index",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                        ),
                                                                                        start: 227,
                                                                                        end: 232,
                                                                                        as_str(): "index",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            rhs: Expression {
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
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 241,
                                                                                                                end: 242,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                                ),
                                                                                                                start: 241,
                                                                                                                end: 242,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "add",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 241,
                                                                                                            end: 242,
                                                                                                            as_str(): "+",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe07c8bc640,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                ),
                                                                                                start: 241,
                                                                                                end: 242,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                            ),
                                                                                                            start: 235,
                                                                                                            end: 240,
                                                                                                            as_str(): "index",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                    ),
                                                                                                    start: 235,
                                                                                                    end: 240,
                                                                                                    as_str(): "index",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                                    ),
                                                                                                    start: 243,
                                                                                                    end: 244,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 235,
                                                                                    end: 244,
                                                                                    as_str(): "index + 1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 227,
                                                                        end: 244,
                                                                        as_str(): "index = index + 1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 227,
                                                                end: 244,
                                                                as_str(): "index = index + 1",
                                                            },
                                                        },
                                                    ],
                                                    whole_block_span: Span {
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 147,
                                                        end: 251,
                                                        as_str(): "{\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }",
                                                    },
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 131,
                                            end: 251,
                                            as_str(): "while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 131,
                                    end: 251,
                                    as_str(): "while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
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
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 260,
                                                                        end: 261,
                                                                        as_str(): "+",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "ops",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 260,
                                                                        end: 261,
                                                                        as_str(): "+",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            suffix: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "add",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 260,
                                                                    end: 261,
                                                                    as_str(): "+",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            is_absolute: true,
                                                        },
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 260,
                                                        end: 261,
                                                        as_str(): "+",
                                                    },
                                                },
                                                contract_call_params: [],
                                                arguments: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 256,
                                                                    end: 259,
                                                                    as_str(): "sum",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 256,
                                                            end: 259,
                                                            as_str(): "sum",
                                                        },
                                                    },
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 262,
                                                                    end: 266,
                                                                    as_str(): "init",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 262,
                                                            end: 266,
                                                            as_str(): "init",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 256,
                                            end: 266,
                                            as_str(): "sum + init",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 256,
                                    end: 266,
                                    as_str(): "sum + init",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 81,
                            end: 269,
                            as_str(): "{\n    let mut index = 0;\n    let mut sum = 0;\n    while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }\n    sum + init \n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 55,
                                    end: 59,
                                    as_str(): "init",
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
                                src (ptr): 0x00007fe07c8bc640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                ),
                                start: 61,
                                end: 64,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 66,
                                    end: 67,
                                    as_str(): "n",
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
                                src (ptr): 0x00007fe07c8bc640,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                ),
                                start: 69,
                                end: 72,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe07c8bc640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                        ),
                        start: 48,
                        end: 269,
                        as_str(): "fn foo(init: u64, n: u64) -> u64 {\n    let mut index = 0;\n    let mut sum = 0;\n    while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }\n    sum + init \n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe07c8bc640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                        ),
                        start: 77,
                        end: 80,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe07c8bc640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
            ),
            start: 48,
            end: 269,
            as_str(): "fn foo(init: u64, n: u64) -> u64 {\n    let mut index = 0;\n    let mut sum = 0;\n    while index < n {\n        if index % 2 == 0 {\n            sum = sum + index;\n        };\n        index = index + 1;\n    }\n    sum + init \n}",
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
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 274,
                            end: 278,
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
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 299,
                                                    end: 300,
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
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 303,
                                                                        end: 306,
                                                                        as_str(): "foo",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 303,
                                                                end: 306,
                                                                as_str(): "foo",
                                                            },
                                                        },
                                                        arguments: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        11,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 307,
                                                                    end: 309,
                                                                    as_str(): "11",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        4,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                    ),
                                                                    start: 311,
                                                                    end: 312,
                                                                    as_str(): "4",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe07c8bc640,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                    ),
                                                    start: 303,
                                                    end: 313,
                                                    as_str(): "foo(11, 4)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 295,
                                    end: 314,
                                    as_str(): "let x = foo(11, 4);",
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
                                                                src (ptr): 0x00007fe07c8bc640,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                ),
                                                                start: 319,
                                                                end: 325,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe07c8bc640,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                        ),
                                                        start: 319,
                                                        end: 325,
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
                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                        ),
                                                                                        start: 328,
                                                                                        end: 330,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                        ),
                                                                                        start: 328,
                                                                                        end: 330,
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
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 328,
                                                                                    end: 330,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe07c8bc640,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                        ),
                                                                        start: 328,
                                                                        end: 330,
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
                                                                                    src (ptr): 0x00007fe07c8bc640,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                                    ),
                                                                                    start: 326,
                                                                                    end: 327,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 326,
                                                                            end: 327,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                13,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe07c8bc640,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                                            ),
                                                                            start: 331,
                                                                            end: 333,
                                                                            as_str(): "13",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe07c8bc640,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                                            ),
                                                            start: 326,
                                                            end: 333,
                                                            as_str(): "x == 13",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 319,
                                            end: 334,
                                            as_str(): "assert(x == 13)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 319,
                                    end: 334,
                                    as_str(): "assert(x == 13)",
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
                                            src (ptr): 0x00007fe07c8bc640,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                            ),
                                            start: 340,
                                            end: 344,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe07c8bc640,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                                    ),
                                    start: 340,
                                    end: 344,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe07c8bc640,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                            ),
                            start: 289,
                            end: 346,
                            as_str(): "{\n    let x = foo(11, 4);\n    assert(x == 13);\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe07c8bc640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                        ),
                        start: 271,
                        end: 346,
                        as_str(): "fn main() -> bool {\n    let x = foo(11, 4);\n    assert(x == 13);\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe07c8bc640,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
                        ),
                        start: 284,
                        end: 288,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe07c8bc640,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQA5O18/nested_while_and_if/src/main.sw",
            ),
            start: 271,
            end: 346,
            as_str(): "fn main() -> bool {\n    let x = foo(11, 4);\n    assert(x == 13);\n    true\n}",
        },
    },
]
