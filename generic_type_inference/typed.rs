


TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 69,
            end: 79,
            as_str(): "CustomType",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 86,
                    end: 90,
                    as_str(): "name",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                33091,
            ),
            initial_type_id: TypeId(
                33091,
            ),
            span: Span {
                src (ptr): 0x00007fe0eb7482d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                ),
                start: 86,
                end: 98,
                as_str(): "name: str[3]",
            },
            type_span: Span {
                src (ptr): 0x00007fe0eb7482d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                ),
                start: 92,
                end: 98,
                as_str(): "str[3]",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 62,
        end: 101,
        as_str(): "struct CustomType {\n    name: str[3],\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 108,
            end: 116,
            as_str(): "MyResult",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(33092),
        E: TypeId(33093),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 129,
                    end: 131,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                33092,
            ),
            initial_type_id: TypeId(
                33094,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0eb7482d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                ),
                start: 133,
                end: 134,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0eb7482d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                ),
                start: 129,
                end: 134,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 140,
                    end: 143,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                33093,
            ),
            initial_type_id: TypeId(
                33095,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0eb7482d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                ),
                start: 145,
                end: 146,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0eb7482d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                ),
                start: 140,
                end: 146,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 103,
        end: 149,
        as_str(): "enum MyResult<T, E> {\n    Ok: T,\n    Err: E,\n}",
    },
    visibility: Private,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 158,
            end: 168,
            as_str(): "SomeStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 178,
                    end: 179,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                33096,
            ),
            initial_type_id: TypeId(
                33097,
            ),
            span: Span {
                src (ptr): 0x00007fe0eb7482d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                ),
                start: 178,
                end: 182,
                as_str(): "a: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0eb7482d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                ),
                start: 181,
                end: 182,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(33096),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 151,
        end: 185,
        as_str(): "struct SomeStruct<T> {\n    a: T,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 190,
            end: 205,
            as_str(): "simple_vec_test",
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
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 222,
                                    end: 226,
                                    as_str(): "vec1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 229,
                                                    end: 232,
                                                    as_str(): "Vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 234,
                                                end: 237,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        14632,
                                        Span {
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 1761,
                                            end: 1866,
                                            as_str(): "pub fn new() -> Self {\n        Self {\n            buf: RawVec::new(),\n            len: 0,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 229,
                                                end: 237,
                                                as_str(): "Vec::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33333,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 229,
                                    end: 239,
                                    as_str(): "Vec::new()",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                33333,
                            ),
                            type_ascription: TypeId(
                                33101,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 214,
                    end: 240,
                    as_str(): "let mut vec1 = Vec::new();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 253,
                                    end: 257,
                                    as_str(): "vec2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 260,
                                                    end: 263,
                                                    as_str(): "Vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 265,
                                                end: 268,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        15878,
                                        Span {
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 1761,
                                            end: 1866,
                                            as_str(): "pub fn new() -> Self {\n        Self {\n            buf: RawVec::new(),\n            len: 0,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 260,
                                                end: 268,
                                                as_str(): "Vec::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    34893,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 260,
                                    end: 270,
                                    as_str(): "Vec::new()",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                34893,
                            ),
                            type_ascription: TypeId(
                                34072,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 245,
                    end: 271,
                    as_str(): "let mut vec2 = Vec::new();",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 282,
                                        end: 286,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 253,
                                                    end: 257,
                                                    as_str(): "vec2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 277,
                                                end: 281,
                                                as_str(): "vec2",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            34893,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 277,
                                            end: 281,
                                            as_str(): "vec2",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Literal(
                                            U32(
                                                54,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            33,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 287,
                                            end: 292,
                                            as_str(): "54u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16875,
                                Span {
                                    src (ptr): 0x00007fe0fb5577b0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 282,
                                        end: 286,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36334,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 277,
                            end: 293,
                            as_str(): "vec2.push(54u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 277,
                    end: 293,
                    as_str(): "vec2.push(54u32)",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 304,
                                        end: 308,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 222,
                                                    end: 226,
                                                    as_str(): "vec1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 299,
                                                end: 303,
                                                as_str(): "vec1",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            33333,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 299,
                                            end: 303,
                                            as_str(): "vec1",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: StructExpression {
                                            struct_name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 158,
                                                    end: 168,
                                                    as_str(): "SomeStruct",
                                                },
                                                is_raw_ident: false,
                                            },
                                            fields: [
                                                TyStructExpressionField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 322,
                                                            end: 323,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    value: TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 325,
                                                            end: 327,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                },
                                            ],
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 309,
                                                end: 319,
                                                as_str(): "SomeStruct",
                                            },
                                        },
                                        return_type: TypeId(
                                            36340,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 309,
                                            end: 329,
                                            as_str(): "SomeStruct { a: 42 }",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16876,
                                Span {
                                    src (ptr): 0x00007fe0fb5577b0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 304,
                                        end: 308,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36343,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 299,
                            end: 330,
                            as_str(): "vec1.push(SomeStruct { a: 42 })",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 299,
                    end: 330,
                    as_str(): "vec1.push(SomeStruct { a: 42 })",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 337,
                                        end: 343,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe0fb8b5410,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 367,
                                                            end: 369,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 367,
                                                            end: 369,
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
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 367,
                                                        end: 369,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe0fb609480,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 356,
                                                                                end: 362,
                                                                                as_str(): "unwrap",
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
                                                                                    src (ptr): 0x00007fe0fab55c60,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                    ),
                                                                                    start: 4635,
                                                                                    end: 4639,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: FunctionApplication {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 349,
                                                                                                end: 352,
                                                                                                as_str(): "get",
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
                                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                    ),
                                                                                                    start: 4586,
                                                                                                    end: 4590,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 222,
                                                                                                            end: 226,
                                                                                                            as_str(): "vec1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 344,
                                                                                                        end: 348,
                                                                                                        as_str(): "vec1",
                                                                                                    },
                                                                                                    mutability: Mutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    33333,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 344,
                                                                                                    end: 348,
                                                                                                    as_str(): "vec1",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                    ),
                                                                                                    start: 4592,
                                                                                                    end: 4597,
                                                                                                    as_str(): "index",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: Literal(
                                                                                                    U64(
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 353,
                                                                                                    end: 354,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        16878,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                            ),
                                                                                            start: 4575,
                                                                                            end: 4935,
                                                                                            as_str(): "pub fn get(self, index: u64) -> Option<T> {\n        // First check that index is within bounds.\n        if self.len <= index {\n            return Option::None::<T>;\n        };\n\n        // Get a pointer to the desired element using `index`\n        let ptr = self.buf.ptr().add::<T>(index);\n\n        // Read from `ptr`\n        Option::Some(ptr.read::<T>())\n    }",
                                                                                        },
                                                                                    ),
                                                                                    self_state_idx: None,
                                                                                    selector: None,
                                                                                    type_binding: Some(
                                                                                        TypeBinding {
                                                                                            inner: (),
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 349,
                                                                                                end: 352,
                                                                                                as_str(): "get",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    33187,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 344,
                                                                                    end: 355,
                                                                                    as_str(): "vec1.get(0)",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        16900,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0fab55c60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 4621,
                                                                            end: 4766,
                                                                            as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: Some(
                                                                        TypeBinding {
                                                                            inner: (),
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 356,
                                                                                end: 362,
                                                                                as_str(): "unwrap",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    33103,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 344,
                                                                    end: 364,
                                                                    as_str(): "vec1.get(0).unwrap()",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 178,
                                                                        end: 179,
                                                                        as_str(): "a",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    36339,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    33097,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 178,
                                                                    end: 182,
                                                                    as_str(): "a: T",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 181,
                                                                    end: 182,
                                                                    as_str(): "T",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 365,
                                                                end: 366,
                                                                as_str(): "a",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                33103,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            36339,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 344,
                                                            end: 366,
                                                            as_str(): "vec1.get(0).unwrap().a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb609480,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 370,
                                                            end: 372,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                16904,
                                                Span {
                                                    src (ptr): 0x00007fe0fb609480,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 367,
                                                        end: 369,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 344,
                                            end: 372,
                                            as_str(): "vec1.get(0).unwrap().a == 42",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16905,
                                Span {
                                    src (ptr): 0x00007fe0fb8b5410,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 337,
                                        end: 343,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36396,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 337,
                            end: 373,
                            as_str(): "assert(vec1.get(0).unwrap().a == 42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 337,
                    end: 373,
                    as_str(): "assert(vec1.get(0).unwrap().a == 42)",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 379,
                                        end: 385,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe0fb8b5410,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 407,
                                                            end: 409,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 407,
                                                            end: 409,
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
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 407,
                                                        end: 409,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe0fb609480,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3114,
                                                            end: 3118,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 398,
                                                                        end: 404,
                                                                        as_str(): "unwrap",
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
                                                                            src (ptr): 0x00007fe0fab55c60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 4635,
                                                                            end: 4639,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 391,
                                                                                        end: 394,
                                                                                        as_str(): "get",
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
                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                            ),
                                                                                            start: 4586,
                                                                                            end: 4590,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 253,
                                                                                                    end: 257,
                                                                                                    as_str(): "vec2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 386,
                                                                                                end: 390,
                                                                                                as_str(): "vec2",
                                                                                            },
                                                                                            mutability: Mutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            34893,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 386,
                                                                                            end: 390,
                                                                                            as_str(): "vec2",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                            ),
                                                                                            start: 4592,
                                                                                            end: 4597,
                                                                                            as_str(): "index",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: Literal(
                                                                                            U64(
                                                                                                0,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 395,
                                                                                            end: 396,
                                                                                            as_str(): "0",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                16907,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                    ),
                                                                                    start: 4575,
                                                                                    end: 4935,
                                                                                    as_str(): "pub fn get(self, index: u64) -> Option<T> {\n        // First check that index is within bounds.\n        if self.len <= index {\n            return Option::None::<T>;\n        };\n\n        // Get a pointer to the desired element using `index`\n        let ptr = self.buf.ptr().add::<T>(index);\n\n        // Read from `ptr`\n        Option::Some(ptr.read::<T>())\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: Some(
                                                                                TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 391,
                                                                                        end: 394,
                                                                                        as_str(): "get",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            34867,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 386,
                                                                            end: 397,
                                                                            as_str(): "vec2.get(0)",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                16932,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fab55c60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 4621,
                                                                    end: 4766,
                                                                    as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 398,
                                                                        end: 404,
                                                                        as_str(): "unwrap",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            34074,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 386,
                                                            end: 406,
                                                            as_str(): "vec2.get(0).unwrap()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb609480,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3120,
                                                            end: 3125,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U32(
                                                                54,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 410,
                                                            end: 415,
                                                            as_str(): "54u32",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                16936,
                                                Span {
                                                    src (ptr): 0x00007fe0fb609480,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3108,
                                                    end: 3174,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 407,
                                                        end: 409,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 386,
                                            end: 415,
                                            as_str(): "vec2.get(0).unwrap() == 54u32",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16937,
                                Span {
                                    src (ptr): 0x00007fe0fb8b5410,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 379,
                                        end: 385,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36448,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 379,
                            end: 416,
                            as_str(): "assert(vec2.get(0).unwrap() == 54u32)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 379,
                    end: 416,
                    as_str(): "assert(vec2.get(0).unwrap() == 54u32)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 187,
        end: 419,
        as_str(): "fn simple_vec_test() {\n    let mut vec1 = Vec::new();\n    let mut vec2 = Vec::new();\n\n    vec2.push(54u32);\n    vec1.push(SomeStruct { a: 42 });\n\n    assert(vec1.get(0).unwrap().a == 42);\n    assert(vec2.get(0).unwrap() == 54u32);\n}",
    },
    attributes: {},
    return_type: TypeId(
        33100,
    ),
    initial_return_type: TypeId(
        33099,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 187,
        end: 207,
        as_str(): "fn simple_vec_test()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 424,
            end: 440,
            as_str(): "complex_vec_test",
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
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 457,
                                    end: 494,
                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 497,
                                                    end: 500,
                                                    as_str(): "Vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 502,
                                                end: 505,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        17188,
                                        Span {
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 1761,
                                            end: 1866,
                                            as_str(): "pub fn new() -> Self {\n        Self {\n            buf: RawVec::new(),\n            len: 0,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 497,
                                                end: 505,
                                                as_str(): "Vec::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    36685,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 497,
                                    end: 507,
                                    as_str(): "Vec::new()",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                36685,
                            ),
                            type_ascription: TypeId(
                                36453,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 449,
                    end: 508,
                    as_str(): "let mut exp_vec_in_a_vec_in_a_struct_in_a_vec = Vec::new();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 521,
                                    end: 532,
                                    as_str(): "inner_vec_1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 535,
                                                    end: 538,
                                                    as_str(): "Vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 540,
                                                end: 543,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        18434,
                                        Span {
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 1761,
                                            end: 1866,
                                            as_str(): "pub fn new() -> Self {\n        Self {\n            buf: RawVec::new(),\n            len: 0,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 535,
                                                end: 543,
                                                as_str(): "Vec::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    38130,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 535,
                                    end: 545,
                                    as_str(): "Vec::new()",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                38130,
                            ),
                            type_ascription: TypeId(
                                37424,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 513,
                    end: 546,
                    as_str(): "let mut inner_vec_1 = Vec::new();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 555,
                                    end: 572,
                                    as_str(): "inner_inner_vec_1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 575,
                                                end: 583,
                                                as_str(): "vec_from",
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
                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                    ),
                                                    start: 32,
                                                    end: 36,
                                                    as_str(): "vals",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Array {
                                                    contents: [
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    0,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 585,
                                                                end: 586,
                                                                as_str(): "0",
                                                            },
                                                        },
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    1,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 588,
                                                                end: 589,
                                                                as_str(): "1",
                                                            },
                                                        },
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    2,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 591,
                                                                end: 592,
                                                                as_str(): "2",
                                                            },
                                                        },
                                                    ],
                                                },
                                                return_type: TypeId(
                                                    39692,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 584,
                                                    end: 593,
                                                    as_str(): "[0, 1, 2]",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        19681,
                                        Span {
                                            src (ptr): 0x00007fe0eaa4c3f0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                            ),
                                            start: 16,
                                            end: 170,
                                            as_str(): "pub fn vec_from(vals: [u32; 3]) -> Vec<u32> {\n    let mut vec = Vec::new();\n    vec.push(vals[0]);\n    vec.push(vals[1]);\n    vec.push(vals[2]);\n    vec\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 575,
                                                end: 583,
                                                as_str(): "vec_from",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31643,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 575,
                                    end: 594,
                                    as_str(): "vec_from([0, 1, 2])",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31643,
                            ),
                            type_ascription: TypeId(
                                39683,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 551,
                    end: 595,
                    as_str(): "let inner_inner_vec_1 = vec_from([0, 1, 2]);",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 613,
                                        end: 617,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 521,
                                                    end: 532,
                                                    as_str(): "inner_vec_1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 601,
                                                end: 612,
                                                as_str(): "inner_vec_1",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            38130,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 601,
                                            end: 612,
                                            as_str(): "inner_vec_1",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 555,
                                                    end: 572,
                                                    as_str(): "inner_inner_vec_1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 618,
                                                end: 635,
                                                as_str(): "inner_inner_vec_1",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31643,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 618,
                                            end: 635,
                                            as_str(): "inner_inner_vec_1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                19682,
                                Span {
                                    src (ptr): 0x00007fe0fb5577b0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 613,
                                        end: 617,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            40018,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 601,
                            end: 636,
                            as_str(): "inner_vec_1.push(inner_inner_vec_1)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 601,
                    end: 636,
                    as_str(): "inner_vec_1.push(inner_inner_vec_1)",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 680,
                                        end: 684,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 457,
                                                    end: 494,
                                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 642,
                                                end: 679,
                                                as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            36685,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 642,
                                            end: 679,
                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb5577b0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: StructExpression {
                                            struct_name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 158,
                                                    end: 168,
                                                    as_str(): "SomeStruct",
                                                },
                                                is_raw_ident: false,
                                            },
                                            fields: [
                                                TyStructExpressionField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 698,
                                                            end: 699,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    value: TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 521,
                                                                    end: 532,
                                                                    as_str(): "inner_vec_1",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                ),
                                                                start: 701,
                                                                end: 712,
                                                                as_str(): "inner_vec_1",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            38130,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 701,
                                                            end: 712,
                                                            as_str(): "inner_vec_1",
                                                        },
                                                    },
                                                },
                                            ],
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 685,
                                                end: 695,
                                                as_str(): "SomeStruct",
                                            },
                                        },
                                        return_type: TypeId(
                                            40024,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 685,
                                            end: 714,
                                            as_str(): "SomeStruct { a: inner_vec_1 }",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                19683,
                                Span {
                                    src (ptr): 0x00007fe0fb5577b0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 680,
                                        end: 684,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            40026,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 642,
                            end: 715,
                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 })",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 642,
                    end: 715,
                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 })",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 722,
                                        end: 728,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe0fb8b5410,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 773,
                                                            end: 775,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 773,
                                                            end: 775,
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
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 773,
                                                        end: 775,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe0fb609480,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3114,
                                                            end: 3118,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 764,
                                                                        end: 770,
                                                                        as_str(): "unwrap",
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
                                                                            src (ptr): 0x00007fe0fab55c60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 4635,
                                                                            end: 4639,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 757,
                                                                                        end: 760,
                                                                                        as_str(): "get",
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
                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                            ),
                                                                                            start: 4586,
                                                                                            end: 4590,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: FunctionApplication {
                                                                                            call_path: CallPath {
                                                                                                prefixes: [],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 748,
                                                                                                        end: 754,
                                                                                                        as_str(): "unwrap",
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
                                                                                                            src (ptr): 0x00007fe0fab55c60,
                                                                                                            path: Some(
                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                                            ),
                                                                                                            start: 4635,
                                                                                                            end: 4639,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    TyExpression {
                                                                                                        expression: FunctionApplication {
                                                                                                            call_path: CallPath {
                                                                                                                prefixes: [],
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 741,
                                                                                                                        end: 744,
                                                                                                                        as_str(): "get",
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
                                                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                            ),
                                                                                                                            start: 4586,
                                                                                                                            end: 4590,
                                                                                                                            as_str(): "self",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: VariableExpression {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 521,
                                                                                                                                    end: 532,
                                                                                                                                    as_str(): "inner_vec_1",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 729,
                                                                                                                                end: 740,
                                                                                                                                as_str(): "inner_vec_1",
                                                                                                                            },
                                                                                                                            mutability: Mutable,
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            38130,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 729,
                                                                                                                            end: 740,
                                                                                                                            as_str(): "inner_vec_1",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                (
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                            ),
                                                                                                                            start: 4592,
                                                                                                                            end: 4597,
                                                                                                                            as_str(): "index",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: Literal(
                                                                                                                            U64(
                                                                                                                                0,
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        return_type: TypeId(
                                                                                                                            21,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 745,
                                                                                                                            end: 746,
                                                                                                                            as_str(): "0",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                            function_decl_id: DeclId(
                                                                                                                19685,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                    ),
                                                                                                                    start: 4575,
                                                                                                                    end: 4935,
                                                                                                                    as_str(): "pub fn get(self, index: u64) -> Option<T> {\n        // First check that index is within bounds.\n        if self.len <= index {\n            return Option::None::<T>;\n        };\n\n        // Get a pointer to the desired element using `index`\n        let ptr = self.buf.ptr().add::<T>(index);\n\n        // Read from `ptr`\n        Option::Some(ptr.read::<T>())\n    }",
                                                                                                                },
                                                                                                            ),
                                                                                                            self_state_idx: None,
                                                                                                            selector: None,
                                                                                                            type_binding: Some(
                                                                                                                TypeBinding {
                                                                                                                    inner: (),
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 741,
                                                                                                                        end: 744,
                                                                                                                        as_str(): "get",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            38301,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 729,
                                                                                                            end: 747,
                                                                                                            as_str(): "inner_vec_1.get(0)",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            function_decl_id: DeclId(
                                                                                                20147,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe0fab55c60,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                                    ),
                                                                                                    start: 4621,
                                                                                                    end: 4766,
                                                                                                    as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                                                },
                                                                                            ),
                                                                                            self_state_idx: None,
                                                                                            selector: None,
                                                                                            type_binding: Some(
                                                                                                TypeBinding {
                                                                                                    inner: (),
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 748,
                                                                                                        end: 754,
                                                                                                        as_str(): "unwrap",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            37426,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 729,
                                                                                            end: 756,
                                                                                            as_str(): "inner_vec_1.get(0).unwrap()",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                            ),
                                                                                            start: 4592,
                                                                                            end: 4597,
                                                                                            as_str(): "index",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: Literal(
                                                                                            U64(
                                                                                                1,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 761,
                                                                                            end: 762,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                20866,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                    ),
                                                                                    start: 4575,
                                                                                    end: 4935,
                                                                                    as_str(): "pub fn get(self, index: u64) -> Option<T> {\n        // First check that index is within bounds.\n        if self.len <= index {\n            return Option::None::<T>;\n        };\n\n        // Get a pointer to the desired element using `index`\n        let ptr = self.buf.ptr().add::<T>(index);\n\n        // Read from `ptr`\n        Option::Some(ptr.read::<T>())\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: Some(
                                                                                TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 757,
                                                                                        end: 760,
                                                                                        as_str(): "get",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            39774,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 729,
                                                                            end: 763,
                                                                            as_str(): "inner_vec_1.get(0).unwrap().get(1)",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                20888,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fab55c60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 4621,
                                                                    end: 4766,
                                                                    as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 764,
                                                                        end: 770,
                                                                        as_str(): "unwrap",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 729,
                                                            end: 772,
                                                            as_str(): "inner_vec_1.get(0).unwrap().get(1).unwrap()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb609480,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3120,
                                                            end: 3125,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                1,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 776,
                                                            end: 777,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                20889,
                                                Span {
                                                    src (ptr): 0x00007fe0fb609480,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3108,
                                                    end: 3174,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 773,
                                                        end: 775,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 729,
                                            end: 777,
                                            as_str(): "inner_vec_1.get(0).unwrap().get(1).unwrap() == 1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                20890,
                                Span {
                                    src (ptr): 0x00007fe0fb8b5410,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 722,
                                        end: 728,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            40754,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 722,
                            end: 778,
                            as_str(): "assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 722,
                    end: 778,
                    as_str(): "assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1)",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 784,
                                        end: 790,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe0fb8b5410,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
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
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 879,
                                                            end: 881,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 879,
                                                            end: 881,
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
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 879,
                                                        end: 881,
                                                        as_str(): "==",
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
                                                            src (ptr): 0x00007fe0fb609480,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3114,
                                                            end: 3118,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 870,
                                                                        end: 876,
                                                                        as_str(): "unwrap",
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
                                                                            src (ptr): 0x00007fe0fab55c60,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 4635,
                                                                            end: 4639,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 863,
                                                                                        end: 866,
                                                                                        as_str(): "get",
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
                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                            ),
                                                                                            start: 4586,
                                                                                            end: 4590,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: FunctionApplication {
                                                                                            call_path: CallPath {
                                                                                                prefixes: [],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 854,
                                                                                                        end: 860,
                                                                                                        as_str(): "unwrap",
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
                                                                                                            src (ptr): 0x00007fe0fab55c60,
                                                                                                            path: Some(
                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                                            ),
                                                                                                            start: 4635,
                                                                                                            end: 4639,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    TyExpression {
                                                                                                        expression: FunctionApplication {
                                                                                                            call_path: CallPath {
                                                                                                                prefixes: [],
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 847,
                                                                                                                        end: 850,
                                                                                                                        as_str(): "get",
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
                                                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                            ),
                                                                                                                            start: 4586,
                                                                                                                            end: 4590,
                                                                                                                            as_str(): "self",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: StructFieldAccess {
                                                                                                                            prefix: TyExpression {
                                                                                                                                expression: FunctionApplication {
                                                                                                                                    call_path: CallPath {
                                                                                                                                        prefixes: [],
                                                                                                                                        suffix: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 836,
                                                                                                                                                end: 842,
                                                                                                                                                as_str(): "unwrap",
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
                                                                                                                                                    src (ptr): 0x00007fe0fab55c60,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 4635,
                                                                                                                                                    end: 4639,
                                                                                                                                                    as_str(): "self",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            TyExpression {
                                                                                                                                                expression: FunctionApplication {
                                                                                                                                                    call_path: CallPath {
                                                                                                                                                        prefixes: [],
                                                                                                                                                        suffix: BaseIdent {
                                                                                                                                                            name_override_opt: None,
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 829,
                                                                                                                                                                end: 832,
                                                                                                                                                                as_str(): "get",
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
                                                                                                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 4586,
                                                                                                                                                                    end: 4590,
                                                                                                                                                                    as_str(): "self",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            TyExpression {
                                                                                                                                                                expression: VariableExpression {
                                                                                                                                                                    name: BaseIdent {
                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                        span: Span {
                                                                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                            path: Some(
                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                            ),
                                                                                                                                                                            start: 457,
                                                                                                                                                                            end: 494,
                                                                                                                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                                                                                                                                                        },
                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                    },
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 791,
                                                                                                                                                                        end: 828,
                                                                                                                                                                        as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                                                                                                                                                    },
                                                                                                                                                                    mutability: Mutable,
                                                                                                                                                                },
                                                                                                                                                                return_type: TypeId(
                                                                                                                                                                    36685,
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 791,
                                                                                                                                                                    end: 828,
                                                                                                                                                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        (
                                                                                                                                                            BaseIdent {
                                                                                                                                                                name_override_opt: None,
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 4592,
                                                                                                                                                                    end: 4597,
                                                                                                                                                                    as_str(): "index",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            TyExpression {
                                                                                                                                                                expression: Literal(
                                                                                                                                                                    U64(
                                                                                                                                                                        0,
                                                                                                                                                                    ),
                                                                                                                                                                ),
                                                                                                                                                                return_type: TypeId(
                                                                                                                                                                    21,
                                                                                                                                                                ),
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 833,
                                                                                                                                                                    end: 834,
                                                                                                                                                                    as_str(): "0",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                    ],
                                                                                                                                                    function_decl_id: DeclId(
                                                                                                                                                        20892,
                                                                                                                                                        Span {
                                                                                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 4575,
                                                                                                                                                            end: 4935,
                                                                                                                                                            as_str(): "pub fn get(self, index: u64) -> Option<T> {\n        // First check that index is within bounds.\n        if self.len <= index {\n            return Option::None::<T>;\n        };\n\n        // Get a pointer to the desired element using `index`\n        let ptr = self.buf.ptr().add::<T>(index);\n\n        // Read from `ptr`\n        Option::Some(ptr.read::<T>())\n    }",
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    self_state_idx: None,
                                                                                                                                                    selector: None,
                                                                                                                                                    type_binding: Some(
                                                                                                                                                        TypeBinding {
                                                                                                                                                            inner: (),
                                                                                                                                                            type_arguments: [],
                                                                                                                                                            span: Span {
                                                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                                path: Some(
                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                                ),
                                                                                                                                                                start: 829,
                                                                                                                                                                end: 832,
                                                                                                                                                                as_str(): "get",
                                                                                                                                                            },
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                },
                                                                                                                                                return_type: TypeId(
                                                                                                                                                    36539,
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 791,
                                                                                                                                                    end: 835,
                                                                                                                                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0)",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                    ],
                                                                                                                                    function_decl_id: DeclId(
                                                                                                                                        21719,
                                                                                                                                        Span {
                                                                                                                                            src (ptr): 0x00007fe0fab55c60,
                                                                                                                                            path: Some(
                                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                                                                            ),
                                                                                                                                            start: 4621,
                                                                                                                                            end: 4766,
                                                                                                                                            as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    self_state_idx: None,
                                                                                                                                    selector: None,
                                                                                                                                    type_binding: Some(
                                                                                                                                        TypeBinding {
                                                                                                                                            inner: (),
                                                                                                                                            type_arguments: [],
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 836,
                                                                                                                                                end: 842,
                                                                                                                                                as_str(): "unwrap",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                },
                                                                                                                                return_type: TypeId(
                                                                                                                                    36455,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 791,
                                                                                                                                    end: 844,
                                                                                                                                    as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap()",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            field_to_access: TyStructField {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 178,
                                                                                                                                        end: 179,
                                                                                                                                        as_str(): "a",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                type_id: TypeId(
                                                                                                                                    40023,
                                                                                                                                ),
                                                                                                                                initial_type_id: TypeId(
                                                                                                                                    33097,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 178,
                                                                                                                                    end: 182,
                                                                                                                                    as_str(): "a: T",
                                                                                                                                },
                                                                                                                                type_span: Span {
                                                                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 181,
                                                                                                                                    end: 182,
                                                                                                                                    as_str(): "T",
                                                                                                                                },
                                                                                                                                attributes: {},
                                                                                                                            },
                                                                                                                            field_instantiation_span: Span {
                                                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 845,
                                                                                                                                end: 846,
                                                                                                                                as_str(): "a",
                                                                                                                            },
                                                                                                                            resolved_type_of_parent: TypeId(
                                                                                                                                36455,
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            40023,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 791,
                                                                                                                            end: 846,
                                                                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                (
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                            ),
                                                                                                                            start: 4592,
                                                                                                                            end: 4597,
                                                                                                                            as_str(): "index",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: Literal(
                                                                                                                            U64(
                                                                                                                                0,
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        return_type: TypeId(
                                                                                                                            21,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 851,
                                                                                                                            end: 852,
                                                                                                                            as_str(): "0",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                            function_decl_id: DeclId(
                                                                                                                23098,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                                                    ),
                                                                                                                    start: 4575,
                                                                                                                    end: 4935,
                                                                                                                    as_str(): "pub fn get(self, index: u64) -> Option<T> {\n        // First check that index is within bounds.\n        if self.len <= index {\n            return Option::None::<T>;\n        };\n\n        // Get a pointer to the desired element using `index`\n        let ptr = self.buf.ptr().add::<T>(index);\n\n        // Read from `ptr`\n        Option::Some(ptr.read::<T>())\n    }",
                                                                                                                },
                                                                                                            ),
                                                                                                            self_state_idx: None,
                                                                                                            selector: None,
                                                                                                            type_binding: Some(
                                                                                                                TypeBinding {
                                                                                                                    inner: (),
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 847,
                                                                                                                        end: 850,
                                                                                                                        as_str(): "get",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            38301,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                            ),
                                                                                                            start: 791,
                                                                                                            end: 853,
                                                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0)",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            function_decl_id: DeclId(
                                                                                                23801,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe0fab55c60,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                                    ),
                                                                                                    start: 4621,
                                                                                                    end: 4766,
                                                                                                    as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                                                },
                                                                                            ),
                                                                                            self_state_idx: None,
                                                                                            selector: None,
                                                                                            type_binding: Some(
                                                                                                TypeBinding {
                                                                                                    inner: (),
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 854,
                                                                                                        end: 860,
                                                                                                        as_str(): "unwrap",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            37426,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 791,
                                                                                            end: 862,
                                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap()",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fb5577b0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                            ),
                                                                                            start: 4592,
                                                                                            end: 4597,
                                                                                            as_str(): "index",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: Literal(
                                                                                            U64(
                                                                                                2,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 867,
                                                                                            end: 868,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                24520,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe0fb5577b0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                                                                    ),
                                                                                    start: 4575,
                                                                                    end: 4935,
                                                                                    as_str(): "pub fn get(self, index: u64) -> Option<T> {\n        // First check that index is within bounds.\n        if self.len <= index {\n            return Option::None::<T>;\n        };\n\n        // Get a pointer to the desired element using `index`\n        let ptr = self.buf.ptr().add::<T>(index);\n\n        // Read from `ptr`\n        Option::Some(ptr.read::<T>())\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: Some(
                                                                                TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 863,
                                                                                        end: 866,
                                                                                        as_str(): "get",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            39774,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 791,
                                                                            end: 869,
                                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2)",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                24563,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fab55c60,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 4621,
                                                                    end: 4766,
                                                                    as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 870,
                                                                        end: 876,
                                                                        as_str(): "unwrap",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            33,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 791,
                                                            end: 878,
                                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb609480,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3120,
                                                            end: 3125,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 882,
                                                            end: 883,
                                                            as_str(): "2",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                24564,
                                                Span {
                                                    src (ptr): 0x00007fe0fb609480,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3108,
                                                    end: 3174,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 879,
                                                        end: 881,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 791,
                                            end: 883,
                                            as_str(): "exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                24565,
                                Span {
                                    src (ptr): 0x00007fe0fb8b5410,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 784,
                                        end: 790,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            42521,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 784,
                            end: 884,
                            as_str(): "assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 784,
                    end: 884,
                    as_str(): "assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 421,
        end: 887,
        as_str(): "fn complex_vec_test() {\n    let mut exp_vec_in_a_vec_in_a_struct_in_a_vec = Vec::new();\n    let mut inner_vec_1 = Vec::new();\n    let inner_inner_vec_1 = vec_from([0, 1, 2]);\n\n    inner_vec_1.push(inner_inner_vec_1);\n    exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 });\n\n    assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1);\n    assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2);\n}",
    },
    attributes: {},
    return_type: TypeId(
        36452,
    ),
    initial_return_type: TypeId(
        36451,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 421,
        end: 442,
        as_str(): "fn complex_vec_test()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 892,
            end: 919,
            as_str(): "simple_option_generics_test",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 928,
                                        end: 934,
                                        as_str(): "assert",
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
                                            src (ptr): 0x00007fe0fb8b5410,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 958,
                                                        end: 965,
                                                        as_str(): "is_none",
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
                                                            src (ptr): 0x00007fe0fab55c60,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                            ),
                                                            start: 3654,
                                                            end: 3658,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 935,
                                                                        end: 948,
                                                                        as_str(): "get_an_option",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                24590,
                                                                Span {
                                                                    src (ptr): 0x00007fe0eaa4c3f0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/utils.sw",
                                                                    ),
                                                                    start: 172,
                                                                    end: 231,
                                                                    as_str(): "pub fn get_an_option<T>() -> Option<T> {\n    Option::None\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [
                                                                        TypeArgument {
                                                                            type_id: TypeId(
                                                                                21,
                                                                            ),
                                                                            initial_type_id: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 951,
                                                                                end: 954,
                                                                                as_str(): "u64",
                                                                            },
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 935,
                                                                        end: 955,
                                                                        as_str(): "get_an_option::<u64>",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            42529,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 935,
                                                            end: 957,
                                                            as_str(): "get_an_option::<u64>()",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                24591,
                                                Span {
                                                    src (ptr): 0x00007fe0fab55c60,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 3639,
                                                    end: 3767,
                                                    as_str(): "pub fn is_none(self) -> bool {\n        match self {\n            Option::Some(_) => false,\n            _ => true,\n        }\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 958,
                                                        end: 965,
                                                        as_str(): "is_none",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 935,
                                            end: 967,
                                            as_str(): "get_an_option::<u64>().is_none()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                24592,
                                Span {
                                    src (ptr): 0x00007fe0fb8b5410,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 928,
                                        end: 934,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            42572,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 928,
                            end: 968,
                            as_str(): "assert(get_an_option::<u64>().is_none())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 928,
                    end: 968,
                    as_str(): "assert(get_an_option::<u64>().is_none())",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 889,
        end: 971,
        as_str(): "fn simple_option_generics_test() {\n    assert(get_an_option::<u64>().is_none());\n}",
    },
    attributes: {},
    return_type: TypeId(
        42525,
    ),
    initial_return_type: TypeId(
        42524,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 889,
        end: 921,
        as_str(): "fn simple_option_generics_test()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 1093,
            end: 1105,
            as_str(): "sell_product",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        false,
                                    ),
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1147,
                                    end: 1152,
                                    as_str(): "false",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Return(
                                                            TyExpression {
                                                                expression: EnumInstantiation {
                                                                    enum_decl: TyEnumDeclaration {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 108,
                                                                                end: 116,
                                                                                as_str(): "MyResult",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_parameters: [
                                                                            T: TypeId(42582),
                                                                            E: TypeId(42583),
                                                                        ],
                                                                        attributes: {},
                                                                        variants: [
                                                                            TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 129,
                                                                                        end: 131,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    42582,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    33094,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 133,
                                                                                    end: 134,
                                                                                    as_str(): "T",
                                                                                },
                                                                                tag: 0,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 129,
                                                                                    end: 134,
                                                                                    as_str(): "Ok: T",
                                                                                },
                                                                                attributes: {},
                                                                            },
                                                                            TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 140,
                                                                                        end: 143,
                                                                                        as_str(): "Err",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    42583,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    33095,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 145,
                                                                                    end: 146,
                                                                                    as_str(): "E",
                                                                                },
                                                                                tag: 1,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 140,
                                                                                    end: 146,
                                                                                    as_str(): "Err: E",
                                                                                },
                                                                                attributes: {},
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 103,
                                                                            end: 149,
                                                                            as_str(): "enum MyResult<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                                                        },
                                                                        visibility: Private,
                                                                    },
                                                                    variant_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 143,
                                                                            as_str(): "Err",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    tag: 1,
                                                                    contents: Some(
                                                                        TyExpression {
                                                                            expression: StructExpression {
                                                                                struct_name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                        ),
                                                                                        start: 69,
                                                                                        end: 79,
                                                                                        as_str(): "CustomType",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                fields: [
                                                                                    TyStructExpressionField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 1209,
                                                                                                end: 1213,
                                                                                                as_str(): "name",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value: TyExpression {
                                                                                            expression: Literal(
                                                                                                String(
                                                                                                    Span {
                                                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1216,
                                                                                                        end: 1219,
                                                                                                        as_str(): "foo",
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                42588,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 1215,
                                                                                                end: 1220,
                                                                                                as_str(): "\"foo\"",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 1184,
                                                                                    end: 1194,
                                                                                    as_str(): "CustomType",
                                                                                },
                                                                            },
                                                                            return_type: TypeId(
                                                                                42576,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0eb7482d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1184,
                                                                                end: 1230,
                                                                                as_str(): "CustomType {\n            name: \"foo\"\n        }",
                                                                            },
                                                                        },
                                                                    ),
                                                                    enum_instantiation_span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1170,
                                                                        end: 1178,
                                                                        as_str(): "MyResult",
                                                                    },
                                                                    variant_instantiation_span: Span {
                                                                        src (ptr): 0x00007fe0eb7482d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1180,
                                                                        end: 1183,
                                                                        as_str(): "Err",
                                                                    },
                                                                    type_binding: TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0eb7482d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1170,
                                                                            end: 1183,
                                                                            as_str(): "MyResult::Err",
                                                                        },
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    42589,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0eb7482d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1180,
                                                                    end: 1183,
                                                                    as_str(): "Err",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            42590,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0eb7482d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                            ),
                                                            start: 1163,
                                                            end: 1231,
                                                            as_str(): "return MyResult::Err(CustomType {\n            name: \"foo\"\n        })",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 1163,
                                                    end: 1231,
                                                    as_str(): "return MyResult::Err(CustomType {\n            name: \"foo\"\n        })",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7215,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1153,
                                    end: 1238,
                                    as_str(): "{\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    }",
                                },
                            },
                            else: None,
                        },
                        return_type: TypeId(
                            42592,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 1144,
                            end: 1238,
                            as_str(): "if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 1144,
                    end: 1238,
                    as_str(): "if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    }",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 108,
                                                end: 116,
                                                as_str(): "MyResult",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(42595),
                                            E: TypeId(42596),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 129,
                                                        end: 131,
                                                        as_str(): "Ok",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    42595,
                                                ),
                                                initial_type_id: TypeId(
                                                    33094,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 133,
                                                    end: 134,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 129,
                                                    end: 134,
                                                    as_str(): "Ok: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0eb7482d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                        ),
                                                        start: 140,
                                                        end: 143,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    42596,
                                                ),
                                                initial_type_id: TypeId(
                                                    33095,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 145,
                                                    end: 146,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0eb7482d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                    ),
                                                    start: 140,
                                                    end: 146,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 103,
                                            end: 149,
                                            as_str(): "enum MyResult<T, E> {\n    Ok: T,\n    Err: E,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 129,
                                            end: 131,
                                            as_str(): "Ok",
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
                                                src (ptr): 0x00007fe0eb7482d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                                ),
                                                start: 1265,
                                                end: 1270,
                                                as_str(): "false",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1252,
                                        end: 1260,
                                        as_str(): "MyResult",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1262,
                                        end: 1264,
                                        as_str(): "Ok",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0eb7482d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                            ),
                                            start: 1252,
                                            end: 1264,
                                            as_str(): "MyResult::Ok",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    42599,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1262,
                                    end: 1264,
                                    as_str(): "Ok",
                                },
                            },
                        ),
                        return_type: TypeId(
                            42600,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 1245,
                            end: 1271,
                            as_str(): "return MyResult::Ok(false)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 1245,
                    end: 1271,
                    as_str(): "return MyResult::Ok(false)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 1090,
        end: 1274,
        as_str(): "fn sell_product() -> MyResult<bool, CustomType> {\n    if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    };\n\n    return MyResult::Ok(false);\n}",
    },
    attributes: {},
    return_type: TypeId(
        42577,
    ),
    initial_return_type: TypeId(
        42575,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 1111,
        end: 1137,
        as_str(): "MyResult<bool, CustomType>",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0eb7482d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
            ),
            start: 976,
            end: 980,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 989,
                                        end: 1001,
                                        as_str(): "sell_product",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                24598,
                                Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 1090,
                                    end: 1274,
                                    as_str(): "fn sell_product() -> MyResult<bool, CustomType> {\n    if false {\n        return MyResult::Err(CustomType {\n            name: \"foo\"\n        });\n    };\n\n    return MyResult::Ok(false);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 989,
                                        end: 1001,
                                        as_str(): "sell_product",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            42577,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 989,
                            end: 1003,
                            as_str(): "sell_product()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 989,
                    end: 1003,
                    as_str(): "sell_product()",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1009,
                                        end: 1024,
                                        as_str(): "simple_vec_test",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                24600,
                                Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 187,
                                    end: 419,
                                    as_str(): "fn simple_vec_test() {\n    let mut vec1 = Vec::new();\n    let mut vec2 = Vec::new();\n\n    vec2.push(54u32);\n    vec1.push(SomeStruct { a: 42 });\n\n    assert(vec1.get(0).unwrap().a == 42);\n    assert(vec2.get(0).unwrap() == 54u32);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1009,
                                        end: 1024,
                                        as_str(): "simple_vec_test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            42606,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 1009,
                            end: 1026,
                            as_str(): "simple_vec_test()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 1009,
                    end: 1026,
                    as_str(): "simple_vec_test()",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1032,
                                        end: 1048,
                                        as_str(): "complex_vec_test",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                24602,
                                Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 421,
                                    end: 887,
                                    as_str(): "fn complex_vec_test() {\n    let mut exp_vec_in_a_vec_in_a_struct_in_a_vec = Vec::new();\n    let mut inner_vec_1 = Vec::new();\n    let inner_inner_vec_1 = vec_from([0, 1, 2]);\n\n    inner_vec_1.push(inner_inner_vec_1);\n    exp_vec_in_a_vec_in_a_struct_in_a_vec.push(SomeStruct { a: inner_vec_1 });\n\n    assert(inner_vec_1.get(0).unwrap().get(1).unwrap() == 1);\n    assert(exp_vec_in_a_vec_in_a_struct_in_a_vec.get(0).unwrap().a.get(0).unwrap().get(2).unwrap() == 2);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1032,
                                        end: 1048,
                                        as_str(): "complex_vec_test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            42608,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 1032,
                            end: 1050,
                            as_str(): "complex_vec_test()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 1032,
                    end: 1050,
                    as_str(): "complex_vec_test()",
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
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1056,
                                        end: 1083,
                                        as_str(): "simple_option_generics_test",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                24604,
                                Span {
                                    src (ptr): 0x00007fe0eb7482d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                    ),
                                    start: 889,
                                    end: 971,
                                    as_str(): "fn simple_option_generics_test() {\n    assert(get_an_option::<u64>().is_none());\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0eb7482d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                                        ),
                                        start: 1056,
                                        end: 1083,
                                        as_str(): "simple_option_generics_test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            42610,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0eb7482d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                            ),
                            start: 1056,
                            end: 1085,
                            as_str(): "simple_option_generics_test()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0eb7482d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
                    ),
                    start: 1056,
                    end: 1085,
                    as_str(): "simple_option_generics_test()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 973,
        end: 1088,
        as_str(): "fn main() {\n    sell_product();\n    simple_vec_test();\n    complex_vec_test();\n    simple_option_generics_test();\n}",
    },
    attributes: {},
    return_type: TypeId(
        42603,
    ),
    initial_return_type: TypeId(
        42602,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0eb7482d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuzcLhM/generic_type_inference/src/main.sw",
        ),
        start: 973,
        end: 982,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

