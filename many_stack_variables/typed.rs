
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0acfed220,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
            ),
            start: 48,
            end: 49,
            as_str(): "Z",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 56,
                    end: 57,
                    as_str(): "a",
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
                src (ptr): 0x00007fe0acfed220,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                ),
                start: 56,
                end: 62,
                as_str(): "a: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0acfed220,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                ),
                start: 59,
                end: 62,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 68,
                    end: 69,
                    as_str(): "b",
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
                src (ptr): 0x00007fe0acfed220,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                ),
                start: 68,
                end: 74,
                as_str(): "b: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0acfed220,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                ),
                start: 71,
                end: 74,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 80,
                    end: 81,
                    as_str(): "c",
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
                src (ptr): 0x00007fe0acfed220,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                ),
                start: 80,
                end: 86,
                as_str(): "c: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0acfed220,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                ),
                start: 83,
                end: 86,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 92,
                    end: 93,
                    as_str(): "d",
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
                src (ptr): 0x00007fe0acfed220,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                ),
                start: 92,
                end: 98,
                as_str(): "d: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0acfed220,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                ),
                start: 95,
                end: 98,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0acfed220,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
        ),
        start: 41,
        end: 101,
        as_str(): "struct Z {\n    a: u64,\n    b: u64,\n    c: u64,\n    d: u64,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0acfed220,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
            ),
            start: 976,
            end: 979,
            as_str(): "foo",
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
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 992,
                                    end: 993,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 996,
                                    end: 997,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31633,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 988,
                    end: 998,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1007,
                                    end: 1008,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1011,
                                    end: 1012,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31635,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1003,
                    end: 1013,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1022,
                                    end: 1023,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1026,
                                    end: 1027,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31637,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1018,
                    end: 1028,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1037,
                                    end: 1038,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1041,
                                    end: 1042,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31639,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1033,
                    end: 1043,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1052,
                                    end: 1053,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1056,
                                    end: 1057,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31641,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1048,
                    end: 1058,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1067,
                                    end: 1068,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1071,
                                    end: 1072,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31643,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1063,
                    end: 1073,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1082,
                                    end: 1083,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1086,
                                    end: 1087,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31645,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1078,
                    end: 1088,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1097,
                                    end: 1098,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1101,
                                    end: 1102,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31647,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1093,
                    end: 1103,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1112,
                                    end: 1113,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1116,
                                    end: 1117,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31649,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1108,
                    end: 1118,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1127,
                                    end: 1128,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1131,
                                    end: 1132,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31651,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1123,
                    end: 1133,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1142,
                                    end: 1143,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1146,
                                    end: 1147,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31653,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1138,
                    end: 1148,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1157,
                                    end: 1158,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1161,
                                    end: 1162,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31655,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1153,
                    end: 1163,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1172,
                                    end: 1173,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1176,
                                    end: 1177,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31657,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1168,
                    end: 1178,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1187,
                                    end: 1188,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1191,
                                    end: 1192,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31659,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1183,
                    end: 1193,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1202,
                                    end: 1203,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1206,
                                    end: 1207,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31661,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1198,
                    end: 1208,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1217,
                                    end: 1218,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1221,
                                    end: 1222,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31663,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1213,
                    end: 1223,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1232,
                                    end: 1233,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1236,
                                    end: 1237,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31665,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1228,
                    end: 1238,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1247,
                                    end: 1248,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1251,
                                    end: 1252,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31667,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1243,
                    end: 1253,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1262,
                                    end: 1263,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1266,
                                    end: 1267,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31669,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1258,
                    end: 1268,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1277,
                                    end: 1278,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1281,
                                    end: 1282,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31671,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1273,
                    end: 1283,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1292,
                                    end: 1293,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1296,
                                    end: 1297,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31673,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1288,
                    end: 1298,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1307,
                                    end: 1308,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1311,
                                    end: 1312,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31675,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1303,
                    end: 1313,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1322,
                                    end: 1323,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1326,
                                    end: 1327,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31677,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1318,
                    end: 1328,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1337,
                                    end: 1338,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1341,
                                    end: 1342,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31679,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1333,
                    end: 1343,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1352,
                                    end: 1353,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1356,
                                    end: 1357,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31681,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1348,
                    end: 1358,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1367,
                                    end: 1368,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1371,
                                    end: 1372,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31683,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1363,
                    end: 1373,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1382,
                                    end: 1383,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1386,
                                    end: 1387,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31685,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1378,
                    end: 1388,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1397,
                                    end: 1398,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1401,
                                    end: 1402,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31687,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1393,
                    end: 1403,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1412,
                                    end: 1413,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1416,
                                    end: 1417,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31689,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1408,
                    end: 1418,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1427,
                                    end: 1428,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1431,
                                    end: 1432,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31691,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1423,
                    end: 1433,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1442,
                                    end: 1443,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1446,
                                    end: 1447,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31693,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1438,
                    end: 1448,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1457,
                                    end: 1458,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1461,
                                    end: 1462,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31695,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1453,
                    end: 1463,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1472,
                                    end: 1473,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1476,
                                    end: 1477,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31697,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1468,
                    end: 1478,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1487,
                                    end: 1488,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1491,
                                    end: 1492,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31699,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1483,
                    end: 1493,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1502,
                                    end: 1503,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1506,
                                    end: 1507,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31701,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1498,
                    end: 1508,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1517,
                                    end: 1518,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1521,
                                    end: 1522,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31703,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1513,
                    end: 1523,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1532,
                                    end: 1533,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1536,
                                    end: 1537,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31705,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1528,
                    end: 1538,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1547,
                                    end: 1548,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1551,
                                    end: 1552,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31707,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1543,
                    end: 1553,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1562,
                                    end: 1563,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1566,
                                    end: 1567,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31709,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1558,
                    end: 1568,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1577,
                                    end: 1578,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1581,
                                    end: 1582,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31711,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1573,
                    end: 1583,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1592,
                                    end: 1593,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1596,
                                    end: 1597,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31713,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1588,
                    end: 1598,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1607,
                                    end: 1608,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1611,
                                    end: 1612,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31715,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1603,
                    end: 1613,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1622,
                                    end: 1623,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1626,
                                    end: 1627,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31717,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1618,
                    end: 1628,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1637,
                                    end: 1638,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1641,
                                    end: 1642,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31719,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1633,
                    end: 1643,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1652,
                                    end: 1653,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1656,
                                    end: 1657,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31721,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1648,
                    end: 1658,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1667,
                                    end: 1668,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1671,
                                    end: 1672,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31723,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1663,
                    end: 1673,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1682,
                                    end: 1683,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1686,
                                    end: 1687,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31725,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1678,
                    end: 1688,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1697,
                                    end: 1698,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1701,
                                    end: 1702,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31727,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1693,
                    end: 1703,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1712,
                                    end: 1713,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1716,
                                    end: 1717,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31729,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1708,
                    end: 1718,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1727,
                                    end: 1728,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1731,
                                    end: 1732,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31731,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1723,
                    end: 1733,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1742,
                                    end: 1743,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1746,
                                    end: 1747,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31733,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1738,
                    end: 1748,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1757,
                                    end: 1758,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1761,
                                    end: 1762,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31735,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1753,
                    end: 1763,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1772,
                                    end: 1773,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1776,
                                    end: 1777,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31737,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1768,
                    end: 1778,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1787,
                                    end: 1788,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1791,
                                    end: 1792,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31739,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1783,
                    end: 1793,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1802,
                                    end: 1803,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1806,
                                    end: 1807,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31741,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1798,
                    end: 1808,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1817,
                                    end: 1818,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1821,
                                    end: 1822,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31743,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1813,
                    end: 1823,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1832,
                                    end: 1833,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1836,
                                    end: 1837,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31745,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1828,
                    end: 1838,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1847,
                                    end: 1848,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1851,
                                    end: 1852,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31747,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1843,
                    end: 1853,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1862,
                                    end: 1863,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1866,
                                    end: 1867,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31749,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1858,
                    end: 1868,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1877,
                                    end: 1878,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1881,
                                    end: 1882,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31751,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1873,
                    end: 1883,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1892,
                                    end: 1893,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1896,
                                    end: 1897,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31753,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1888,
                    end: 1898,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1907,
                                    end: 1908,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1911,
                                    end: 1912,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31755,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1903,
                    end: 1913,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1922,
                                    end: 1923,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1926,
                                    end: 1927,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31757,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1918,
                    end: 1928,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1937,
                                    end: 1938,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1941,
                                    end: 1942,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31759,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1933,
                    end: 1943,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1952,
                                    end: 1953,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1956,
                                    end: 1957,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31761,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1948,
                    end: 1958,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1967,
                                    end: 1968,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1971,
                                    end: 1972,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31763,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1963,
                    end: 1973,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1982,
                                    end: 1983,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1986,
                                    end: 1987,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31765,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1978,
                    end: 1988,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 1997,
                                    end: 1998,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2001,
                                    end: 2002,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31767,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 1993,
                    end: 2003,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2012,
                                    end: 2013,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2016,
                                    end: 2017,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31769,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2008,
                    end: 2018,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2027,
                                    end: 2028,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2031,
                                    end: 2032,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31771,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2023,
                    end: 2033,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2042,
                                    end: 2043,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2046,
                                    end: 2047,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31773,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2038,
                    end: 2048,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2057,
                                    end: 2058,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2061,
                                    end: 2062,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31775,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2053,
                    end: 2063,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2072,
                                    end: 2073,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2076,
                                    end: 2077,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31777,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2068,
                    end: 2078,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2087,
                                    end: 2088,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2091,
                                    end: 2092,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31779,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2083,
                    end: 2093,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2102,
                                    end: 2103,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2106,
                                    end: 2107,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31781,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2098,
                    end: 2108,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2117,
                                    end: 2118,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2121,
                                    end: 2122,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31783,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2113,
                    end: 2123,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2132,
                                    end: 2133,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2136,
                                    end: 2137,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31785,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2128,
                    end: 2138,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2147,
                                    end: 2148,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2151,
                                    end: 2152,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31787,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2143,
                    end: 2153,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2162,
                                    end: 2163,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2166,
                                    end: 2167,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31789,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2158,
                    end: 2168,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2177,
                                    end: 2178,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2181,
                                    end: 2182,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31791,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2173,
                    end: 2183,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2192,
                                    end: 2193,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2196,
                                    end: 2197,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31793,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2188,
                    end: 2198,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2207,
                                    end: 2208,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2211,
                                    end: 2212,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31795,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2203,
                    end: 2213,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2222,
                                    end: 2223,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2226,
                                    end: 2227,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31797,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2218,
                    end: 2228,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2237,
                                    end: 2238,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2241,
                                    end: 2242,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31799,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2233,
                    end: 2243,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2252,
                                    end: 2253,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2256,
                                    end: 2257,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31801,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2248,
                    end: 2258,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2267,
                                    end: 2268,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2271,
                                    end: 2272,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31803,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2263,
                    end: 2273,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2282,
                                    end: 2283,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2286,
                                    end: 2287,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31805,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2278,
                    end: 2288,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2297,
                                    end: 2298,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2301,
                                    end: 2302,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31807,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2293,
                    end: 2303,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2312,
                                    end: 2313,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2316,
                                    end: 2317,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31809,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2308,
                    end: 2318,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2327,
                                    end: 2328,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2331,
                                    end: 2332,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31811,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2323,
                    end: 2333,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2342,
                                    end: 2343,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2346,
                                    end: 2347,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31813,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2338,
                    end: 2348,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2357,
                                    end: 2358,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2361,
                                    end: 2362,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31815,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2353,
                    end: 2363,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2372,
                                    end: 2373,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2376,
                                    end: 2377,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31817,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2368,
                    end: 2378,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2387,
                                    end: 2388,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2391,
                                    end: 2392,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31819,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2383,
                    end: 2393,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2402,
                                    end: 2403,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2406,
                                    end: 2407,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31821,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2398,
                    end: 2408,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2417,
                                    end: 2418,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2421,
                                    end: 2422,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31823,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2413,
                    end: 2423,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2432,
                                    end: 2433,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2436,
                                    end: 2437,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31825,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2428,
                    end: 2438,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2447,
                                    end: 2448,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2451,
                                    end: 2452,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31827,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2443,
                    end: 2453,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2462,
                                    end: 2463,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2466,
                                    end: 2467,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31829,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2458,
                    end: 2468,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2477,
                                    end: 2478,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2481,
                                    end: 2482,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31831,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2473,
                    end: 2483,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2492,
                                    end: 2493,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2496,
                                    end: 2497,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31833,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2488,
                    end: 2498,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2507,
                                    end: 2508,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2511,
                                    end: 2512,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31835,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2503,
                    end: 2513,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2522,
                                    end: 2523,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2526,
                                    end: 2527,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31837,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2518,
                    end: 2528,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2537,
                                    end: 2538,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2541,
                                    end: 2542,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31839,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2533,
                    end: 2543,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2552,
                                    end: 2553,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2556,
                                    end: 2557,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31841,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2548,
                    end: 2558,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2567,
                                    end: 2568,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2571,
                                    end: 2572,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31843,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2563,
                    end: 2573,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2582,
                                    end: 2583,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2586,
                                    end: 2587,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31845,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2578,
                    end: 2588,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2597,
                                    end: 2598,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2601,
                                    end: 2602,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31847,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2593,
                    end: 2603,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2612,
                                    end: 2613,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2616,
                                    end: 2617,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31849,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2608,
                    end: 2618,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2627,
                                    end: 2628,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2631,
                                    end: 2632,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31851,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2623,
                    end: 2633,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2642,
                                    end: 2643,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2646,
                                    end: 2647,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31853,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2638,
                    end: 2648,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2657,
                                    end: 2658,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2661,
                                    end: 2662,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31855,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2653,
                    end: 2663,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2672,
                                    end: 2673,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2676,
                                    end: 2677,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31857,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2668,
                    end: 2678,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2687,
                                    end: 2688,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2691,
                                    end: 2692,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31859,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2683,
                    end: 2693,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2702,
                                    end: 2703,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2706,
                                    end: 2707,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31861,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2698,
                    end: 2708,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2717,
                                    end: 2718,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2721,
                                    end: 2722,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31863,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2713,
                    end: 2723,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2732,
                                    end: 2733,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2736,
                                    end: 2737,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31865,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2728,
                    end: 2738,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2747,
                                    end: 2748,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2751,
                                    end: 2752,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31867,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2743,
                    end: 2753,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2762,
                                    end: 2763,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2766,
                                    end: 2767,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31869,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2758,
                    end: 2768,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2777,
                                    end: 2778,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2781,
                                    end: 2782,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31871,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2773,
                    end: 2783,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2792,
                                    end: 2793,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2796,
                                    end: 2797,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31873,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2788,
                    end: 2798,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2807,
                                    end: 2808,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2811,
                                    end: 2812,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31875,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2803,
                    end: 2813,
                    as_str(): "let c = 0;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2822,
                                    end: 2823,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        0,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 2826,
                                    end: 2827,
                                    as_str(): "0",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31877,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 2818,
                    end: 2828,
                    as_str(): "let c = 0;",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0acfed220,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
        ),
        start: 973,
        end: 2830,
        as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
    },
    attributes: {},
    return_type: TypeId(
        31632,
    ),
    initial_return_type: TypeId(
        31631,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0acfed220,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
        ),
        start: 973,
        end: 981,
        as_str(): "fn foo()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0acfed220,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
            ),
            start: 106,
            end: 110,
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
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 334,
                                    end: 337,
                                    as_str(): "zzz",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0acfed220,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                            ),
                                            start: 48,
                                            end: 49,
                                            as_str(): "Z",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 352,
                                                    end: 353,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 355,
                                                    end: 356,
                                                    as_str(): "1",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 366,
                                                    end: 367,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        2,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 369,
                                                    end: 370,
                                                    as_str(): "2",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 380,
                                                    end: 381,
                                                    as_str(): "c",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        3,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 383,
                                                    end: 384,
                                                    as_str(): "3",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 394,
                                                    end: 395,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 397,
                                                    end: 398,
                                                    as_str(): "4",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 340,
                                        end: 341,
                                        as_str(): "Z",
                                    },
                                },
                                return_type: TypeId(
                                    31883,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 340,
                                    end: 405,
                                    as_str(): "Z {\n        a: 1,\n        b: 2,\n        c: 3,\n        d: 4,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31883,
                            ),
                            type_ascription: TypeId(
                                31881,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 330,
                    end: 406,
                    as_str(): "let zzz = Z {\n        a: 1,\n        b: 2,\n        c: 3,\n        d: 4,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 453,
                                    end: 455,
                                    as_str(): "z1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        5,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 458,
                                    end: 459,
                                    as_str(): "5",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31892,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 449,
                    end: 460,
                    as_str(): "let z1 = 5;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 469,
                                    end: 471,
                                    as_str(): "z2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        6,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 474,
                                    end: 475,
                                    as_str(): "6",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                31894,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 465,
                    end: 476,
                    as_str(): "let z2 = 6;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 538,
                                        end: 541,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13317,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 538,
                                        end: 541,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31897,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 538,
                            end: 543,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 538,
                    end: 543,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 549,
                                        end: 552,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13319,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 549,
                                        end: 552,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31899,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 549,
                            end: 554,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 549,
                    end: 554,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 560,
                                        end: 563,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13321,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 560,
                                        end: 563,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31901,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 560,
                            end: 565,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 560,
                    end: 565,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 571,
                                        end: 574,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13323,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 571,
                                        end: 574,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31903,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 571,
                            end: 576,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 571,
                    end: 576,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 582,
                                        end: 585,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13325,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 582,
                                        end: 585,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31905,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 582,
                            end: 587,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 582,
                    end: 587,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 593,
                                        end: 596,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13327,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 593,
                                        end: 596,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31907,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 593,
                            end: 598,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 593,
                    end: 598,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 604,
                                        end: 607,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13329,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 604,
                                        end: 607,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31909,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 604,
                            end: 609,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 604,
                    end: 609,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 615,
                                        end: 618,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13331,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 615,
                                        end: 618,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31911,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 615,
                            end: 620,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 615,
                    end: 620,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 626,
                                        end: 629,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13333,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 626,
                                        end: 629,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31913,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 626,
                            end: 631,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 626,
                    end: 631,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 637,
                                        end: 640,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13335,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 637,
                                        end: 640,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31915,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 637,
                            end: 642,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 637,
                    end: 642,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 648,
                                        end: 651,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 648,
                                        end: 651,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31917,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 648,
                            end: 653,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 648,
                    end: 653,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 659,
                                        end: 662,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13339,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 659,
                                        end: 662,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31919,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 659,
                            end: 664,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 659,
                    end: 664,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 670,
                                        end: 673,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13341,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 670,
                                        end: 673,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31921,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 670,
                            end: 675,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 670,
                    end: 675,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 681,
                                        end: 684,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13343,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 681,
                                        end: 684,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31923,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 681,
                            end: 686,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 681,
                    end: 686,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 692,
                                        end: 695,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13345,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 692,
                                        end: 695,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31925,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 692,
                            end: 697,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 692,
                    end: 697,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 703,
                                        end: 706,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13347,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 703,
                                        end: 706,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31927,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 703,
                            end: 708,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 703,
                    end: 708,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 714,
                                        end: 717,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13349,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 714,
                                        end: 717,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31929,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 714,
                            end: 719,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 714,
                    end: 719,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 725,
                                        end: 728,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13351,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 725,
                                        end: 728,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31931,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 725,
                            end: 730,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 725,
                    end: 730,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 736,
                                        end: 739,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13353,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 736,
                                        end: 739,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31933,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 736,
                            end: 741,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 736,
                    end: 741,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 747,
                                        end: 750,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13355,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 747,
                                        end: 750,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31935,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 747,
                            end: 752,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 747,
                    end: 752,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 758,
                                        end: 761,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13357,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 758,
                                        end: 761,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31937,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 758,
                            end: 763,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 758,
                    end: 763,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 769,
                                        end: 772,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13359,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 769,
                                        end: 772,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31939,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 769,
                            end: 774,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 769,
                    end: 774,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 780,
                                        end: 783,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13361,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 780,
                                        end: 783,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31941,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 780,
                            end: 785,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 780,
                    end: 785,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 791,
                                        end: 794,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13363,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 791,
                                        end: 794,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31943,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 791,
                            end: 796,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 791,
                    end: 796,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 802,
                                        end: 805,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13365,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 802,
                                        end: 805,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31945,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 802,
                            end: 807,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 802,
                    end: 807,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 813,
                                        end: 816,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13367,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 813,
                                        end: 816,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31947,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 813,
                            end: 818,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 813,
                    end: 818,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 824,
                                        end: 827,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13369,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 824,
                                        end: 827,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31949,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 824,
                            end: 829,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 824,
                    end: 829,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 835,
                                        end: 838,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13371,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 835,
                                        end: 838,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31951,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 835,
                            end: 840,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 835,
                    end: 840,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 846,
                                        end: 849,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13373,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 846,
                                        end: 849,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31953,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 846,
                            end: 851,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 846,
                    end: 851,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 857,
                                        end: 860,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13375,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 857,
                                        end: 860,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31955,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 857,
                            end: 862,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 857,
                    end: 862,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 868,
                                        end: 871,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13377,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 868,
                                        end: 871,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31957,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 868,
                            end: 873,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 868,
                    end: 873,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 879,
                                        end: 882,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13379,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 879,
                                        end: 882,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31959,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 879,
                            end: 884,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 879,
                    end: 884,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 890,
                                        end: 893,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13381,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 890,
                                        end: 893,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31961,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 890,
                            end: 895,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 890,
                    end: 895,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 901,
                                        end: 904,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                13383,
                                Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 2830,
                                    as_str(): "fn foo() {\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n    let c = 0;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0acfed220,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                        ),
                                        start: 901,
                                        end: 904,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31963,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 901,
                            end: 906,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 901,
                    end: 906,
                    as_str(): "foo()",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 955,
                                                    end: 956,
                                                    as_str(): "+",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 955,
                                                    end: 956,
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
                                                src (ptr): 0x00007fe0acfed220,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                ),
                                                start: 955,
                                                end: 956,
                                                as_str(): "+",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c1500980,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 124,
                                                    end: 128,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "core",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 950,
                                                                    end: 951,
                                                                    as_str(): "+",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 950,
                                                                    end: 951,
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
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 950,
                                                                end: 951,
                                                                as_str(): "+",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: true,
                                                    },
                                                    contract_call_params: {},
                                                    arguments: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c1500980,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 124,
                                                                    end: 128,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 942,
                                                                                    end: 943,
                                                                                    as_str(): "+",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 942,
                                                                                    end: 943,
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
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 942,
                                                                                end: 943,
                                                                                as_str(): "+",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        is_absolute: true,
                                                                    },
                                                                    contract_call_params: {},
                                                                    arguments: [
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c1500980,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 124,
                                                                                    end: 128,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: FunctionApplication {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "core",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 934,
                                                                                                    end: 935,
                                                                                                    as_str(): "+",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 934,
                                                                                                    end: 935,
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
                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                ),
                                                                                                start: 934,
                                                                                                end: 935,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                    contract_call_params: {},
                                                                                    arguments: [
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c1500980,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                    ),
                                                                                                    start: 124,
                                                                                                    end: 128,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: FunctionApplication {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "core",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 926,
                                                                                                                    end: 927,
                                                                                                                    as_str(): "+",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 926,
                                                                                                                    end: 927,
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
                                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                ),
                                                                                                                start: 926,
                                                                                                                end: 927,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        is_absolute: true,
                                                                                                    },
                                                                                                    contract_call_params: {},
                                                                                                    arguments: [
                                                                                                        (
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c1500980,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 124,
                                                                                                                    end: 128,
                                                                                                                    as_str(): "self",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            TyExpression {
                                                                                                                expression: StructFieldAccess {
                                                                                                                    prefix: TyExpression {
                                                                                                                        expression: VariableExpression {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 334,
                                                                                                                                    end: 337,
                                                                                                                                    as_str(): "zzz",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 920,
                                                                                                                                end: 923,
                                                                                                                                as_str(): "zzz",
                                                                                                                            },
                                                                                                                            mutability: Immutable,
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            31883,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 920,
                                                                                                                            end: 923,
                                                                                                                            as_str(): "zzz",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    field_to_access: TyStructField {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 56,
                                                                                                                                end: 57,
                                                                                                                                as_str(): "a",
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
                                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 56,
                                                                                                                            end: 62,
                                                                                                                            as_str(): "a: u64",
                                                                                                                        },
                                                                                                                        type_span: Span {
                                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 59,
                                                                                                                            end: 62,
                                                                                                                            as_str(): "u64",
                                                                                                                        },
                                                                                                                        attributes: {},
                                                                                                                    },
                                                                                                                    field_instantiation_span: Span {
                                                                                                                        src (ptr): 0x00007fe0acfed220,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 924,
                                                                                                                        end: 925,
                                                                                                                        as_str(): "a",
                                                                                                                    },
                                                                                                                    resolved_type_of_parent: TypeId(
                                                                                                                        31883,
                                                                                                                    ),
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 920,
                                                                                                                    end: 925,
                                                                                                                    as_str(): "zzz.a",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        (
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c1500980,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 130,
                                                                                                                    end: 135,
                                                                                                                    as_str(): "other",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            TyExpression {
                                                                                                                expression: StructFieldAccess {
                                                                                                                    prefix: TyExpression {
                                                                                                                        expression: VariableExpression {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 334,
                                                                                                                                    end: 337,
                                                                                                                                    as_str(): "zzz",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 928,
                                                                                                                                end: 931,
                                                                                                                                as_str(): "zzz",
                                                                                                                            },
                                                                                                                            mutability: Immutable,
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            31883,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 928,
                                                                                                                            end: 931,
                                                                                                                            as_str(): "zzz",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    field_to_access: TyStructField {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 68,
                                                                                                                                end: 69,
                                                                                                                                as_str(): "b",
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
                                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 68,
                                                                                                                            end: 74,
                                                                                                                            as_str(): "b: u64",
                                                                                                                        },
                                                                                                                        type_span: Span {
                                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 71,
                                                                                                                            end: 74,
                                                                                                                            as_str(): "u64",
                                                                                                                        },
                                                                                                                        attributes: {},
                                                                                                                    },
                                                                                                                    field_instantiation_span: Span {
                                                                                                                        src (ptr): 0x00007fe0acfed220,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 932,
                                                                                                                        end: 933,
                                                                                                                        as_str(): "b",
                                                                                                                    },
                                                                                                                    resolved_type_of_parent: TypeId(
                                                                                                                        31883,
                                                                                                                    ),
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 928,
                                                                                                                    end: 933,
                                                                                                                    as_str(): "zzz.b",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                    function_decl_id: DeclId(
                                                                                                        13384,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe0c1500980,
                                                                                                            path: Some(
                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                            ),
                                                                                                            start: 117,
                                                                                                            end: 185,
                                                                                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                                        },
                                                                                                    ),
                                                                                                    self_state_idx: None,
                                                                                                    selector: None,
                                                                                                    type_binding: Some(
                                                                                                        TypeBinding {
                                                                                                            inner: (),
                                                                                                            type_arguments: [],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                ),
                                                                                                                start: 926,
                                                                                                                end: 927,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 920,
                                                                                                    end: 933,
                                                                                                    as_str(): "zzz.a + zzz.b",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c1500980,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                    ),
                                                                                                    start: 130,
                                                                                                    end: 135,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: StructFieldAccess {
                                                                                                    prefix: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 334,
                                                                                                                    end: 337,
                                                                                                                    as_str(): "zzz",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                ),
                                                                                                                start: 936,
                                                                                                                end: 939,
                                                                                                                as_str(): "zzz",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            31883,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                            ),
                                                                                                            start: 936,
                                                                                                            end: 939,
                                                                                                            as_str(): "zzz",
                                                                                                        },
                                                                                                    },
                                                                                                    field_to_access: TyStructField {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                                ),
                                                                                                                start: 80,
                                                                                                                end: 81,
                                                                                                                as_str(): "c",
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
                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                            ),
                                                                                                            start: 80,
                                                                                                            end: 86,
                                                                                                            as_str(): "c: u64",
                                                                                                        },
                                                                                                        type_span: Span {
                                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                            ),
                                                                                                            start: 83,
                                                                                                            end: 86,
                                                                                                            as_str(): "u64",
                                                                                                        },
                                                                                                        attributes: {},
                                                                                                    },
                                                                                                    field_instantiation_span: Span {
                                                                                                        src (ptr): 0x00007fe0acfed220,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                        ),
                                                                                                        start: 940,
                                                                                                        end: 941,
                                                                                                        as_str(): "c",
                                                                                                    },
                                                                                                    resolved_type_of_parent: TypeId(
                                                                                                        31883,
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 936,
                                                                                                    end: 941,
                                                                                                    as_str(): "zzz.c",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        13385,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe0c1500980,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 117,
                                                                                            end: 185,
                                                                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                                        },
                                                                                    ),
                                                                                    self_state_idx: None,
                                                                                    selector: None,
                                                                                    type_binding: Some(
                                                                                        TypeBinding {
                                                                                            inner: (),
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                ),
                                                                                                start: 934,
                                                                                                end: 935,
                                                                                                as_str(): "+",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 920,
                                                                                    end: 941,
                                                                                    as_str(): "zzz.a + zzz.b + zzz.c",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c1500980,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 130,
                                                                                    end: 135,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: StructFieldAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                    ),
                                                                                                    start: 334,
                                                                                                    end: 337,
                                                                                                    as_str(): "zzz",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                ),
                                                                                                start: 944,
                                                                                                end: 947,
                                                                                                as_str(): "zzz",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            31883,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                            ),
                                                                                            start: 944,
                                                                                            end: 947,
                                                                                            as_str(): "zzz",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: TyStructField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                                ),
                                                                                                start: 92,
                                                                                                end: 93,
                                                                                                as_str(): "d",
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
                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                            ),
                                                                                            start: 92,
                                                                                            end: 98,
                                                                                            as_str(): "d: u64",
                                                                                        },
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fe0acfed220,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                            ),
                                                                                            start: 95,
                                                                                            end: 98,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    field_instantiation_span: Span {
                                                                                        src (ptr): 0x00007fe0acfed220,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                        ),
                                                                                        start: 948,
                                                                                        end: 949,
                                                                                        as_str(): "d",
                                                                                    },
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        31883,
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0acfed220,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                    ),
                                                                                    start: 944,
                                                                                    end: 949,
                                                                                    as_str(): "zzz.d",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13386,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0c1500980,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 117,
                                                                            end: 185,
                                                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: Some(
                                                                        TypeBinding {
                                                                            inner: (),
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0acfed220,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                                ),
                                                                                start: 942,
                                                                                end: 943,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 920,
                                                                    end: 949,
                                                                    as_str(): "zzz.a + zzz.b + zzz.c + zzz.d",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c1500980,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                    ),
                                                                    start: 130,
                                                                    end: 135,
                                                                    as_str(): "other",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0acfed220,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                            ),
                                                                            start: 453,
                                                                            end: 455,
                                                                            as_str(): "z1",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0acfed220,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                        ),
                                                                        start: 952,
                                                                        end: 954,
                                                                        as_str(): "z1",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0acfed220,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                    ),
                                                                    start: 952,
                                                                    end: 954,
                                                                    as_str(): "z1",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13387,
                                                        Span {
                                                            src (ptr): 0x00007fe0c1500980,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 117,
                                                            end: 185,
                                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0acfed220,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                                ),
                                                                start: 950,
                                                                end: 951,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 920,
                                                    end: 954,
                                                    as_str(): "zzz.a + zzz.b + zzz.c + zzz.d + z1",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c1500980,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 130,
                                                    end: 135,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0acfed220,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                            ),
                                                            start: 469,
                                                            end: 471,
                                                            as_str(): "z2",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0acfed220,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                        ),
                                                        start: 957,
                                                        end: 959,
                                                        as_str(): "z2",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0acfed220,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                    ),
                                                    start: 957,
                                                    end: 959,
                                                    as_str(): "z2",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13388,
                                        Span {
                                            src (ptr): 0x00007fe0c1500980,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 117,
                                            end: 185,
                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0acfed220,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                                ),
                                                start: 955,
                                                end: 956,
                                                as_str(): "+",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0acfed220,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                                    ),
                                    start: 920,
                                    end: 959,
                                    as_str(): "zzz.a + zzz.b + zzz.c + zzz.d + z1 + z2",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31979,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0acfed220,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                            ),
                            start: 913,
                            end: 959,
                            as_str(): "return zzz.a + zzz.b + zzz.c + zzz.d + z1 + z2",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0acfed220,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
                    ),
                    start: 913,
                    end: 959,
                    as_str(): "return zzz.a + zzz.b + zzz.c + zzz.d + z1 + z2",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0acfed220,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
        ),
        start: 103,
        end: 971,
        as_str(): "fn main() -> u64 {\n    // Chosen names force these variables to show up last in the list of locals in IR so they will\n    // be allocated last and require the highest offset to be computed\n\n    // Test get_ptr large offset\n    let zzz = Z {\n        a: 1,\n        b: 2,\n        c: 3,\n        d: 4,\n    };\n\n    // Test LW/SW with large offsets\n    let z1 = 5;\n    let z2 = 6;\n\n    // Add enough stack variables to reach > 4096 words\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n    foo();\n\n    return zzz.a + zzz.b + zzz.c + zzz.d + z1 + z2 // get 16\n}",
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
        src (ptr): 0x00007fe0acfed220,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHujjPr/many_stack_variables/src/main.sw",
        ),
        start: 116,
        end: 119,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

