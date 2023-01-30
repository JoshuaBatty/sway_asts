TyAbiDeclaration {
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
        DeclId(
            13314,
            Span {
                src (ptr): 0x00007fb10cf456b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                ),
                start: 114,
                end: 123,
                as_str(): "good_func",
            },
        ),
        DeclId(
            13315,
            Span {
                src (ptr): 0x00007fb10cf456b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
                ),
                start: 165,
                end: 173,
                as_str(): "bad_func",
            },
        ),
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
}
ImplTrait(
    DeclId(
        13319,
        Span {
            src (ptr): 0x00007fb10cf456b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRYYC3Yn/diagnose_unknown_annotations/src/main.sw",
            ),
            start: 188,
            end: 380,
            as_str(): "impl GoodAbi for Contract {\n    #[storage(read, write)]\n    #[doc(Test)]\n    fn good_func() -> bool {\n        true\n    }\n\n    #[bad_attr(blah)]\n    fn bad_func() -> bool {\n        true\n    }\n}",
        },
    ),
)
TyStructDeclaration {
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
    attributes: {},
}

