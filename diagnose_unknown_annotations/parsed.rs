[
    AstNode {
        content: Declaration(
            AbiDeclaration(
                AbiDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb10cf456b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                            ),
                            start: 52,
                            end: 59,
                            as_str(): "GoodAbi",
                        },
                        is_raw_ident: false,
                    },
                    interface_surface: [
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb10cf456b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                    ),
                                    start: 114,
                                    end: 123,
                                    as_str(): "good_func",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {
                                Doc: [
                                    Attribute {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb10cf456b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                ),
                                                start: 96,
                                                end: 99,
                                                as_str(): "doc",
                                            },
                                            is_raw_ident: false,
                                        },
                                        args: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 100,
                                                    end: 104,
                                                    as_str(): "test",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 94,
                                            end: 106,
                                            as_str(): "#[doc(test)]",
                                        },
                                    },
                                ],
                                Storage: [
                                    Attribute {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb10cf456b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                ),
                                                start: 68,
                                                end: 75,
                                                as_str(): "storage",
                                            },
                                            is_raw_ident: false,
                                        },
                                        args: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 76,
                                                    end: 80,
                                                    as_str(): "read",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 82,
                                                    end: 87,
                                                    as_str(): "write",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 66,
                                            end: 89,
                                            as_str(): "#[storage(read, write)]",
                                        },
                                    },
                                ],
                            },
                            purity: ReadsWrites,
                            parameters: [],
                            return_type: Boolean,
                            return_type_span: Span {
                                src (ptr): 0x00007fb10cf456b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                ),
                                start: 129,
                                end: 133,
                                as_str(): "bool",
                            },
                        },
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb10cf456b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                    ),
                                    start: 165,
                                    end: 173,
                                    as_str(): "bad_func",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            purity: Pure,
                            parameters: [],
                            return_type: Boolean,
                            return_type_span: Span {
                                src (ptr): 0x00007fb10cf456b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                ),
                                start: 179,
                                end: 183,
                                as_str(): "bool",
                            },
                        },
                    ],
                    methods: [],
                    span: Span {
                        src (ptr): 0x00007fb10cf456b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                        ),
                        start: 48,
                        end: 186,
                        as_str(): "abi GoodAbi {\n    #[storage(read, write)]\n    #[doc(test)]\n    fn good_func() -> bool;\n\n    #[bad_attr(blah)]\n    fn bad_func() -> bool;\n}",
                    },
                    attributes: {
                        Doc: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 37,
                                        end: 40,
                                        as_str(): "doc",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 41,
                                            end: 45,
                                            as_str(): "test",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007fb10cf456b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                    ),
                                    start: 35,
                                    end: 47,
                                    as_str(): "#[doc(test)]",
                                },
                            },
                        ],
                        Storage: [
                            Attribute {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb10cf456b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                        ),
                                        start: 13,
                                        end: 20,
                                        as_str(): "storage",
                                    },
                                    is_raw_ident: false,
                                },
                                args: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 21,
                                            end: 25,
                                            as_str(): "read",
                                        },
                                        is_raw_ident: false,
                                    },
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 27,
                                            end: 32,
                                            as_str(): "write",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                span: Span {
                                    src (ptr): 0x00007fb10cf456b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                    ),
                                    start: 11,
                                    end: 34,
                                    as_str(): "#[storage(read, write)]",
                                },
                            },
                        ],
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb10cf456b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
            ),
            start: 11,
            end: 186,
            as_str(): "#[storage(read, write)]\n#[doc(test)]\nabi GoodAbi {\n    #[storage(read, write)]\n    #[doc(test)]\n    fn good_func() -> bool;\n\n    #[bad_attr(blah)]\n    fn bad_func() -> bool;\n}",
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
                                src (ptr): 0x00007fb10cf456b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                ),
                                start: 193,
                                end: 200,
                                as_str(): "GoodAbi",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Contract,
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fb10cf456b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                        ),
                        start: 205,
                        end: 213,
                        as_str(): "Contract",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: ReadsWrites,
                            attributes: {
                                Storage: [
                                    Attribute {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb10cf456b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                ),
                                                start: 222,
                                                end: 229,
                                                as_str(): "storage",
                                            },
                                            is_raw_ident: false,
                                        },
                                        args: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 230,
                                                    end: 234,
                                                    as_str(): "read",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 236,
                                                    end: 241,
                                                    as_str(): "write",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 220,
                                            end: 243,
                                            as_str(): "#[storage(read, write)]",
                                        },
                                    },
                                ],
                                Doc: [
                                    Attribute {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb10cf456b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                ),
                                                start: 250,
                                                end: 253,
                                                as_str(): "doc",
                                            },
                                            is_raw_ident: false,
                                        },
                                        args: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 254,
                                                    end: 258,
                                                    as_str(): "Test",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 248,
                                            end: 260,
                                            as_str(): "#[doc(Test)]",
                                        },
                                    },
                                ],
                            },
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb10cf456b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                    ),
                                    start: 268,
                                    end: 277,
                                    as_str(): "good_func",
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
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 298,
                                                    end: 302,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 298,
                                            end: 302,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fb10cf456b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                    ),
                                    start: 288,
                                    end: 308,
                                    as_str(): "{\n        true\n    }",
                                },
                            },
                            parameters: [],
                            span: Span {
                                src (ptr): 0x00007fb10cf456b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                ),
                                start: 265,
                                end: 308,
                                as_str(): "fn good_func() -> bool {\n        true\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fb10cf456b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                ),
                                start: 283,
                                end: 287,
                                as_str(): "bool",
                            },
                        },
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb10cf456b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                    ),
                                    start: 339,
                                    end: 347,
                                    as_str(): "bad_func",
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
                                                    src (ptr): 0x00007fb10cf456b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                                    ),
                                                    start: 368,
                                                    end: 372,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb10cf456b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                            ),
                                            start: 368,
                                            end: 372,
                                            as_str(): "true",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fb10cf456b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                    ),
                                    start: 358,
                                    end: 378,
                                    as_str(): "{\n        true\n    }",
                                },
                            },
                            parameters: [],
                            span: Span {
                                src (ptr): 0x00007fb10cf456b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                ),
                                start: 336,
                                end: 378,
                                as_str(): "fn bad_func() -> bool {\n        true\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fb10cf456b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                                ),
                                start: 353,
                                end: 357,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fb10cf456b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                        ),
                        start: 188,
                        end: 380,
                        as_str(): "impl GoodAbi for Contract {\n    #[storage(read, write)]\n    #[doc(Test)]\n    fn good_func() -> bool {\n        true\n    }\n\n    #[bad_attr(blah)]\n    fn bad_func() -> bool {\n        true\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb10cf456b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
            ),
            start: 188,
            end: 380,
            as_str(): "impl GoodAbi for Contract {\n    #[storage(read, write)]\n    #[doc(Test)]\n    fn good_func() -> bool {\n        true\n    }\n\n    #[bad_attr(blah)]\n    fn bad_func() -> bool {\n        true\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb10cf456b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                            ),
                            start: 407,
                            end: 416,
                            as_str(): "BadStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [],
                    type_parameters: [],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fb10cf456b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                        ),
                        start: 400,
                        end: 419,
                        as_str(): "struct BadStruct {}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb10cf456b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
            ),
            start: 382,
            end: 419,
            as_str(): "#[bad_attr(blah)]\nstruct BadStruct {}",
        },
    },
]
