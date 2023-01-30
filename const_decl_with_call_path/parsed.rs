[
    AstNode {
        content: IncludeStatement(
            IncludeStatement {
                _alias: None,
                span: Span {
                    src (ptr): 0x00007fb110d20660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                    ),
                    start: 9,
                    end: 22,
                    as_str(): "dep test_lib;",
                },
                _path_span: Span {
                    src (ptr): 0x00007fb110d20660,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                    ),
                    start: 13,
                    end: 21,
                    as_str(): "test_lib",
                },
            },
        ),
        span: Span {
            src (ptr): 0x00007fb110d20660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
            ),
            start: 9,
            end: 22,
            as_str(): "dep test_lib;",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 28,
                            end: 31,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 34,
                            end: 40,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 42,
                            end: 48,
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
            src (ptr): 0x00007fb110d20660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
            ),
            start: 24,
            end: 75,
            as_str(): "use std::{assert::assert, contract_id::ContractId};",
        },
    },
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 28,
                            end: 31,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 50,
                            end: 61,
                            as_str(): "contract_id",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 63,
                            end: 73,
                            as_str(): "ContractId",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fb110d20660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
            ),
            start: 24,
            end: 75,
            as_str(): "use std::{assert::assert, contract_id::ContractId};",
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
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 80,
                            end: 84,
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
                                                    src (ptr): 0x00007fb110d20660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                    ),
                                                    start: 104,
                                                    end: 105,
                                                    as_str(): "x",
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
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 108,
                                                                            end: 116,
                                                                            as_str(): "test_lib",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 118,
                                                                        end: 124,
                                                                        as_str(): "NUMBER",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 118,
                                                                end: 124,
                                                                as_str(): "NUMBER",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb110d20660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                    ),
                                                    start: 108,
                                                    end: 124,
                                                    as_str(): "test_lib::NUMBER",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 100,
                                    end: 125,
                                    as_str(): "let x = test_lib::NUMBER;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb110d20660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                    ),
                                                    start: 134,
                                                    end: 138,
                                                    as_str(): "zero",
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
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 141,
                                                                            end: 144,
                                                                            as_str(): "std",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 146,
                                                                            end: 155,
                                                                            as_str(): "constants",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 157,
                                                                        end: 166,
                                                                        as_str(): "ZERO_B256",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 157,
                                                                end: 166,
                                                                as_str(): "ZERO_B256",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb110d20660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                    ),
                                                    start: 141,
                                                    end: 166,
                                                    as_str(): "std::constants::ZERO_B256",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 130,
                                    end: 167,
                                    as_str(): "let zero = std::constants::ZERO_B256;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb110d20660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                    ),
                                                    start: 176,
                                                    end: 189,
                                                    as_str(): "base_asset_id",
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
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 192,
                                                                            end: 195,
                                                                            as_str(): "std",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 197,
                                                                            end: 206,
                                                                            as_str(): "constants",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 208,
                                                                        end: 221,
                                                                        as_str(): "BASE_ASSET_ID",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 208,
                                                                end: 221,
                                                                as_str(): "BASE_ASSET_ID",
                                                            },
                                                        },
                                                        args: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb110d20660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                    ),
                                                    start: 192,
                                                    end: 221,
                                                    as_str(): "std::constants::BASE_ASSET_ID",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 172,
                                    end: 222,
                                    as_str(): "let base_asset_id = std::constants::BASE_ASSET_ID;",
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
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 227,
                                                                end: 233,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb110d20660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                        ),
                                                        start: 227,
                                                        end: 233,
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
                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                        ),
                                                                                        start: 239,
                                                                                        end: 241,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                        ),
                                                                                        start: 239,
                                                                                        end: 241,
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
                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                    ),
                                                                                    start: 239,
                                                                                    end: 241,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 239,
                                                                        end: 241,
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
                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                    ),
                                                                                    start: 234,
                                                                                    end: 238,
                                                                                    as_str(): "zero",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 234,
                                                                            end: 238,
                                                                            as_str(): "zero",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            B256(
                                                                                [
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                    0,
                                                                                ],
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 242,
                                                                            end: 308,
                                                                            as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 234,
                                                            end: 308,
                                                            as_str(): "zero == 0x0000000000000000000000000000000000000000000000000000000000000000",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 227,
                                            end: 309,
                                            as_str(): "assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 227,
                                    end: 309,
                                    as_str(): "assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000)",
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
                                                                src (ptr): 0x00007fb110d20660,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                ),
                                                                start: 315,
                                                                end: 321,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb110d20660,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                        ),
                                                        start: 315,
                                                        end: 321,
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
                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                        ),
                                                                                        start: 336,
                                                                                        end: 338,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                        ),
                                                                                        start: 336,
                                                                                        end: 338,
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
                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                    ),
                                                                                    start: 336,
                                                                                    end: 338,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110d20660,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                        ),
                                                                        start: 336,
                                                                        end: 338,
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
                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                    ),
                                                                                    start: 322,
                                                                                    end: 335,
                                                                                    as_str(): "base_asset_id",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 322,
                                                                            end: 335,
                                                                            as_str(): "base_asset_id",
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
                                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                                        ),
                                                                                                        start: 339,
                                                                                                        end: 349,
                                                                                                        as_str(): "ContractId",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                                    ),
                                                                                                    start: 339,
                                                                                                    end: 349,
                                                                                                    as_str(): "ContractId",
                                                                                                },
                                                                                            },
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                                    ),
                                                                                                    start: 351,
                                                                                                    end: 355,
                                                                                                    as_str(): "from",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb110d20660,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                        ),
                                                                                        start: 339,
                                                                                        end: 355,
                                                                                        as_str(): "ContractId::from",
                                                                                    },
                                                                                },
                                                                                args: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb110d20660,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                                    ),
                                                                                                    start: 356,
                                                                                                    end: 360,
                                                                                                    as_str(): "zero",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb110d20660,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                                            ),
                                                                                            start: 356,
                                                                                            end: 360,
                                                                                            as_str(): "zero",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb110d20660,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                                            ),
                                                                            start: 339,
                                                                            end: 361,
                                                                            as_str(): "ContractId::from(zero)",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb110d20660,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                            ),
                                                            start: 322,
                                                            end: 361,
                                                            as_str(): "base_asset_id == ContractId::from(zero)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 315,
                                            end: 362,
                                            as_str(): "assert(base_asset_id == ContractId::from(zero))",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 315,
                                    end: 362,
                                    as_str(): "assert(base_asset_id == ContractId::from(zero))",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb110d20660,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                                    ),
                                                    start: 368,
                                                    end: 369,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb110d20660,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                            ),
                                            start: 368,
                                            end: 369,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb110d20660,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                                    ),
                                    start: 368,
                                    end: 369,
                                    as_str(): "x",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb110d20660,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                            ),
                            start: 94,
                            end: 371,
                            as_str(): "{\n    let x = test_lib::NUMBER;\n    let zero = std::constants::ZERO_B256;\n    let base_asset_id = std::constants::BASE_ASSET_ID;\n    assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000);\n    assert(base_asset_id == ContractId::from(zero));\n    x\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb110d20660,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                        ),
                        start: 77,
                        end: 371,
                        as_str(): "fn main() -> u64 {\n    let x = test_lib::NUMBER;\n    let zero = std::constants::ZERO_B256;\n    let base_asset_id = std::constants::BASE_ASSET_ID;\n    assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000);\n    assert(base_asset_id == ContractId::from(zero));\n    x\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb110d20660,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
                        ),
                        start: 90,
                        end: 93,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb110d20660,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR3aWVLr/const_decl_with_call_path/src/main.sw",
            ),
            start: 77,
            end: 371,
            as_str(): "fn main() -> u64 {\n    let x = test_lib::NUMBER;\n    let zero = std::constants::ZERO_B256;\n    let base_asset_id = std::constants::BASE_ASSET_ID;\n    assert(zero == 0x0000000000000000000000000000000000000000000000000000000000000000);\n    assert(base_asset_id == ContractId::from(zero));\n    x\n}",
        },
    },
]
