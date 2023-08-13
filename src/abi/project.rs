pub use project::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod project {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BALLOT_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BALLOT_TYPEHASH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CLOCK_MODE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CLOCK_MODE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("COUNTING_MODE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("COUNTING_MODE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EXTENDED_BALLOT_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EXTENDED_BALLOT_TYPEHASH",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cancel"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cancel"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("descriptionHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("castVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("castVote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("castVoteBySig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("castVoteBySig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("castVoteWithReason"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("castVoteWithReason"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("castVoteWithReasonAndParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "castVoteWithReasonAndParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("castVoteWithReasonAndParamsBySig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "castVoteWithReasonAndParamsBySig",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clock"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eip712Domain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eip712Domain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        1usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes1"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("verifyingContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extensions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("descriptionHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProposalIpfsHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getProposalIpfsHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVotes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timepoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getVotesWithParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVotesWithParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timepoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasVoted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasVoted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hashProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("descriptionHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ipfsHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("inludeContributions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "inludeContributions",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155BatchReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "onERC1155BatchReceived",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalDeadline"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalDeadline"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalProposer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalProposer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalSnapshot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalSnapshot"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalThreshold"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposalVotes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("againstVotes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forVotes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("abstainVotes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("propose"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("propose"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("description"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorum"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorumDenominator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumDenominator"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorumNumerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumerator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timepoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumerator"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("relay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("relay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProposalThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setProposalThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newProposalThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setVotingDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setVotingDelay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newVotingDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setVotingPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setVotingPeriod"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newVotingPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("state"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IGovernor.ProposalState",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitContributionsRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "submitContributionsRequest",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ipfsHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("description"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC5805"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("trackerContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trackerContract"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Contributions"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateQuorumNumerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateQuorumNumerator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newQuorumNumerator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("votingDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("votingDelay"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("votingPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("votingPeriod"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EIP712DomainChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EIP712DomainChanged",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalCanceled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("targets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("values"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("calldatas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voteStart"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voteEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("description"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalExecuted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposalThresholdSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProposalThresholdSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldProposalThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newProposalThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QuorumNumeratorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "QuorumNumeratorUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldQuorumNumerator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newQuorumNumerator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VoteCast"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VoteCast"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VoteCastWithParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VoteCastWithParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposalId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("support"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VotingDelaySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VotingDelaySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldVotingDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newVotingDelay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VotingPeriodSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VotingPeriodSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldVotingPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newVotingPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Empty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Empty"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StringTooLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("StringTooLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("str"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PROJECT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\x80`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x84$8\x03\x80b\0\x84$\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\t'V[`B`@Qb\0\0E\x90b\0\x08\xDDV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0bW=`\0\x80>=`\0\xFD[Pa\x1C a\xC4\xE0`\0\x85\x80b\0\0\x8C`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01R\x90V[b\0\0\x99\x82`\0b\0\x01\xB9V[a\x01 Rb\0\0\xAA\x81`\x01b\0\x01\xB9V[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Rb\0\x018`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0R`\x02b\0\x01O\x82\x82b\0\nnV[Pb\0\x01]\x90P\x83b\0\x01\xF2V[b\0\x01h\x82b\0\x023V[b\0\x01s\x81b\0\x02\xDAV[PPP`\x01`\x01`\xA0\x1B\x03\x16a\x01`Rb\0\x01\x8E\x81b\0\x03\x1BV[PPa\x01`Q`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x0B\xE0V[`\0` \x83Q\x10\x15b\0\x01\xD9Wb\0\x01\xD1\x83b\0\x04\x9AV[\x90Pb\0\x01\xECV[\x81b\0\x01\xE6\x84\x82b\0\nnV[P`\xFF\x90P[\x92\x91PPV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[`\0\x81\x11b\0\x02\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x07T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x07UV[`\x08T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x08UV[`d\x81\x11\x15b\0\x03\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01b\0\x02\x90V[`\0b\0\x03\xACb\0\x04\xDDV[\x90P\x80\x15\x80\x15\x90b\0\x03\xBEWP`\x0BT\x15[\x15b\0\x04&W`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R`\x0B\x90` \x81\x01b\0\x03\xE4\x84b\0\x05\x0CV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[b\0\x04[b\0\x04Fb\0\x048b\0\x05{V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\x05\xF2V[b\0\x04Q\x84b\0\x05\x0CV[`\x0B\x91\x90b\0\x06YV[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15b\0\x04\xC8W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01b\0\x02\x90\x91\x90b\0\x0B:V[\x80Qb\0\x04\xD5\x82b\0\x0BoV[\x17\x93\x92PPPV[`\x0BT`\0\x90\x15b\0\x05\x03Wb\0\x04\xF5`\x0Bb\0\x06vV[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[`\nT[\x90P\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15b\0\x05wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01b\0\x02\x90V[P\x90V[`\0a\x01`Q`\x01`\x01`\xA0\x1B\x03\x16c\x91\xDD\xAD\xF4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15b\0\x05\xDDWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Rb\0\x05\xDA\x91\x81\x01\x90b\0\x0B\x94V[`\x01[b\0\x05\xEDWb\0\x05\x07Cb\0\x06\xC4V[\x91\x90PV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x05wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x02\x90V[`\0\x80b\0\x06i\x85\x85\x85b\0\x07-V[\x91P\x91P[\x93P\x93\x91PPV[\x80T`\0\x90\x80\x15b\0\x06\xBAWb\0\x06\xA2\x83b\0\x06\x94`\x01\x84b\0\x0B\xBEV[`\0\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16b\0\x06\xBDV[`\0[\x93\x92PPPV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x05wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x02\x90V[\x82T`\0\x90\x81\x90\x80\x15b\0\x08\x82W`\0b\0\x07O\x87b\0\x06\x94`\x01\x85b\0\x0B\xBEV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15b\0\x07\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01b\0\x02\x90V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03b\0\x08!W\x84b\0\x07\xF8\x88b\0\x06\x94`\x01\x86b\0\x0B\xBEV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x08qV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16d\x01\0\0\0\0\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pb\0\x06n\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16d\x01\0\0\0\0\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81b\0\x06nV[a0\xA9\x80b\0S{\x839\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\t\x1EW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\t\x04V[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15b\0\t:W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\tRW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12b\0\tgW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\t|Wb\0\t|b\0\x08\xEBV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\t\xA7Wb\0\t\xA7b\0\x08\xEBV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15b\0\t\xC1W`\0\x80\xFD[b\0\t\xD4\x83` \x83\x01` \x88\x01b\0\t\x01V[\x97\x96PPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\t\xF4W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\n\x15WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\niW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\nDWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\neW\x82\x81U`\x01\x01b\0\nPV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\n\x8AWb\0\n\x8Ab\0\x08\xEBV[b\0\n\xA2\x81b\0\n\x9B\x84Tb\0\t\xDFV[\x84b\0\n\x1BV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\n\xDAW`\0\x84\x15b\0\n\xC1WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\neV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x0B\x0BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\n\xEAV[P\x85\x82\x10\x15b\0\x0B*W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R`\0\x82Q\x80` \x84\x01Rb\0\x0B[\x81`@\x85\x01` \x87\x01b\0\t\x01V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15b\0\n\x15W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x0B\xA7W`\0\x80\xFD[\x81Qe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x06\xBDW`\0\x80\xFD[\x81\x81\x03\x81\x81\x11\x15b\0\x01\xECWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`QaG\x0Bb\0\x0Cp`\09`\0\x81\x81a\tk\x01R\x81\x81a\x11$\x01R\x81\x81a\x17\xF2\x01R\x81\x81a\x18\x9B\x01R\x81\x81a\x1D2\x01R\x81\x81a)o\x01Ra+3\x01R`\0a\x17\x9E\x01R`\0a\x17t\x01R`\0a,\x94\x01R`\0a,l\x01R`\0a+\xC7\x01R`\0a+\xF1\x01R`\0a,\x1B\x01RaG\x0B`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x8CW`\x005`\xE0\x1C\x80c{<q\xD3\x11a\x01ZW\x80c\xC2\x8B\xC2\xFA\x11a\0\xC1W\x80c\xEC\xE4\x0C\xC1\x11a\0zW\x80c\xEC\xE4\x0C\xC1\x14a\x08\xCDW\x80c\xF2:na\x14a\x08\xEDW\x80c\xF8\xCEV\n\x14a\t\x19W\x80c\xFB\x04\x92\x93\x14a\t9W\x80c\xFC\x0CTj\x14a\tYW\x80c\xFD\xC1\xF8\xD5\x14a\t\x8DW`\0\x80\xFD[\x80c\xC2\x8B\xC2\xFA\x14a\x07\xE0W\x80c\xC5\x90W\xE4\x14a\x07\xF3W\x80c\xDDN+\xA5\x14a\x08\x13W\x80c\xDE\xAA\xA7\xCC\x14a\x08YW\x80c\xEA\x02\x17\xCF\x14a\x08\x8DW\x80c\xEB\x90\x19\xD4\x14a\x08\xADW`\0\x80\xFD[\x80c\x9A\x02\x04&\x11a\x01\x13W\x80c\x9A\x02\x04&\x14a\x07\x11W\x80c\x9A\x80*m\x14a\x071W\x80c\xA7q:p\x14a\x07QW\x80c\xB5\x811\xB0\x14a\x07fW\x80c\xBC\x19|\x81\x14a\x07{W\x80c\xC0\x1F\x9E7\x14a\x07\xA7W`\0\x80\xFD[\x80c{<q\xD3\x14a\x06IW\x80c}^\x81\xE2\x14a\x06iW\x80c\x84\xB0\x19n\x14a\x06\x89W\x80c\x8Bd\xBB\x87\x14a\x06\xB1W\x80c\x91\xDD\xAD\xF4\x14a\x06\xD1W\x80c\x97\xC3\xD34\x14a\x06\xFDW`\0\x80\xFD[\x80c;\xCC\xF4\xFD\x11a\x01\xFEW\x80cTO\xFC\x9C\x11a\x01\xB7W\x80cTO\xFC\x9C\x14a\x05JW\x80cT\xFDMP\x14a\x05\x9FW\x80cVx\x13\x88\x14a\x05\xC9W\x80c_9\x8A\x14\x14a\x05\xE9W\x80c`\xC4$\x7F\x14a\x06\tW\x80cp\xB0\xF6`\x14a\x06)W`\0\x80\xFD[\x80c;\xCC\xF4\xFD\x14a\x04\x88W\x80c>OI\xE6\x14a\x04\xA8W\x80cC\x85\x962\x14a\x04\xD5W\x80cC\x9F\xAB\x91\x14a\x04\xF5W\x80cE!\x15\xD6\x14a\x05\x15W\x80cK\xF5\xD7\xE9\x14a\x055W`\0\x80\xFD[\x80c\x144\x89\xD0\x11a\x02PW\x80c\x144\x89\xD0\x14a\x03]W\x80c\x15\x0Bz\x02\x14a\x03\xB2W\x80c&V\"}\x14a\x03\xF6W\x80c-c\xF6\x93\x14a\x04\tW\x80c/\xE3\xE2a\x14a\x04?W\x80c92\xAB\xB1\x14a\x04sW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02\xA3W\x80c\x02\xA2Q\xA3\x14a\x02\xD8W\x80c\x03B\x01\x81\x14a\x02\xFBW\x80c\x06\xF3\xF9\xE6\x14a\x03\x1BW\x80c\x06\xFD\xDE\x03\x14a\x03;W`\0\x80\xFD[6a\x02\x9EW\0[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x02\xAFW`\0\x80\xFD[Pa\x02\xC3a\x02\xBE6`\x04a5\xFAV[a\t\xADV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xE4W`\0\x80\xFD[Pa\x02\xEDa\nJV[`@Q\x90\x81R` \x01a\x02\xCFV[4\x80\x15a\x03\x07W`\0\x80\xFD[Pa\x02\xEDa\x03\x166`\x04a7HV[a\nZV[4\x80\x15a\x03'W`\0\x80\xFD[Pa\x02\x9Ca\x0366`\x04a7\xEEV[a\x0BRV[4\x80\x15a\x03GW`\0\x80\xFD[Pa\x03Pa\x0B\x94V[`@Qa\x02\xCF\x91\x90a8WV[4\x80\x15a\x03iW`\0\x80\xFD[Pa\x03\x9Aa\x03x6`\x04a7\xEEV[`\0\x90\x81R`\x03` R`@\x90 T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xCFV[4\x80\x15a\x03\xBEW`\0\x80\xFD[Pa\x03\xDDa\x03\xCD6`\x04a8\x81V[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\xCFV[a\x02\xEDa\x04\x046`\x04a:RV[a\x0C&V[4\x80\x15a\x04\x15W`\0\x80\xFD[Pa\x02\xEDa\x04$6`\x04a7\xEEV[`\0\x90\x81R`\x03` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[4\x80\x15a\x04KW`\0\x80\xFD[Pa\x02\xED\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x81V[4\x80\x15a\x04\x7FW`\0\x80\xFD[Pa\x02\xEDa\rRV[4\x80\x15a\x04\x94W`\0\x80\xFD[Pa\x02\xEDa\x04\xA36`\x04a:\xE1V[a\r]V[4\x80\x15a\x04\xB4W`\0\x80\xFD[Pa\x04\xC8a\x04\xC36`\x04a7\xEEV[a\r\xD3V[`@Qa\x02\xCF\x91\x90a;EV[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x02\xC3a\x04\xF06`\x04a;mV[a\x0F\x13V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x02\x9Ca\x05\x106`\x04a;\x99V[a\x0FCV[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\xEDa\x0506`\x04a:RV[a\x10\x19V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x03Pa\x11 V[4\x80\x15a\x05VW`\0\x80\xFD[Pa\x05\x84a\x05e6`\x04a7\xEEV[`\0\x90\x81R`\t` R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xCFV[4\x80\x15a\x05\xABW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01Ra\x03PV[4\x80\x15a\x05\xD5W`\0\x80\xFD[Pa\x02\xEDa\x05\xE46`\x04a;\xDAV[a\x11\xE2V[4\x80\x15a\x05\xF5W`\0\x80\xFD[Pa\x02\xEDa\x06\x046`\x04a;\xFDV[a\x12\x0BV[4\x80\x15a\x06\x15W`\0\x80\xFD[Pa\x02\xEDa\x06$6`\x04a7\xEEV[a\x12UV[4\x80\x15a\x065W`\0\x80\xFD[Pa\x02\x9Ca\x06D6`\x04a7\xEEV[a\x13\x06V[4\x80\x15a\x06UW`\0\x80\xFD[Pa\x02\xEDa\x06d6`\x04a<\x80V[a\x13EV[4\x80\x15a\x06uW`\0\x80\xFD[Pa\x02\xEDa\x06\x846`\x04a<\xD9V[a\x13\x8DV[4\x80\x15a\x06\x95W`\0\x80\xFD[Pa\x06\x9Ea\x17fV[`@Qa\x02\xCF\x97\x96\x95\x94\x93\x92\x91\x90a=\xC8V[4\x80\x15a\x06\xBDW`\0\x80\xFD[P`\x0CTa\x03\x9A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xDDW`\0\x80\xFD[Pa\x06\xE6a\x17\xEEV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xCFV[4\x80\x15a\x07\tW`\0\x80\xFD[P`da\x02\xEDV[4\x80\x15a\x07\x1DW`\0\x80\xFD[Pa\x02\x9Ca\x07,6`\x04a7\xEEV[a\x18wV[4\x80\x15a\x07=W`\0\x80\xFD[Pa\x02\xEDa\x07L6`\x04a>*V[a\x1ArV[4\x80\x15a\x07]W`\0\x80\xFD[Pa\x02\xEDa\x1A\x89V[4\x80\x15a\x07rW`\0\x80\xFD[Pa\x02\xEDa\x1A\xB3V[4\x80\x15a\x07\x87W`\0\x80\xFD[Pa\x03\xDDa\x07\x966`\x04a>\x80V[c\xBC\x19|\x81`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x07\xB3W`\0\x80\xFD[Pa\x02\xEDa\x07\xC26`\x04a7\xEEV[`\0\x90\x81R`\x03` R`@\x90 `\x01\x01T`\x01`\x01`@\x1B\x03\x16\x90V[a\x02\x9Ca\x07\xEE6`\x04a?\x0FV[a\x1A\xBEV[4\x80\x15a\x07\xFFW`\0\x80\xFD[Pa\x02\xEDa\x08\x0E6`\x04a:RV[a\x1B\x85V[4\x80\x15a\x08\x1FW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R` \x80\x82R\x7Fsupport=bravo&quorum=for,abstain\x90\x82\x01Ra\x03PV[4\x80\x15a\x08eW`\0\x80\xFD[Pa\x02\xED\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F\x81V[4\x80\x15a\x08\x99W`\0\x80\xFD[Pa\x02\x9Ca\x08\xA86`\x04a7\xEEV[a\x1B\xBFV[4\x80\x15a\x08\xB9W`\0\x80\xFD[Pa\x02\xEDa\x08\xC86`\x04a?PV[a\x1B\xFEV[4\x80\x15a\x08\xD9W`\0\x80\xFD[Pa\x02\x9Ca\x08\xE86`\x04a7\xEEV[a\x1C\x1FV[4\x80\x15a\x08\xF9W`\0\x80\xFD[Pa\x03\xDDa\t\x086`\x04a?zV[c\xF2:na`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\t%W`\0\x80\xFD[Pa\x02\xEDa\t46`\x04a7\xEEV[a\x1C^V[4\x80\x15a\tEW`\0\x80\xFD[Pa\x03Pa\tT6`\x04a7\xEEV[a\x1CiV[4\x80\x15a\teW`\0\x80\xFD[Pa\x03\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\t\x99W`\0\x80\xFD[Pa\x02\xEDa\t\xA86`\x04a?\xDEV[a\x1D\x0BV[`\0c(\x8A\xCE\x03`\xE1\x1Bc\x18\xDFt?`\xE3\x1Bc\xBF&\xD8\x97`\xE0\x1Bcy\xDDyo`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x82\x14\x80a\t\xF3WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x81\x16\x90\x82\x16\x14[\x80a\n\nWP`\x01`\x01`\xE0\x1B\x03\x19\x86\x81\x16\x90\x85\x16\x14[\x80a\n%WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x16c\x02q\x18\x97`\xE5\x1B\x14[\x80a\n@WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x87\x16\x14[\x96\x95PPPPPPV[`\0a\nU`\x07T\x90V[\x90P\x90V[`\0\x80a\n\xFEa\n\xF6\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x8C\x8C\x8C\x8C`@Qa\n\x96\x92\x91\x90a@=V[`@Q\x80\x91\x03\x90 \x8B\x80Q\x90` \x01 `@Q` \x01a\n\xDB\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R`\xFF\x91\x90\x91\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1E\xDAV[\x86\x86\x86a\x1F\x07V[\x90Pa\x0BD\x8A\x82\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8D\x92Pa\x1F%\x91PPV[\x9A\x99PPPPPPPPPPV[30\x14a\x0BqW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x0B\x88V[\x80a\x0B\x81`\x04a zV[\x03a\x0BvWP[a\x0B\x91\x81a \xF9V[PV[```\x02\x80Ta\x0B\xA3\x90a@\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xCF\x90a@\x84V[\x80\x15a\x0C\x1CW\x80`\x1F\x10a\x0B\xF1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x1CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xFFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80a\x0C5\x86\x86\x86\x86a\x1B\x85V[\x90P`\0a\x0CB\x82a\r\xD3V[\x90P`\x04\x81`\x07\x81\x11\x15a\x0CXWa\x0CXa;/V[\x14\x80a\x0CuWP`\x05\x81`\x07\x81\x11\x15a\x0CsWa\x0Csa;/V[\x14[a\x0C\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal not successfu`D\x82\x01R`\x1B`\xFA\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0\x82\x81R`\x03` R`@\x90\x81\x90 `\x02\x01\x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fq*\xE18?y\xAC\x85?\x8D\x88!Sw\x8E\x02`\xEF\x8F\x03\xB5\x04\xE2\x86n\x05\x93\xE0M+)\x1F\x90a\r\x19\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1a\r.\x82\x88\x88\x88\x88a\"gV[a\r;\x82\x88\x88\x88\x88a\"\xF2V[a\rH\x82\x88\x88\x88\x88a\"gV[P\x95\x94PPPPPV[`\0a\nU`\x06T\x90V[`@\x80Q\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F` \x82\x01R\x90\x81\x01\x86\x90R`\xFF\x85\x16``\x82\x01R`\0\x90\x81\x90a\r\xAB\x90a\n\xF6\x90`\x80\x01a\n\xDBV[\x90Pa\r\xC8\x87\x82\x88`@Q\x80` \x01`@R\x80`\0\x81RPa#\xE8V[\x97\x96PPPPPPPV[`\0\x81\x81R`\x03` R`@\x81 `\x02\x81\x01T`\xFF\x16\x15a\r\xF7WP`\x07\x92\x91PPV[`\x02\x81\x01Ta\x01\0\x90\x04`\xFF\x16\x15a\x0E\x12WP`\x02\x92\x91PPV[`\0\x83\x81R`\x03` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90\x03a\x0EzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: unknown proposal id\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0a\x0E\x84a\x17\xEEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x82\x10a\x0E\xA0WP`\0\x94\x93PPPPV[`\0\x85\x81R`\x03` R`@\x90 `\x01\x01T`\x01`\x01`@\x1B\x03\x16\x81\x81\x10a\x0E\xCEWP`\x01\x95\x94PPPPPV[a\x0E\xD7\x86a$\x14V[\x80\x15a\x0E\xF6WP`\0\x86\x81R`\t` R`@\x90 \x80T`\x01\x90\x91\x01T\x11[\x15a\x0F\x07WP`\x04\x95\x94PPPPPV[P`\x03\x95\x94PPPPPV[`\0\x82\x81R`\t` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R`\x03\x01\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0F\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FProject: already initialized\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\x0C\x80T`\xFF`\xA0\x1B\x19\x81\x16`\x01`\xA0\x1B\x17\x90\x91U`@Qc<-\xD8\x1F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx[\xB0>\x90a\x0F\xE3\x903\x90\x86\x90\x86\x90`\x04\x01a@\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x11W=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x80a\x10(\x86\x86\x86\x86a\x1B\x85V[\x90P`\0a\x105\x82a\r\xD3V[`\x07\x81\x11\x15a\x10FWa\x10Fa;/V[\x14a\x10\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FGovernor: too late to cancel\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FGovernor: only proposer can canc`D\x82\x01Ra\x19[`\xF2\x1B`d\x82\x01R`\x84\x01a\x02\x93V[a\n@\x86\x86\x86\x86a$`V[``\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cK\xF5\xD7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x11\xA1WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\x9E\x91\x90\x81\x01\x90a@\xFEV[`\x01[a\x11\xDDWP`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R\x90V[\x91\x90PV[`\0\x803\x90Pa\x12\x03\x84\x82\x85`@Q\x80` \x01`@R\x80`\0\x81RPa#\xE8V[\x94\x93PPPPV[`\0\x803\x90Pa\r\xC8\x87\x82\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92Pa\x1F%\x91PPV[`\x0BT`\0\x90\x80\x82\x03a\x12lWPP`\nT\x91\x90PV[`\0`\x0Ba\x12{`\x01\x84aA\x81V[\x81T\x81\x10a\x12\x8BWa\x12\x8BaA\x94V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`\x01` \x1B\x90\x91\x04`\x01`\x01`\xE0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R\x91P\x84\x10a\x12\xE1W` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x12\xF5a\x12\xED\x85a%~V[`\x0B\x90a%\xE7V[`\x01`\x01`\xE0\x1B\x03\x16\x94\x93PPPPV[30\x14a\x13%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x13<V[\x80a\x135`\x04a zV[\x03a\x13*WP[a\x0B\x91\x81a&\x9AV[`\0\x803\x90Pa\n@\x86\x82\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa#\xE8\x92PPPV[`\x003a\x13\x9A\x81\x84a&\xDBV[a\x13\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: proposer restricted\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0a\x13\xF0a\x17\xEEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x14\x02a\x1A\xB3V[a\x14\x11\x83a\x08\xC8`\x01\x85aA\x81V[\x10\x15a\x14yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FGovernor: proposer votes below p`D\x82\x01Rp\x1C\x9B\xDC\x1B\xDC\xD8[\x08\x1D\x1A\x1C\x99\\\xDA\x1B\xDB\x19`z\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0a\x14\x8E\x88\x88\x88\x88\x80Q\x90` \x01 a\x1B\x85V[\x90P\x86Q\x88Q\x14a\x14\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90aA\xAAV[\x85Q\x88Q\x14a\x14\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90aA\xAAV[`\0\x88Q\x11a\x15#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FGovernor: empty proposal\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15a\x15\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal already exist`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0a\x15\x9Ca\rRV[a\x15\xA6\x90\x84aA\xEBV[\x90P`\0a\x15\xB2a\nJV[a\x15\xBC\x90\x83aA\xEBV[\x90P`@Q\x80`\xE0\x01`@R\x80a\x15\xD2\x84a'\xCCV[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R`\0`@\x82\x01R``\x01a\x15\xFF\x83a'\xCCV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\0` \x80\x84\x01\x82\x90R`@\x80\x85\x01\x83\x90R``\x94\x85\x01\x83\x90R\x88\x83R`\x03\x82R\x91\x82\x90 \x85Q\x81T\x92\x87\x01Q\x87\x85\x01Q\x91\x86\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84\x02\x17`\x01`\x01`\xE0\x1B\x03\x16`\x01`\xE0\x1B`\xE0\x92\x90\x92\x1C\x91\x90\x91\x02\x17\x81U\x93\x85\x01Q`\x80\x86\x01Q\x90\x84\x16\x92\x1C\x02\x17`\x01\x83\x01U`\xA0\x83\x01Q`\x02\x90\x92\x01\x80T`\xC0\x90\x94\x01Qa\xFF\xFF\x19\x90\x94\x16\x92\x15\x15a\xFF\0\x19\x16\x92\x90\x92\x17a\x01\0\x93\x15\x15\x93\x90\x93\x02\x92\x90\x92\x17\x90U\x8AQ\x7F}\x84\xA6&:\xE0\xD9\x8D3)\xBD{F\xBBN\x8Do\x98\xCD5\xA7\xAD\xB4\\'L\x8B\x7F\xD5\xEB\xD5\xE0\x91\x85\x91\x88\x91\x8E\x91\x8E\x91\x81\x11\x15a\x17\x03Wa\x17\x03a6}V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x176W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17!W\x90P[P\x8D\x88\x88\x8F`@Qa\x17P\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aB\x8CV[`@Q\x80\x91\x03\x90\xA1P\x90\x98\x97PPPPPPPPV[`\0``\x80\x82\x80\x80\x83a\x17\x99\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a(4V[a\x17\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a(4V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x91\xDD\xAD\xF4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x18jWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x18g\x91\x81\x01\x90aCcV[`\x01[a\x11\xDDWa\nUCa(\xDFV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x18\xCDWa\x18\xCDaA\x94V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x19\x1FWa\x19\x1FaA\x94V[` \x02` \x01\x01\x81\x81RPP`\x003`\r`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q`$\x01a\x19R\x92\x91\x90aC\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c<-\xD8\x1F`\xE1\x1B\x17\x90R\x81Q`\x01\x80\x82R\x81\x84\x01\x90\x93R\x92\x93P`\0\x92\x91\x90\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\x95W\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x19\xC0Wa\x19\xC0aA\x94V[` \x02` \x01\x01\x81\x90RPa\x10\x11\x84\x84\x83`\x0E`\0\x8A\x81R` \x01\x90\x81R` \x01`\0 \x80Ta\x19\xEF\x90a@\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x1B\x90a@\x84V[\x80\x15a\x1AhW\x80`\x1F\x10a\x1A=Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1AhV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1AKW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x13\x8DV[`\0a\x1A\x7F\x84\x84\x84a)FV[\x90P[\x93\x92PPPV[`\x0BT`\0\x90\x15a\x1A\xACWa\x1A\x9E`\x0Ba)\xDCV[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[P`\nT\x90V[`\0a\nU`\x08T\x90V[30\x14a\x1A\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x1A\xF4V[\x80a\x1A\xED`\x04a zV[\x03a\x1A\xE2WP[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85\x85`@Qa\x1B\x12\x92\x91\x90a@=V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1BOW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1BTV[``\x91P[P\x91P\x91Pa\x1B|\x82\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01aF\x87`(\x919a*\x15V[PPPPPPPV[`\0\x84\x84\x84\x84`@Q` \x01a\x1B\x9E\x94\x93\x92\x91\x90aD&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[30\x14a\x1B\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x1B\xF5V[\x80a\x1B\xEE`\x04a zV[\x03a\x1B\xE3WP[a\x0B\x91\x81a*.V[`\0a\x1A\x82\x83\x83a\x1C\x1A`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a)FV[30\x14a\x1C>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x1CUV[\x80a\x1CN`\x04a zV[\x03a\x1CCWP[a\x0B\x91\x81a*\xCFV[`\0a\x0F=\x82a+\x10V[`\0\x81\x81R`\r` R`@\x90 \x80T``\x91\x90a\x1C\x86\x90a@\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xB2\x90a@\x84V[\x80\x15a\x1C\xFFW\x80`\x1F\x10a\x1C\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x82\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x1DdWa\x1DdaA\x94V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x1D\xB6Wa\x1D\xB6aA\x94V[` \x02` \x01\x01\x81\x81RPP`\x003\x88\x88`@Q`$\x01a\x1D\xD9\x93\x92\x91\x90a@\xBEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c<-\xD8\x1F`\xE1\x1B\x17\x90R\x81Q`\x01\x80\x82R\x81\x84\x01\x90\x93R\x92\x93P`\0\x92\x91\x90\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\x1CW\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x1EGWa\x1EGaA\x94V[` \x02` \x01\x01\x81\x90RP`\0a\x1E\x96\x85\x85\x84\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x13\x8D\x92PPPV[`\0\x81\x81R`\r` R`@\x90 \x90\x91Pa\x1E\xB2\x8A\x8C\x83aD\xBCV[P`\0\x81\x81R`\x0E` R`@\x90 a\x1E\xCC\x88\x8A\x83aD\xBCV[P\x99\x98PPPPPPPPPV[`\0a\x0F=a\x1E\xE7a+\xBAV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x1F\x18\x87\x87\x87\x87a,\xE5V[\x91P\x91Pa\rH\x81a-\xA9V[`\0\x85\x81R`\x03` R`@\x81 `\x01a\x1F>\x88a\r\xD3V[`\x07\x81\x11\x15a\x1FOWa\x1FOa;/V[\x14a\x1F\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FGovernor: vote not currently act`D\x82\x01Rbive`\xE8\x1B`d\x82\x01R`\x84\x01a\x02\x93V[\x80T`\0\x90a\x1F\xC2\x90\x88\x90`\x01`\x01`@\x1B\x03\x16\x86a)FV[\x90Pa\x1F\xD1\x88\x88\x88\x84\x88a.\xF3V[\x83Q`\0\x03a &W\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB8\xE18\x88}\n\xA1;\xABD~\x82\xDE\x9D\\\x17w\x04\x1E\xCD!\xCA6\xBA\x82O\xF1\xE6\xC0}\xDD\xA4\x89\x88\x84\x89`@Qa \x19\x94\x93\x92\x91\x90aE{V[`@Q\x80\x91\x03\x90\xA2a\r\xC8V[\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE2\xBA\xBF\xBA\xC5\x88\x9Ap\x9Bc\xBB\x7FY\x8B2N\x08\xBCZO\xB9\xECd\x7F\xB3\xCB\xC9\xEC\x07\xEB\x87\x12\x89\x88\x84\x89\x89`@Qa g\x95\x94\x93\x92\x91\x90aE\xA3V[`@Q\x80\x91\x03\x90\xA2\x97\x96PPPPPPPV[`\0a \x95\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a \xB3W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`d\x81\x11\x15a!|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x02\x93V[`\0a!\x86a\x1A\x89V[\x90P\x80\x15\x80\x15\x90a!\x97WP`\x0BT\x15[\x15a!\xFBW`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R`\x0B\x90` \x81\x01a!\xBA\x84a0mV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[a\"(a\"\x16a\"\ta\x17\xEEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a%~V[a\"\x1F\x84a0mV[`\x0B\x91\x90a0\xD6V[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\"\xEBV[\x84Q\x81\x10\x15a\x10\x11W0`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\"\x91Wa\"\x91aA\x94V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\"\xDBWa\"\xDB\x83\x82\x81Q\x81\x10a\"\xBCWa\"\xBCaA\x94V[` \x02` \x01\x01Q\x80Q\x90` \x01 `\x04a0\xF1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\"\xE4\x81aE\xE9V[\x90Pa\"lV[PPPPPV[`\0`@Q\x80``\x01`@R\x80`'\x81R` \x01aF\xAF`'\x919\x90P`\0[\x85Q\x81\x10\x15a\x1B|W`\0\x80\x87\x83\x81Q\x81\x10a#0Wa#0aA\x94V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x87\x84\x81Q\x81\x10a#SWa#SaA\x94V[` \x02` \x01\x01Q\x87\x85\x81Q\x81\x10a#mWa#maA\x94V[` \x02` \x01\x01Q`@Qa#\x82\x91\x90aF\x02V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a#\xBFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a#\xC4V[``\x91P[P\x91P\x91Pa#\xD4\x82\x82\x86a*\x15V[PPP\x80a#\xE1\x90aE\xE9V[\x90Pa#\x12V[`\0a$\x0B\x85\x85\x85\x85a$\x06`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a\x1F%V[\x95\x94PPPPPV[`\0\x81\x81R`\t` R`@\x81 `\x02\x81\x01T`\x01\x82\x01Ta$6\x91\x90aA\xEBV[`\0\x84\x81R`\x03` R`@\x90 Ta$W\x90`\x01`\x01`@\x1B\x03\x16a\x1C^V[\x11\x15\x93\x92PPPV[`\0\x80a$o\x86\x86\x86\x86a\x1B\x85V[\x90P`\0a$|\x82a\r\xD3V[\x90P`\x02\x81`\x07\x81\x11\x15a$\x92Wa$\x92a;/V[\x14\x15\x80\x15a$\xB2WP`\x06\x81`\x07\x81\x11\x15a$\xAFWa$\xAFa;/V[\x14\x15[\x80\x15a$\xD0WP`\x07\x81`\x07\x81\x11\x15a$\xCDWa$\xCDa;/V[\x14\x15[a%\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: proposal not active\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0\x82\x81R`\x03` R`@\x90\x81\x90 `\x02\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UQ\x7Fx\x9C\xF5[\xE9\x80s\x9D\xAD\x1D\x06\x99\xB9;X\xE8\x06\xB5\x1C\x9D\x96a\x9B\xFA\x8F\xE0\xA2\x8A\xBA\xA7\xB3\x0C\x90a%l\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1P\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a%\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\x93V[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a&DW`\0a&\x02\x84a1-V[a&\x0C\x90\x85aA\x81V[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a&4W\x80\x91Pa&BV[a&?\x81`\x01aA\xEBV[\x92P[P[`\0a&R\x87\x87\x85\x85a2\x15V[\x90P\x80\x15a&\x8DWa&w\x87a&i`\x01\x84aA\x81V[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\r\xC8V[`\0\x97\x96PPPPPPPV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[\x80Q`\0\x90`4\x81\x10\x15a&\xF3W`\x01\x91PPa\x0F=V[\x82\x81\x01`\x13\x19\x01Q`\x01`\x01`\xA0\x1B\x03\x19\x81\x16k\x04n\x0EM\xEE\r\xEEl\xAEG\xA6\x0F`\xA3\x1B\x14a'&W`\x01\x92PPPa\x0F=V[`\0\x80a'4`(\x85aA\x81V[\x90P[\x83\x81\x10\x15a'\xABW`\0\x80a'k\x88\x84\x81Q\x81\x10a'WWa'WaA\x94V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16a2sV[\x91P\x91P\x81a'\x83W`\x01\x96PPPPPPPa\x0F=V[\x80`\xFF\x16`\x04\x85`\x01`\x01`\xA0\x1B\x03\x16\x90\x1B\x17\x93PPP\x80a'\xA4\x90aE\xE9V[\x90Pa'7V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x93PPPP\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a%\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 6`D\x82\x01Re4 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\x93V[```\xFF\x83\x14a(NWa(G\x83a3\x05V[\x90Pa\x0F=V[\x81\x80Ta(Z\x90a@\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\x86\x90a@\x84V[\x80\x15a(\xD3W\x80`\x1F\x10a(\xA8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xD3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x0F=V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c:F\xB1\xA8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x7F\x91\x90aF\x1EV[\x80T`\0\x90\x80\x15a*\x0CWa)\xF6\x83a&i`\x01\x84aA\x81V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x1A\x82V[`\0\x93\x92PPPV[``\x83\x15a*$WP\x81a\x1A\x82V[a\x1A\x82\x83\x83a3DV[`\0\x81\x11a*\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\x07T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x07UV[`\x08T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x08UV[`\0`da+\x1D\x83a\x12UV[`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8ES\x9E\x8C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xA6\x91\x90aF\x1EV[a+\xB0\x91\x90aF7V[a\x0F=\x91\x90aFdV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a,\x13WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a,=WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\nU`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a-\x1CWP`\0\x90P`\x03a-\xA0V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a-pW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a-\x99W`\0`\x01\x92P\x92PPa-\xA0V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a-\xBDWa-\xBDa;/V[\x03a-\xC5WPV[`\x01\x81`\x04\x81\x11\x15a-\xD9Wa-\xD9a;/V[\x03a.&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\x02\x81`\x04\x81\x11\x15a.:Wa.:a;/V[\x03a.\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02\x93V[`\x03\x81`\x04\x81\x11\x15a.\x9BWa.\x9Ba;/V[\x03a\x0B\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0\x85\x81R`\t` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R`\x03\x81\x01\x90\x92R\x90\x91 T`\xFF\x16\x15a/{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorVotingSimple: vote alrea`D\x82\x01Rf\x19\x1EH\x18\xD8\\\xDD`\xCA\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\xFF\x84\x16a/\xC7W\x82\x81`\0\x01`\0\x82\x82Ta/\xBC\x91\x90aA\xEBV[\x90\x91UPa\x10\x11\x90PV[`\0\x19`\xFF\x85\x16\x01a/\xE7W\x82\x81`\x01\x01`\0\x82\x82Ta/\xBC\x91\x90aA\xEBV[`\x01\x19`\xFF\x85\x16\x01a0\x07W\x82\x81`\x02\x01`\0\x82\x82Ta/\xBC\x91\x90aA\xEBV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FGovernorVotingSimple: invalid va`D\x82\x01Rtlue for enum VoteType`X\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a%\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0\x80a0\xE4\x85\x85\x85a3nV[\x91P\x91P[\x93P\x93\x91PPV[\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B`\0\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[`\0\x81`\0\x03a1?WP`\0\x91\x90PV[`\0`\x01a1L\x84a5\rV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a1eWa1eaFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1}Wa1}aFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\x95Wa1\x95aFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\xADWa1\xADaFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\xC5Wa1\xC5aFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\xDDWa1\xDDaFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\xF5Wa1\xF5aFNV[\x04\x82\x01\x90\x1C\x90Pa\x1A\x82\x81\x82\x85\x81a2\x0FWa2\x0FaFNV[\x04a5\xA1V[`\0[\x81\x83\x10\x15a2kW`\0a2,\x84\x84a5\xB7V[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a2WW\x80\x92Pa2eV[a2b\x81`\x01aA\xEBV[\x93P[Pa2\x18V[P\x93\x92PPPV[`\0\x80`\xF8\x83\x90\x1C`/\x81\x11\x80\x15a2\x8EWP`:\x81`\xFF\x16\x10[\x15a2\xA3W`\x01\x94`/\x19\x90\x91\x01\x93P\x91PPV[\x80`\xFF\x16`@\x10\x80\x15a2\xB9WP`G\x81`\xFF\x16\x10[\x15a2\xCEW`\x01\x94`6\x19\x90\x91\x01\x93P\x91PPV[\x80`\xFF\x16``\x10\x80\x15a2\xE4WP`g\x81`\xFF\x16\x10[\x15a2\xF9W`\x01\x94`V\x19\x90\x91\x01\x93P\x91PPV[P`\0\x93\x84\x93P\x91PPV[```\0a3\x12\x83a5\xD2V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[\x81Q\x15a3TW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x91\x90a8WV[\x82T`\0\x90\x81\x90\x80\x15a4\xB4W`\0a3\x8C\x87a&i`\x01\x85aA\x81V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a4\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a4UW\x84a4.\x88a&i`\x01\x86aA\x81V[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua4\xA4V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01` \x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa0\xE9\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01` \x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a0\xE9V[`\0\x80`\x80\x83\x90\x1C\x15a5\"W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a54W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a5FW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a5XW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a5jW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a5|W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a5\x8EW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x0F=W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a5\xB0W\x81a\x1A\x82V[P\x90\x91\x90PV[`\0a5\xC6`\x02\x84\x84\x18aFdV[a\x1A\x82\x90\x84\x84\x16aA\xEBV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x0F=W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a6\x0CW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1A\x82W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a\x11\xDDW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a6GW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a6^W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a6vW`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a6\xBBWa6\xBBa6}V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a6\xDCWa6\xDCa6}V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a6\xFDa6\xF8\x84a6\xC3V[a6\x93V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a7\x11W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a79W`\0\x80\xFD[a\x1A\x82\x83\x835` \x85\x01a6\xEAV[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15a7dW`\0\x80\xFD[\x885\x97Pa7t` \x8A\x01a6$V[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7\x90W`\0\x80\xFD[a7\x9C\x8C\x83\x8D\x01a65V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a7\xB5W`\0\x80\xFD[Pa7\xC2\x8B\x82\x8C\x01a7(V[\x94PPa7\xD1`\x80\x8A\x01a6$V[\x92P`\xA0\x89\x015\x91P`\xC0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a8\0W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a8\"W\x81\x81\x01Q\x83\x82\x01R` \x01a8\nV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra8C\x81` \x86\x01` \x86\x01a8\x07V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1A\x82` \x83\x01\x84a8+V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\xDDW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a8\x97W`\0\x80\xFD[a8\xA0\x85a8jV[\x93Pa8\xAE` \x86\x01a8jV[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xD0W`\0\x80\xFD[a8\xDC\x87\x82\x88\x01a7(V[\x91PP\x92\x95\x91\x94P\x92PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a9\x01Wa9\x01a6}V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a9\x1CW`\0\x80\xFD[\x815` a9,a6\xF8\x83a8\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a9KW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a9mWa9`\x81a8jV[\x83R\x91\x83\x01\x91\x83\x01a9OV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a9\x89W`\0\x80\xFD[\x815` a9\x99a6\xF8\x83a8\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a9\xB8W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a9mW\x805\x83R\x91\x83\x01\x91\x83\x01a9\xBCV[`\0\x82`\x1F\x83\x01\x12a9\xE4W`\0\x80\xFD[\x815` a9\xF4a6\xF8\x83a8\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a:\x13W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a9mW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a:6W`\0\x80\x81\xFD[a:D\x89\x86\x83\x8B\x01\x01a7(V[\x84RP\x91\x83\x01\x91\x83\x01a:\x17V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:hW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\x7FW`\0\x80\xFD[a:\x8B\x88\x83\x89\x01a9\x0BV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a:\xA1W`\0\x80\xFD[a:\xAD\x88\x83\x89\x01a9xV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a:\xC3W`\0\x80\xFD[Pa:\xD0\x87\x82\x88\x01a9\xD3V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a:\xF9W`\0\x80\xFD[\x855\x94Pa;\t` \x87\x01a6$V[\x93Pa;\x17`@\x87\x01a6$V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x08\x83\x10a;gWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`@\x83\x85\x03\x12\x15a;\x80W`\0\x80\xFD[\x825\x91Pa;\x90` \x84\x01a8jV[\x90P\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a;\xACW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a;\xC2W`\0\x80\xFD[a;\xCE\x85\x82\x86\x01a65V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a;\xEDW`\0\x80\xFD[\x825\x91Pa;\x90` \x84\x01a6$V[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a<\x15W`\0\x80\xFD[\x855\x94Pa<%` \x87\x01a6$V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<AW`\0\x80\xFD[a<M\x89\x83\x8A\x01a65V[\x90\x95P\x93P``\x88\x015\x91P\x80\x82\x11\x15a<fW`\0\x80\xFD[Pa<s\x88\x82\x89\x01a7(V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a<\x96W`\0\x80\xFD[\x845\x93Pa<\xA6` \x86\x01a6$V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xC1W`\0\x80\xFD[a<\xCD\x87\x82\x88\x01a65V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a<\xEFW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=\x06W`\0\x80\xFD[a=\x12\x88\x83\x89\x01a9\x0BV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a=(W`\0\x80\xFD[a=4\x88\x83\x89\x01a9xV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a=JW`\0\x80\xFD[a=V\x88\x83\x89\x01a9\xD3V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a=lW`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a=~W`\0\x80\xFD[a8\xDC\x87\x825` \x84\x01a6\xEAV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a=\xBDW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a=\xA1V[P\x94\x95\x94PPPPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a=\xE7`\xE0\x83\x01\x89a8+V[\x82\x81\x03`@\x84\x01Ra=\xF9\x81\x89a8+V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x90Pa\x0BD\x81\x85a=\x8DV[`\0\x80`\0``\x84\x86\x03\x12\x15a>?W`\0\x80\xFD[a>H\x84a8jV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a>jW`\0\x80\xFD[a>v\x86\x82\x87\x01a7(V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a>\x98W`\0\x80\xFD[a>\xA1\x86a8jV[\x94Pa>\xAF` \x87\x01a8jV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>\xCBW`\0\x80\xFD[a>\xD7\x89\x83\x8A\x01a9xV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a>\xEDW`\0\x80\xFD[a>\xF9\x89\x83\x8A\x01a9xV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a<fW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a?%W`\0\x80\xFD[a?.\x85a8jV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xC1W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a?cW`\0\x80\xFD[a?l\x83a8jV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a?\x92W`\0\x80\xFD[a?\x9B\x86a8jV[\x94Pa?\xA9` \x87\x01a8jV[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xD2W`\0\x80\xFD[a<s\x88\x82\x89\x01a7(V[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a?\xF4W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\x0BW`\0\x80\xFD[a@\x17\x88\x83\x89\x01a65V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a@0W`\0\x80\xFD[Pa<\xCD\x87\x82\x88\x01a65V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x80\x82R`\x18\x90\x82\x01R\x7FGovernor: onlyGovernance\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a@\x98W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a@\xB8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aA\x10W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aA&W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aA7W`\0\x80\xFD[\x80QaAEa6\xF8\x82a6\xC3V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aAZW`\0\x80\xFD[a$\x0B\x82` \x83\x01` \x86\x01a8\x07V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0F=Wa\x0F=aAkV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`!\x90\x82\x01R\x7FGovernor: invalid proposal lengt`@\x82\x01R`\r`\xFB\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0F=Wa\x0F=aAkV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a=\xBDW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aB\x12V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aB\x7FW\x82\x84\x03\x89RaBm\x84\x83Qa8+V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aBUV[P\x91\x97\x96PPPPPPPV[`\0a\x01 \x8B\x83R` `\x01\x80`\xA0\x1B\x03\x8C\x16\x81\x85\x01R\x81`@\x85\x01RaB\xB5\x82\x85\x01\x8CaA\xFEV[\x91P\x83\x82\x03``\x85\x01RaB\xC9\x82\x8Ba=\x8DV[\x91P\x83\x82\x03`\x80\x85\x01R\x81\x89Q\x80\x84R\x82\x84\x01\x91P\x82\x81`\x05\x1B\x85\x01\x01\x83\x8C\x01`\0[\x83\x81\x10\x15aC\x1AW`\x1F\x19\x87\x84\x03\x01\x85RaC\x08\x83\x83Qa8+V[\x94\x86\x01\x94\x92P\x90\x85\x01\x90`\x01\x01aB\xECV[PP\x86\x81\x03`\xA0\x88\x01RaC.\x81\x8CaB7V[\x94PPPPP\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01RaCS\x81\x85a8+V[\x9C\x9BPPPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15aCuW`\0\x80\xFD[\x81Qe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x82W`\0\x80\xFD[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R`\0\x84TaC\xAD\x81a@\x84V[\x80`@\x87\x01R```\x01\x80\x84\x16`\0\x81\x14aC\xCFW`\x01\x81\x14aC\xE9WaD\x17V[`\xFF\x19\x85\x16\x89\x84\x01R\x83\x15\x15`\x05\x1B\x89\x01\x83\x01\x95PaD\x17V[\x89`\0R\x86`\0 `\0[\x85\x81\x10\x15aD\x0FW\x81T\x8B\x82\x01\x86\x01R\x90\x83\x01\x90\x88\x01aC\xF4V[\x8A\x01\x84\x01\x96PP[P\x93\x99\x98PPPPPPPPPV[`\x80\x81R`\0aD9`\x80\x83\x01\x87aA\xFEV[\x82\x81\x03` \x84\x01RaDK\x81\x87a=\x8DV[\x90P\x82\x81\x03`@\x84\x01RaD_\x81\x86aB7V[\x91PP\x82``\x83\x01R\x95\x94PPPPPV[`\x1F\x82\x11\x15aD\xB7W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aD\x98WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x10\x11W\x82\x81U`\x01\x01aD\xA4V[PPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15aD\xD3WaD\xD3a6}V[aD\xE7\x83aD\xE1\x83Ta@\x84V[\x83aDqV[`\0`\x1F\x84\x11`\x01\x81\x14aE\x1BW`\0\x85\x15aE\x03WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\"\xEBV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15aELW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aE,V[P\x86\x82\x10\x15aEiW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x84\x81R`\xFF\x84\x16` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\n@`\x80\x83\x01\x84a8+V[\x85\x81R`\xFF\x85\x16` \x82\x01R\x83`@\x82\x01R`\xA0``\x82\x01R`\0aE\xCB`\xA0\x83\x01\x85a8+V[\x82\x81\x03`\x80\x84\x01RaE\xDD\x81\x85a8+V[\x98\x97PPPPPPPPV[`\0`\x01\x82\x01aE\xFBWaE\xFBaAkV[P`\x01\x01\x90V[`\0\x82QaF\x14\x81\x84` \x87\x01a8\x07V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aF0W`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0F=Wa\x0F=aAkV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aF\x81WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFEGovernor: relay reverted without messageGovernor: call reverted without message\xA2dipfsX\"\x12 \xEF]\xA7\x1E\x1C)Q\x86\xBB\xCB\xC2\x06\x9Bb7V\x96 J~\x87\xAC\x1D3\xC1\xA7\x03\xA8\xC3y\xDC\x8DdsolcC\0\x08\x14\x003a\x01``@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\r\x80\x82RlContributions`\x98\x1B` \x80\x84\x01\x82\x90R\x84Q\x80\x86\x01\x86R`\x01\x81R`1`\xF8\x1B\x81\x83\x01R\x85Q\x80\x87\x01\x87R\x93\x84R\x83\x82\x01\x92\x90\x92R\x84Q\x80\x86\x01\x90\x95R`\x04\x85Rc!\xA7*)`\xE1\x1B\x90\x85\x01R\x91\x92`\0b\0\0\x87\x83\x82b\0\x02\xEEV[P`\x01b\0\0\x96\x82\x82b\0\x02\xEEV[PPPb\0\0\xB3b\0\0\xADb\0\x01n` \x1B` \x1CV[b\0\x01rV[b\0\0\xC0\x82`\x07b\0\x01\xC4V[a\x01 Rb\0\0\xD1\x81`\x08b\0\x01\xC4V[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Rb\0\x01_`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0Rb\0\x04/V[3\x90V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0` \x83Q\x10\x15b\0\x01\xE4Wb\0\x01\xDC\x83b\0\x01\xFDV[\x90Pb\0\x01\xF7V[\x81b\0\x01\xF1\x84\x82b\0\x02\xEEV[P`\xFF\x90P[\x92\x91PPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15b\0\x024W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01b\0\x02+\x91\x90b\0\x03\xBAV[`@Q\x80\x91\x03\x90\xFD[\x80Qb\0\x02A\x82b\0\x04\nV[\x17\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x02\x95WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\xE9W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\xC4WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\xE5W\x82\x81U`\x01\x01b\0\x02\xD0V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\nWb\0\x03\nb\0\x02IV[b\0\x03\"\x81b\0\x03\x1B\x84Tb\0\x02_V[\x84b\0\x02\x9BV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03ZW`\0\x84\x15b\0\x03AWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\xE5V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03\x8BW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03jV[P\x85\x82\x10\x15b\0\x03\xAAW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15b\0\x03\xE9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01b\0\x03\xCBV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15b\0\x02\x95W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa,\x1Fb\0\x04\x8A`\09`\0a\n\x96\x01R`\0a\nk\x01R`\0a\x11\xA1\x01R`\0a\x11y\x01R`\0a\x10\xD4\x01R`\0a\x10\xFE\x01R`\0a\x11(\x01Ra,\x1F`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xE5W`\x005`\xE0\x1C\x80cx[\xB0>\x11a\x01\x0FW\x80c\x9A\xB2N\xB0\x11a\0\xA2W\x80c\xC3\xCD\xA5 \x11a\0qW\x80c\xC3\xCD\xA5 \x14a\x04\x1EW\x80c\xC8{V\xDD\x14a\x041W\x80c\xE9\x85\xE9\xC5\x14a\x04DW\x80c\xF2\xFD\xE3\x8B\x14a\x04\x80W`\0\x80\xFD[\x80c\x9A\xB2N\xB0\x14a\x03\xD2W\x80c\xA2,\xB4e\x14a\x03\xE5W\x80c\xAE6\xE8\xC0\x14a\x03\xF8W\x80c\xB8\x8DO\xDE\x14a\x04\x0BW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xDEW\x80c\x8D\xA5\xCB[\x14a\x03\x87W\x80c\x8ES\x9E\x8C\x14a\x03\x98W\x80c\x91\xDD\xAD\xF4\x14a\x03\xABW\x80c\x95\xD8\x9BA\x14a\x03\xCAW`\0\x80\xFD[\x80cx[\xB0>\x14a\x03>W\x80c~\xCE\xBE\0\x14a\x03QW\x80c\x80\xAD\xE0l\x14a\x03dW\x80c\x84\xB0\x19n\x14a\x03lW`\0\x80\xFD[\x80cB\x84.\x0E\x11a\x01\x87W\x80c\\\x19\xA9\\\x11a\x01VW\x80c\\\x19\xA9\\\x14a\x02\xFDW\x80ccR!\x1E\x14a\x03\x10W\x80cp\xA0\x821\x14a\x03#W\x80cqP\x18\xA6\x14a\x036W`\0\x80\xFD[\x80cB\x84.\x0E\x14a\x02\xA3W\x80cI%\xECU\x14a\x02\xB6W\x80cK\xF5\xD7\xE9\x14a\x02\xC9W\x80cX|\xDE\x1E\x14a\x02\xD1W`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x01\xC3W\x80c\t^\xA7\xB3\x14a\x02RW\x80c#\xB8r\xDD\x14a\x02gW\x80c6D\xE5\x15\x14a\x02zW\x80c:F\xB1\xA8\x14a\x02\x90W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xEAW\x80c\x06\xFD\xDE\x03\x14a\x02\x12W\x80c\x08\x18\x12\xFC\x14a\x02'W[`\0\x80\xFD[a\x01\xFDa\x01\xF86`\x04a$XV[a\x04\x93V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x1Aa\x04\xE5V[`@Qa\x02\t\x91\x90a$\xC5V[a\x02:a\x0256`\x04a$\xD8V[a\x05wV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\tV[a\x02ea\x02`6`\x04a%\rV[a\x05\x9EV[\0[a\x02ea\x02u6`\x04a%7V[a\x06\xB8V[a\x02\x82a\x06\xE9V[`@Q\x90\x81R` \x01a\x02\tV[a\x02\x82a\x02\x9E6`\x04a%\rV[a\x06\xF8V[a\x02ea\x02\xB16`\x04a%7V[a\x07\x89V[a\x02\x1Aa\x02\xC46`\x04a$\xD8V[a\x07\xA4V[a\x02\x1Aa\x08FV[a\x02:a\x02\xDF6`\x04a%sV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\t` R`@\x90 T\x16\x90V[a\x02ea\x03\x0B6`\x04a%sV[a\x08\xDEV[a\x02:a\x03\x1E6`\x04a$\xD8V[a\x08\xEDV[a\x02\x82a\x0316`\x04a%sV[a\tMV[a\x02ea\t\xD3V[a\x02ea\x03L6`\x04a%\x8EV[a\t\xE7V[a\x02\x82a\x03_6`\x04a%sV[a\n4V[a\x02\x82a\nRV[a\x03ta\n]V[`@Qa\x02\t\x97\x96\x95\x94\x93\x92\x91\x90a&\x11V[`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\x02:V[a\x02\x82a\x03\xA66`\x04a$\xD8V[a\n\xE6V[a\x03\xB3a\x0B`V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\tV[a\x02\x1Aa\x0BkV[a\x02\x82a\x03\xE06`\x04a%sV[a\x0BzV[a\x02ea\x03\xF36`\x04a&\xA7V[a\x0B\x9BV[a\x02\x1Aa\x04\x066`\x04a$\xD8V[a\x0B\xA6V[a\x02ea\x04\x196`\x04a&\xF9V[a\x0B\xC3V[a\x02ea\x04,6`\x04a'\xD5V[a\x0B\xF5V[a\x02\x1Aa\x04?6`\x04a$\xD8V[a\r\"V[a\x01\xFDa\x04R6`\x04a(5V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x02ea\x04\x8E6`\x04a%sV[a\r\x96V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x04\xC4WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x04\xDFWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\0\x80Ta\x04\xF4\x90a(hV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05 \x90a(hV[\x80\x15a\x05mW\x80`\x1F\x10a\x05BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05mV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05PW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x05\x82\x82a\x0E\x0FV[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x05\xA9\x82a\x08\xEDV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x067WPa\x067\x813a\x04RV[a\x06\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\x06\x12V[a\x06\xB3\x83\x83a\x0EnV[PPPV[a\x06\xC23\x82a\x0E\xDCV[a\x06\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x12\x90a(\x9CV[a\x06\xB3\x83\x83\x83a\x0F[V[`\0a\x06\xF3a\x10\xC7V[\x90P\x90V[`\0a\x07\x02a\x0B`V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x07OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`d\x1B`D\x82\x01R`d\x01a\x06\x12V[a\x07ya\x07[\x83a\x11\xF2V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\n` R`@\x90 \x90a\x12[V[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x06\xB3\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x0B\xC3V[`\0\x81\x81R`\x0F` R`@\x90 \x80T``\x91\x90a\x07\xC1\x90a(hV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xED\x90a(hV[\x80\x15a\x08:W\x80`\x1F\x10a\x08\x0FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08:V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[``Ca\x08Qa\x0B`V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x08\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FVotes: broken clock mode\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\x12V[P`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R\x90V[3a\x08\xE9\x81\x83a\x13\x10V[PPV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x04\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x06\x12V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01Rh64\xB2\x107\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x06\x12V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\t\xDBa\x13\x82V[a\t\xE5`\0a\x13\xDCV[V[a\t\xEFa\x13\x82V[`\0a\t\xFA`\rT\x90V[\x90Pa\n\n`\r\x80T`\x01\x01\x90UV[`\0\x81\x81R`\x0E` R`@\x90 a\n#\x83\x85\x83a)7V[Pa\n.\x84\x82a\x14.V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x81 Ta\x04\xDFV[`\0a\x06\xF3`\rT\x90V[`\0``\x80\x82\x80\x80\x83a\n\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x07a\x14HV[a\n\xBC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08a\x14HV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0a\n\xF0a\x0B`V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x0B=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`d\x1B`D\x82\x01R`d\x01a\x06\x12V[a\x0BQa\x0BI\x83a\x11\xF2V[`\x0B\x90a\x12[V[`\x01`\x01`\xE0\x1B\x03\x16\x92\x91PPV[`\0a\x06\xF3Ca\x14\xF3V[```\x01\x80Ta\x04\xF4\x90a(hV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x81 a\x0BQ\x90a\x15ZV[a\x08\xE93\x83\x83a\x15\x94V[`\0\x81\x81R`\x0E` R`@\x90 \x80T``\x91\x90a\x07\xC1\x90a(hV[a\x0B\xCD3\x83a\x0E\xDCV[a\x0B\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x12\x90a(\x9CV[a\n.\x84\x84\x84\x84a\x16bV[\x83B\x11\x15a\x0CEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FVotes: signature expired\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\x12V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x0C\xBF\x90a\x0C\xB7\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x16\x95V[\x85\x85\x85a\x16\xC2V[\x90Pa\x0C\xCA\x81a\x16\xEAV[\x86\x14a\r\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsVotes: invalid nonce``\x1B`D\x82\x01R`d\x01a\x06\x12V[a\r\x19\x81\x88a\x13\x10V[PPPPPPPV[``a\r-\x82a\x0E\x0FV[`\0a\rD`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\rdW`@Q\x80` \x01`@R\x80`\0\x81RPa\r\x8FV[\x80a\rn\x84a\x17\x12V[`@Q` \x01a\r\x7F\x92\x91\x90a)\xF8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[a\r\x9Ea\x13\x82V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\x12V[a\x0E\x0C\x81a\x13\xDCV[PV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x0E\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x06\x12V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\x0E\xA3\x82a\x08\xEDV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0\x80a\x0E\xE8\x83a\x08\xEDV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0F/WP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x0FSWP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x0FH\x84a\x05wV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0Fn\x82a\x08\xEDV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x12\x90a*'V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0F\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x06\x12V[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x10\t\x82a\x08\xEDV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x12\x90a*'V[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4a\x06\xB3\x83\x83\x83`\x01a\x17\xA5V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x11 WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x11JWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x06\xF3`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x12WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\x12V[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x12\xB8W`\0a\x12v\x84a\x17\xB1V[a\x12\x80\x90\x85a*\x82V[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x12\xA8W\x80\x91Pa\x12\xB6V[a\x12\xB3\x81`\x01a*\x95V[\x92P[P[`\0a\x12\xC6\x87\x87\x85\x85a\x18\x99V[\x90P\x80\x15a\x13\x02Wa\x12\xEB\x87a\x12\xDD`\x01\x84a*\x82V[`\0\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x13\x05V[`\0[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\t` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x06\xB3\x81\x83a\x13}\x86a\x18\xF7V[a\x19\x02V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\x12V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[a\x08\xE9\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x1AnV[```\xFF\x83\x14a\x14bWa\x14[\x83a\x1A\xA1V[\x90Pa\x04\xDFV[\x81\x80Ta\x14n\x90a(hV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\x9A\x90a(hV[\x80\x15a\x14\xE7W\x80`\x1F\x10a\x14\xBCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xE7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xCAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x04\xDFV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\x12V[\x80T`\0\x90\x80\x15a\x15\x8BWa\x15t\x83a\x12\xDD`\x01\x84a*\x82V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\r\x8FV[`\0\x93\x92PPPV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x15\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\x12V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x16m\x84\x84\x84a\x0F[V[a\x16y\x84\x84\x84\x84a\x1A\xE0V[a\n.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x12\x90a*\xA8V[`\0a\x04\xDFa\x16\xA2a\x10\xC7V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x16\xD3\x87\x87\x87\x87a\x1B\xDEV[\x91P\x91Pa\x16\xE0\x81a\x1C\xA2V[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[```\0a\x17\x1F\x83a\x1D\xECV[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17?Wa\x17?a&\xE3V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17iW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x17sWP\x93\x92PPPV[a\n.\x84\x84\x84\x84a\x1E\xC4V[`\0\x81`\0\x03a\x17\xC3WP`\0\x91\x90PV[`\0`\x01a\x17\xD0\x84a\x1E\xD4V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x17\xE9Wa\x17\xE9a*\xFAV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x18\x01Wa\x18\x01a*\xFAV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x18\x19Wa\x18\x19a*\xFAV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x181Wa\x181a*\xFAV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x18IWa\x18Ia*\xFAV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x18aWa\x18aa*\xFAV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x18yWa\x18ya*\xFAV[\x04\x82\x01\x90\x1C\x90Pa\r\x8F\x81\x82\x85\x81a\x18\x93Wa\x18\x93a*\xFAV[\x04a\x1FhV[`\0[\x81\x83\x10\x15a\x18\xEFW`\0a\x18\xB0\x84\x84a\x1F~V[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x18\xDBW\x80\x92Pa\x18\xE9V[a\x18\xE6\x81`\x01a*\x95V[\x93P[Pa\x18\x9CV[P\x93\x92PPPV[`\0a\x04\xDF\x82a\tMV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x19$WP`\0\x81\x11[\x15a\x06\xB3W`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x19\xCCW`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\n` R`@\x81 \x81\x90a\x19g\x90a\x1F\x99a\x19b\x86a\x1F\xA5V[a \x0EV[`\x01`\x01`\xE0\x1B\x03\x16\x91P`\x01`\x01`\xE0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x19\xC1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x06\xB3W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\n` R`@\x81 \x81\x90a\x1A\x05\x90a Wa\x19b\x86a\x1F\xA5V[`\x01`\x01`\xE0\x1B\x03\x16\x91P`\x01`\x01`\xE0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x1A_\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x1Ax\x83\x83a cV[a\x1A\x85`\0\x84\x84\x84a\x1A\xE0V[a\x06\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x12\x90a*\xA8V[```\0a\x1A\xAE\x83a!\xF8V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x1B\xD6W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x1B$\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a+\x10V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x1B_WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1B\\\x91\x81\x01\x90a+MV[`\x01[a\x1B\xBCW=\x80\x80\x15a\x1B\x8DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1B\x92V[``\x91P[P\x80Q`\0\x03a\x1B\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x12\x90a*\xA8V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x0FSV[P`\x01a\x0FSV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1C\x15WP`\0\x90P`\x03a\x1C\x99V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1CiW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x92W`\0`\x01\x92P\x92PPa\x1C\x99V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a\x1C\xB6Wa\x1C\xB6a+jV[\x03a\x1C\xBEWPV[`\x01\x81`\x04\x81\x11\x15a\x1C\xD2Wa\x1C\xD2a+jV[\x03a\x1D\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\x12V[`\x02\x81`\x04\x81\x11\x15a\x1D3Wa\x1D3a+jV[\x03a\x1D\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x06\x12V[`\x03\x81`\x04\x81\x11\x15a\x1D\x94Wa\x1D\x94a+jV[\x03a\x0E\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x06\x12V[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x1E+Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x1EWWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x1EuWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x1E\x8DWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x1E\xA1Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x1E\xB3W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x04\xDFW`\x01\x01\x92\x91PPV[a\x1E\xCF\x84\x84\x83a\" V[a\n.V[`\0\x80`\x80\x83\x90\x1C\x15a\x1E\xE9W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a\x1E\xFBW`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a\x1F\rW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a\x1F\x1FW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a\x1F1W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a\x1FCW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a\x1FUW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x04\xDFW`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a\x1FwW\x81a\r\x8FV[P\x90\x91\x90PV[`\0a\x1F\x8D`\x02\x84\x84\x18a+\x80V[a\r\x8F\x90\x84\x84\x16a*\x95V[`\0a\r\x8F\x82\x84a+\xA2V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x12WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x06\x12V[`\0\x80a Ja ,a \x1Fa\x0B`V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x11\xF2V[a Ba 8\x88a\x15ZV[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a\"\x96V[\x91P\x91P[\x93P\x93\x91PPV[`\0a\r\x8F\x82\x84a+\xC9V[`\x01`\x01`\xA0\x1B\x03\x82\x16a \xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\x06\x12V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a!\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x06\x12V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a!\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x06\x12V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4a\x08\xE9`\0\x83\x83`\x01a\x17\xA5V[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x04\xDFW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a\"BWa\"?`\x0Ba Wa\x19b\x84a\x1F\xA5V[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a\"dWa\"a`\x0Ba\x1F\x99a\x19b\x84a\x1F\xA5V[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\t` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x06\xB3\x92\x91\x82\x16\x91\x16\x83a\x19\x02V[`\0\x80a J\x85\x85\x85\x82T`\0\x90\x81\x90\x80\x15a#\xE8W`\0a\"\xBD\x87a\x12\xDD`\x01\x85a*\x82V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a#?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01a\x06\x12V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a#\x88W\x84a#`\x88a\x12\xDD`\x01\x86a*\x82V[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua#\xD8V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16d\x01\0\0\0\0\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa O\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16d\x01\0\0\0\0\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a OV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0E\x0CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$jW`\0\x80\xFD[\x815a\r\x8F\x81a$BV[`\0[\x83\x81\x10\x15a$\x90W\x81\x81\x01Q\x83\x82\x01R` \x01a$xV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\xB1\x81` \x86\x01` \x86\x01a$uV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\r\x8F` \x83\x01\x84a$\x99V[`\0` \x82\x84\x03\x12\x15a$\xEAW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%\x08W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a% W`\0\x80\xFD[a%)\x83a$\xF1V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a%LW`\0\x80\xFD[a%U\x84a$\xF1V[\x92Pa%c` \x85\x01a$\xF1V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a%\x85W`\0\x80\xFD[a\r\x8F\x82a$\xF1V[`\0\x80`\0`@\x84\x86\x03\x12\x15a%\xA3W`\0\x80\xFD[a%\xAC\x84a$\xF1V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\xC9W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a%\xDDW`\0\x80\xFD[\x815\x81\x81\x11\x15a%\xECW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a%\xFEW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\xFF`\xF8\x1B\x88\x16\x81R`\0` `\xE0\x81\x84\x01Ra&1`\xE0\x84\x01\x8Aa$\x99V[\x83\x81\x03`@\x85\x01Ra&C\x81\x8Aa$\x99V[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R\x83\x87\x01\x92P\x90\x83\x01\x90`\0[\x81\x81\x10\x15a&\x95W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a&yV[P\x90\x9C\x9BPPPPPPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a&\xBAW`\0\x80\xFD[a&\xC3\x83a$\xF1V[\x91P` \x83\x015\x80\x15\x15\x81\x14a&\xD8W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a'\x0FW`\0\x80\xFD[a'\x18\x85a$\xF1V[\x93Pa'&` \x86\x01a$\xF1V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a'JW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a'^W`\0\x80\xFD[\x815\x81\x81\x11\x15a'pWa'pa&\xE3V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a'\x98Wa'\x98a&\xE3V[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a'\xB1W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a'\xEEW`\0\x80\xFD[a'\xF7\x87a$\xF1V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a(\x1BW`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a(HW`\0\x80\xFD[a(Q\x83a$\xF1V[\x91Pa(_` \x84\x01a$\xF1V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a(|W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x17\x0CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`-\x90\x82\x01R\x7FERC721: caller is not token owne`@\x82\x01Rl\x1C\x88\x1B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\x9A\x1B``\x82\x01R`\x80\x01\x90V[`\x1F\x82\x11\x15a\x06\xB3W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a)\x10WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a)/W\x82\x81U`\x01\x01a)\x1CV[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a)OWa)Oa&\xE3V[a)c\x83a)]\x83Ta(hV[\x83a(\xE9V[`\0`\x1F\x84\x11`\x01\x81\x14a)\x97W`\0\x85\x15a)\x7FWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua)\xF1V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a)\xC8W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a)\xA8V[P\x86\x82\x10\x15a)\xE5W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x83Qa*\n\x81\x84` \x88\x01a$uV[\x83Q\x90\x83\x01\x90a*\x1E\x81\x83` \x88\x01a$uV[\x01\x94\x93PPPPV[` \x80\x82R`%\x90\x82\x01R\x7FERC721: transfer from incorrect `@\x82\x01Rd7\xBB\xB72\xB9`\xD9\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\xDFWa\x04\xDFa*lV[\x80\x82\x01\x80\x82\x11\x15a\x04\xDFWa\x04\xDFa*lV[` \x80\x82R`2\x90\x82\x01R\x7FERC721: transfer to non ERC721Re`@\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a+C\x90\x83\x01\x84a$\x99V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a+_W`\0\x80\xFD[\x81Qa\r\x8F\x81a$BV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82a+\x9DWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01`\x01`\xE0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a+\xC2Wa+\xC2a*lV[P\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a+\xC2Wa+\xC2a*lV\xFE\xA2dipfsX\"\x12 W\xADA\x01{\xB4\xBD\xE7\x8A\xB1\xA6H8\xD47\xF84\xC0i\xE9\xBD\x12\x1C)\xFC\xFC3\x1F\xC7m\xDD\xFFdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static PROJECT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02\x8CW`\x005`\xE0\x1C\x80c{<q\xD3\x11a\x01ZW\x80c\xC2\x8B\xC2\xFA\x11a\0\xC1W\x80c\xEC\xE4\x0C\xC1\x11a\0zW\x80c\xEC\xE4\x0C\xC1\x14a\x08\xCDW\x80c\xF2:na\x14a\x08\xEDW\x80c\xF8\xCEV\n\x14a\t\x19W\x80c\xFB\x04\x92\x93\x14a\t9W\x80c\xFC\x0CTj\x14a\tYW\x80c\xFD\xC1\xF8\xD5\x14a\t\x8DW`\0\x80\xFD[\x80c\xC2\x8B\xC2\xFA\x14a\x07\xE0W\x80c\xC5\x90W\xE4\x14a\x07\xF3W\x80c\xDDN+\xA5\x14a\x08\x13W\x80c\xDE\xAA\xA7\xCC\x14a\x08YW\x80c\xEA\x02\x17\xCF\x14a\x08\x8DW\x80c\xEB\x90\x19\xD4\x14a\x08\xADW`\0\x80\xFD[\x80c\x9A\x02\x04&\x11a\x01\x13W\x80c\x9A\x02\x04&\x14a\x07\x11W\x80c\x9A\x80*m\x14a\x071W\x80c\xA7q:p\x14a\x07QW\x80c\xB5\x811\xB0\x14a\x07fW\x80c\xBC\x19|\x81\x14a\x07{W\x80c\xC0\x1F\x9E7\x14a\x07\xA7W`\0\x80\xFD[\x80c{<q\xD3\x14a\x06IW\x80c}^\x81\xE2\x14a\x06iW\x80c\x84\xB0\x19n\x14a\x06\x89W\x80c\x8Bd\xBB\x87\x14a\x06\xB1W\x80c\x91\xDD\xAD\xF4\x14a\x06\xD1W\x80c\x97\xC3\xD34\x14a\x06\xFDW`\0\x80\xFD[\x80c;\xCC\xF4\xFD\x11a\x01\xFEW\x80cTO\xFC\x9C\x11a\x01\xB7W\x80cTO\xFC\x9C\x14a\x05JW\x80cT\xFDMP\x14a\x05\x9FW\x80cVx\x13\x88\x14a\x05\xC9W\x80c_9\x8A\x14\x14a\x05\xE9W\x80c`\xC4$\x7F\x14a\x06\tW\x80cp\xB0\xF6`\x14a\x06)W`\0\x80\xFD[\x80c;\xCC\xF4\xFD\x14a\x04\x88W\x80c>OI\xE6\x14a\x04\xA8W\x80cC\x85\x962\x14a\x04\xD5W\x80cC\x9F\xAB\x91\x14a\x04\xF5W\x80cE!\x15\xD6\x14a\x05\x15W\x80cK\xF5\xD7\xE9\x14a\x055W`\0\x80\xFD[\x80c\x144\x89\xD0\x11a\x02PW\x80c\x144\x89\xD0\x14a\x03]W\x80c\x15\x0Bz\x02\x14a\x03\xB2W\x80c&V\"}\x14a\x03\xF6W\x80c-c\xF6\x93\x14a\x04\tW\x80c/\xE3\xE2a\x14a\x04?W\x80c92\xAB\xB1\x14a\x04sW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02\xA3W\x80c\x02\xA2Q\xA3\x14a\x02\xD8W\x80c\x03B\x01\x81\x14a\x02\xFBW\x80c\x06\xF3\xF9\xE6\x14a\x03\x1BW\x80c\x06\xFD\xDE\x03\x14a\x03;W`\0\x80\xFD[6a\x02\x9EW\0[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x02\xAFW`\0\x80\xFD[Pa\x02\xC3a\x02\xBE6`\x04a5\xFAV[a\t\xADV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xE4W`\0\x80\xFD[Pa\x02\xEDa\nJV[`@Q\x90\x81R` \x01a\x02\xCFV[4\x80\x15a\x03\x07W`\0\x80\xFD[Pa\x02\xEDa\x03\x166`\x04a7HV[a\nZV[4\x80\x15a\x03'W`\0\x80\xFD[Pa\x02\x9Ca\x0366`\x04a7\xEEV[a\x0BRV[4\x80\x15a\x03GW`\0\x80\xFD[Pa\x03Pa\x0B\x94V[`@Qa\x02\xCF\x91\x90a8WV[4\x80\x15a\x03iW`\0\x80\xFD[Pa\x03\x9Aa\x03x6`\x04a7\xEEV[`\0\x90\x81R`\x03` R`@\x90 T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xCFV[4\x80\x15a\x03\xBEW`\0\x80\xFD[Pa\x03\xDDa\x03\xCD6`\x04a8\x81V[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\xCFV[a\x02\xEDa\x04\x046`\x04a:RV[a\x0C&V[4\x80\x15a\x04\x15W`\0\x80\xFD[Pa\x02\xEDa\x04$6`\x04a7\xEEV[`\0\x90\x81R`\x03` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[4\x80\x15a\x04KW`\0\x80\xFD[Pa\x02\xED\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x81V[4\x80\x15a\x04\x7FW`\0\x80\xFD[Pa\x02\xEDa\rRV[4\x80\x15a\x04\x94W`\0\x80\xFD[Pa\x02\xEDa\x04\xA36`\x04a:\xE1V[a\r]V[4\x80\x15a\x04\xB4W`\0\x80\xFD[Pa\x04\xC8a\x04\xC36`\x04a7\xEEV[a\r\xD3V[`@Qa\x02\xCF\x91\x90a;EV[4\x80\x15a\x04\xE1W`\0\x80\xFD[Pa\x02\xC3a\x04\xF06`\x04a;mV[a\x0F\x13V[4\x80\x15a\x05\x01W`\0\x80\xFD[Pa\x02\x9Ca\x05\x106`\x04a;\x99V[a\x0FCV[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x02\xEDa\x0506`\x04a:RV[a\x10\x19V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x03Pa\x11 V[4\x80\x15a\x05VW`\0\x80\xFD[Pa\x05\x84a\x05e6`\x04a7\xEEV[`\0\x90\x81R`\t` R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xCFV[4\x80\x15a\x05\xABW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01Ra\x03PV[4\x80\x15a\x05\xD5W`\0\x80\xFD[Pa\x02\xEDa\x05\xE46`\x04a;\xDAV[a\x11\xE2V[4\x80\x15a\x05\xF5W`\0\x80\xFD[Pa\x02\xEDa\x06\x046`\x04a;\xFDV[a\x12\x0BV[4\x80\x15a\x06\x15W`\0\x80\xFD[Pa\x02\xEDa\x06$6`\x04a7\xEEV[a\x12UV[4\x80\x15a\x065W`\0\x80\xFD[Pa\x02\x9Ca\x06D6`\x04a7\xEEV[a\x13\x06V[4\x80\x15a\x06UW`\0\x80\xFD[Pa\x02\xEDa\x06d6`\x04a<\x80V[a\x13EV[4\x80\x15a\x06uW`\0\x80\xFD[Pa\x02\xEDa\x06\x846`\x04a<\xD9V[a\x13\x8DV[4\x80\x15a\x06\x95W`\0\x80\xFD[Pa\x06\x9Ea\x17fV[`@Qa\x02\xCF\x97\x96\x95\x94\x93\x92\x91\x90a=\xC8V[4\x80\x15a\x06\xBDW`\0\x80\xFD[P`\x0CTa\x03\x9A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xDDW`\0\x80\xFD[Pa\x06\xE6a\x17\xEEV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xCFV[4\x80\x15a\x07\tW`\0\x80\xFD[P`da\x02\xEDV[4\x80\x15a\x07\x1DW`\0\x80\xFD[Pa\x02\x9Ca\x07,6`\x04a7\xEEV[a\x18wV[4\x80\x15a\x07=W`\0\x80\xFD[Pa\x02\xEDa\x07L6`\x04a>*V[a\x1ArV[4\x80\x15a\x07]W`\0\x80\xFD[Pa\x02\xEDa\x1A\x89V[4\x80\x15a\x07rW`\0\x80\xFD[Pa\x02\xEDa\x1A\xB3V[4\x80\x15a\x07\x87W`\0\x80\xFD[Pa\x03\xDDa\x07\x966`\x04a>\x80V[c\xBC\x19|\x81`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x07\xB3W`\0\x80\xFD[Pa\x02\xEDa\x07\xC26`\x04a7\xEEV[`\0\x90\x81R`\x03` R`@\x90 `\x01\x01T`\x01`\x01`@\x1B\x03\x16\x90V[a\x02\x9Ca\x07\xEE6`\x04a?\x0FV[a\x1A\xBEV[4\x80\x15a\x07\xFFW`\0\x80\xFD[Pa\x02\xEDa\x08\x0E6`\x04a:RV[a\x1B\x85V[4\x80\x15a\x08\x1FW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R` \x80\x82R\x7Fsupport=bravo&quorum=for,abstain\x90\x82\x01Ra\x03PV[4\x80\x15a\x08eW`\0\x80\xFD[Pa\x02\xED\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F\x81V[4\x80\x15a\x08\x99W`\0\x80\xFD[Pa\x02\x9Ca\x08\xA86`\x04a7\xEEV[a\x1B\xBFV[4\x80\x15a\x08\xB9W`\0\x80\xFD[Pa\x02\xEDa\x08\xC86`\x04a?PV[a\x1B\xFEV[4\x80\x15a\x08\xD9W`\0\x80\xFD[Pa\x02\x9Ca\x08\xE86`\x04a7\xEEV[a\x1C\x1FV[4\x80\x15a\x08\xF9W`\0\x80\xFD[Pa\x03\xDDa\t\x086`\x04a?zV[c\xF2:na`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\t%W`\0\x80\xFD[Pa\x02\xEDa\t46`\x04a7\xEEV[a\x1C^V[4\x80\x15a\tEW`\0\x80\xFD[Pa\x03Pa\tT6`\x04a7\xEEV[a\x1CiV[4\x80\x15a\teW`\0\x80\xFD[Pa\x03\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\t\x99W`\0\x80\xFD[Pa\x02\xEDa\t\xA86`\x04a?\xDEV[a\x1D\x0BV[`\0c(\x8A\xCE\x03`\xE1\x1Bc\x18\xDFt?`\xE3\x1Bc\xBF&\xD8\x97`\xE0\x1Bcy\xDDyo`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x82\x14\x80a\t\xF3WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x81\x16\x90\x82\x16\x14[\x80a\n\nWP`\x01`\x01`\xE0\x1B\x03\x19\x86\x81\x16\x90\x85\x16\x14[\x80a\n%WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x16c\x02q\x18\x97`\xE5\x1B\x14[\x80a\n@WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x87\x16\x14[\x96\x95PPPPPPV[`\0a\nU`\x07T\x90V[\x90P\x90V[`\0\x80a\n\xFEa\n\xF6\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x8C\x8C\x8C\x8C`@Qa\n\x96\x92\x91\x90a@=V[`@Q\x80\x91\x03\x90 \x8B\x80Q\x90` \x01 `@Q` \x01a\n\xDB\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R`\xFF\x91\x90\x91\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1E\xDAV[\x86\x86\x86a\x1F\x07V[\x90Pa\x0BD\x8A\x82\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8D\x92Pa\x1F%\x91PPV[\x9A\x99PPPPPPPPPPV[30\x14a\x0BqW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x0B\x88V[\x80a\x0B\x81`\x04a zV[\x03a\x0BvWP[a\x0B\x91\x81a \xF9V[PV[```\x02\x80Ta\x0B\xA3\x90a@\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xCF\x90a@\x84V[\x80\x15a\x0C\x1CW\x80`\x1F\x10a\x0B\xF1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x1CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xFFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80a\x0C5\x86\x86\x86\x86a\x1B\x85V[\x90P`\0a\x0CB\x82a\r\xD3V[\x90P`\x04\x81`\x07\x81\x11\x15a\x0CXWa\x0CXa;/V[\x14\x80a\x0CuWP`\x05\x81`\x07\x81\x11\x15a\x0CsWa\x0Csa;/V[\x14[a\x0C\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal not successfu`D\x82\x01R`\x1B`\xFA\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0\x82\x81R`\x03` R`@\x90\x81\x90 `\x02\x01\x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fq*\xE18?y\xAC\x85?\x8D\x88!Sw\x8E\x02`\xEF\x8F\x03\xB5\x04\xE2\x86n\x05\x93\xE0M+)\x1F\x90a\r\x19\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1a\r.\x82\x88\x88\x88\x88a\"gV[a\r;\x82\x88\x88\x88\x88a\"\xF2V[a\rH\x82\x88\x88\x88\x88a\"gV[P\x95\x94PPPPPV[`\0a\nU`\x06T\x90V[`@\x80Q\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F` \x82\x01R\x90\x81\x01\x86\x90R`\xFF\x85\x16``\x82\x01R`\0\x90\x81\x90a\r\xAB\x90a\n\xF6\x90`\x80\x01a\n\xDBV[\x90Pa\r\xC8\x87\x82\x88`@Q\x80` \x01`@R\x80`\0\x81RPa#\xE8V[\x97\x96PPPPPPPV[`\0\x81\x81R`\x03` R`@\x81 `\x02\x81\x01T`\xFF\x16\x15a\r\xF7WP`\x07\x92\x91PPV[`\x02\x81\x01Ta\x01\0\x90\x04`\xFF\x16\x15a\x0E\x12WP`\x02\x92\x91PPV[`\0\x83\x81R`\x03` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90\x03a\x0EzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: unknown proposal id\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0a\x0E\x84a\x17\xEEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x82\x10a\x0E\xA0WP`\0\x94\x93PPPPV[`\0\x85\x81R`\x03` R`@\x90 `\x01\x01T`\x01`\x01`@\x1B\x03\x16\x81\x81\x10a\x0E\xCEWP`\x01\x95\x94PPPPPV[a\x0E\xD7\x86a$\x14V[\x80\x15a\x0E\xF6WP`\0\x86\x81R`\t` R`@\x90 \x80T`\x01\x90\x91\x01T\x11[\x15a\x0F\x07WP`\x04\x95\x94PPPPPV[P`\x03\x95\x94PPPPPV[`\0\x82\x81R`\t` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R`\x03\x01\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[`\x0CT`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0F\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FProject: already initialized\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\x0C\x80T`\xFF`\xA0\x1B\x19\x81\x16`\x01`\xA0\x1B\x17\x90\x91U`@Qc<-\xD8\x1F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx[\xB0>\x90a\x0F\xE3\x903\x90\x86\x90\x86\x90`\x04\x01a@\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x11W=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x80a\x10(\x86\x86\x86\x86a\x1B\x85V[\x90P`\0a\x105\x82a\r\xD3V[`\x07\x81\x11\x15a\x10FWa\x10Fa;/V[\x14a\x10\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FGovernor: too late to cancel\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FGovernor: only proposer can canc`D\x82\x01Ra\x19[`\xF2\x1B`d\x82\x01R`\x84\x01a\x02\x93V[a\n@\x86\x86\x86\x86a$`V[``\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cK\xF5\xD7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x11\xA1WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\x9E\x91\x90\x81\x01\x90a@\xFEV[`\x01[a\x11\xDDWP`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R\x90V[\x91\x90PV[`\0\x803\x90Pa\x12\x03\x84\x82\x85`@Q\x80` \x01`@R\x80`\0\x81RPa#\xE8V[\x94\x93PPPPV[`\0\x803\x90Pa\r\xC8\x87\x82\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92Pa\x1F%\x91PPV[`\x0BT`\0\x90\x80\x82\x03a\x12lWPP`\nT\x91\x90PV[`\0`\x0Ba\x12{`\x01\x84aA\x81V[\x81T\x81\x10a\x12\x8BWa\x12\x8BaA\x94V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`\x01` \x1B\x90\x91\x04`\x01`\x01`\xE0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R\x91P\x84\x10a\x12\xE1W` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x12\xF5a\x12\xED\x85a%~V[`\x0B\x90a%\xE7V[`\x01`\x01`\xE0\x1B\x03\x16\x94\x93PPPPV[30\x14a\x13%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x13<V[\x80a\x135`\x04a zV[\x03a\x13*WP[a\x0B\x91\x81a&\x9AV[`\0\x803\x90Pa\n@\x86\x82\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa#\xE8\x92PPPV[`\x003a\x13\x9A\x81\x84a&\xDBV[a\x13\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: proposer restricted\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0a\x13\xF0a\x17\xEEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x14\x02a\x1A\xB3V[a\x14\x11\x83a\x08\xC8`\x01\x85aA\x81V[\x10\x15a\x14yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FGovernor: proposer votes below p`D\x82\x01Rp\x1C\x9B\xDC\x1B\xDC\xD8[\x08\x1D\x1A\x1C\x99\\\xDA\x1B\xDB\x19`z\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0a\x14\x8E\x88\x88\x88\x88\x80Q\x90` \x01 a\x1B\x85V[\x90P\x86Q\x88Q\x14a\x14\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90aA\xAAV[\x85Q\x88Q\x14a\x14\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90aA\xAAV[`\0\x88Q\x11a\x15#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FGovernor: empty proposal\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15a\x15\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal already exist`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0a\x15\x9Ca\rRV[a\x15\xA6\x90\x84aA\xEBV[\x90P`\0a\x15\xB2a\nJV[a\x15\xBC\x90\x83aA\xEBV[\x90P`@Q\x80`\xE0\x01`@R\x80a\x15\xD2\x84a'\xCCV[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R`\0`@\x82\x01R``\x01a\x15\xFF\x83a'\xCCV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\0` \x80\x84\x01\x82\x90R`@\x80\x85\x01\x83\x90R``\x94\x85\x01\x83\x90R\x88\x83R`\x03\x82R\x91\x82\x90 \x85Q\x81T\x92\x87\x01Q\x87\x85\x01Q\x91\x86\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84\x02\x17`\x01`\x01`\xE0\x1B\x03\x16`\x01`\xE0\x1B`\xE0\x92\x90\x92\x1C\x91\x90\x91\x02\x17\x81U\x93\x85\x01Q`\x80\x86\x01Q\x90\x84\x16\x92\x1C\x02\x17`\x01\x83\x01U`\xA0\x83\x01Q`\x02\x90\x92\x01\x80T`\xC0\x90\x94\x01Qa\xFF\xFF\x19\x90\x94\x16\x92\x15\x15a\xFF\0\x19\x16\x92\x90\x92\x17a\x01\0\x93\x15\x15\x93\x90\x93\x02\x92\x90\x92\x17\x90U\x8AQ\x7F}\x84\xA6&:\xE0\xD9\x8D3)\xBD{F\xBBN\x8Do\x98\xCD5\xA7\xAD\xB4\\'L\x8B\x7F\xD5\xEB\xD5\xE0\x91\x85\x91\x88\x91\x8E\x91\x8E\x91\x81\x11\x15a\x17\x03Wa\x17\x03a6}V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x176W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17!W\x90P[P\x8D\x88\x88\x8F`@Qa\x17P\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aB\x8CV[`@Q\x80\x91\x03\x90\xA1P\x90\x98\x97PPPPPPPPV[`\0``\x80\x82\x80\x80\x83a\x17\x99\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a(4V[a\x17\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a(4V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x91\xDD\xAD\xF4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x18jWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x18g\x91\x81\x01\x90aCcV[`\x01[a\x11\xDDWa\nUCa(\xDFV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x18\xCDWa\x18\xCDaA\x94V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x19\x1FWa\x19\x1FaA\x94V[` \x02` \x01\x01\x81\x81RPP`\x003`\r`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q`$\x01a\x19R\x92\x91\x90aC\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c<-\xD8\x1F`\xE1\x1B\x17\x90R\x81Q`\x01\x80\x82R\x81\x84\x01\x90\x93R\x92\x93P`\0\x92\x91\x90\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\x95W\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x19\xC0Wa\x19\xC0aA\x94V[` \x02` \x01\x01\x81\x90RPa\x10\x11\x84\x84\x83`\x0E`\0\x8A\x81R` \x01\x90\x81R` \x01`\0 \x80Ta\x19\xEF\x90a@\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x1B\x90a@\x84V[\x80\x15a\x1AhW\x80`\x1F\x10a\x1A=Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1AhV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1AKW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x13\x8DV[`\0a\x1A\x7F\x84\x84\x84a)FV[\x90P[\x93\x92PPPV[`\x0BT`\0\x90\x15a\x1A\xACWa\x1A\x9E`\x0Ba)\xDCV[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[P`\nT\x90V[`\0a\nU`\x08T\x90V[30\x14a\x1A\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x1A\xF4V[\x80a\x1A\xED`\x04a zV[\x03a\x1A\xE2WP[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85\x85`@Qa\x1B\x12\x92\x91\x90a@=V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1BOW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1BTV[``\x91P[P\x91P\x91Pa\x1B|\x82\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01aF\x87`(\x919a*\x15V[PPPPPPPV[`\0\x84\x84\x84\x84`@Q` \x01a\x1B\x9E\x94\x93\x92\x91\x90aD&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[30\x14a\x1B\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x1B\xF5V[\x80a\x1B\xEE`\x04a zV[\x03a\x1B\xE3WP[a\x0B\x91\x81a*.V[`\0a\x1A\x82\x83\x83a\x1C\x1A`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a)FV[30\x14a\x1C>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x90a@MV[a\x1CUV[\x80a\x1CN`\x04a zV[\x03a\x1CCWP[a\x0B\x91\x81a*\xCFV[`\0a\x0F=\x82a+\x10V[`\0\x81\x81R`\r` R`@\x90 \x80T``\x91\x90a\x1C\x86\x90a@\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xB2\x90a@\x84V[\x80\x15a\x1C\xFFW\x80`\x1F\x10a\x1C\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x82\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x1DdWa\x1DdaA\x94V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x1D\xB6Wa\x1D\xB6aA\x94V[` \x02` \x01\x01\x81\x81RPP`\x003\x88\x88`@Q`$\x01a\x1D\xD9\x93\x92\x91\x90a@\xBEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c<-\xD8\x1F`\xE1\x1B\x17\x90R\x81Q`\x01\x80\x82R\x81\x84\x01\x90\x93R\x92\x93P`\0\x92\x91\x90\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\x1CW\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x1EGWa\x1EGaA\x94V[` \x02` \x01\x01\x81\x90RP`\0a\x1E\x96\x85\x85\x84\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x13\x8D\x92PPPV[`\0\x81\x81R`\r` R`@\x90 \x90\x91Pa\x1E\xB2\x8A\x8C\x83aD\xBCV[P`\0\x81\x81R`\x0E` R`@\x90 a\x1E\xCC\x88\x8A\x83aD\xBCV[P\x99\x98PPPPPPPPPV[`\0a\x0F=a\x1E\xE7a+\xBAV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x1F\x18\x87\x87\x87\x87a,\xE5V[\x91P\x91Pa\rH\x81a-\xA9V[`\0\x85\x81R`\x03` R`@\x81 `\x01a\x1F>\x88a\r\xD3V[`\x07\x81\x11\x15a\x1FOWa\x1FOa;/V[\x14a\x1F\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FGovernor: vote not currently act`D\x82\x01Rbive`\xE8\x1B`d\x82\x01R`\x84\x01a\x02\x93V[\x80T`\0\x90a\x1F\xC2\x90\x88\x90`\x01`\x01`@\x1B\x03\x16\x86a)FV[\x90Pa\x1F\xD1\x88\x88\x88\x84\x88a.\xF3V[\x83Q`\0\x03a &W\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB8\xE18\x88}\n\xA1;\xABD~\x82\xDE\x9D\\\x17w\x04\x1E\xCD!\xCA6\xBA\x82O\xF1\xE6\xC0}\xDD\xA4\x89\x88\x84\x89`@Qa \x19\x94\x93\x92\x91\x90aE{V[`@Q\x80\x91\x03\x90\xA2a\r\xC8V[\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE2\xBA\xBF\xBA\xC5\x88\x9Ap\x9Bc\xBB\x7FY\x8B2N\x08\xBCZO\xB9\xECd\x7F\xB3\xCB\xC9\xEC\x07\xEB\x87\x12\x89\x88\x84\x89\x89`@Qa g\x95\x94\x93\x92\x91\x90aE\xA3V[`@Q\x80\x91\x03\x90\xA2\x97\x96PPPPPPPV[`\0a \x95\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a \xB3W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`d\x81\x11\x15a!|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x02\x93V[`\0a!\x86a\x1A\x89V[\x90P\x80\x15\x80\x15\x90a!\x97WP`\x0BT\x15[\x15a!\xFBW`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R`\x0B\x90` \x81\x01a!\xBA\x84a0mV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[a\"(a\"\x16a\"\ta\x17\xEEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a%~V[a\"\x1F\x84a0mV[`\x0B\x91\x90a0\xD6V[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\"\xEBV[\x84Q\x81\x10\x15a\x10\x11W0`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\"\x91Wa\"\x91aA\x94V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\"\xDBWa\"\xDB\x83\x82\x81Q\x81\x10a\"\xBCWa\"\xBCaA\x94V[` \x02` \x01\x01Q\x80Q\x90` \x01 `\x04a0\xF1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\"\xE4\x81aE\xE9V[\x90Pa\"lV[PPPPPV[`\0`@Q\x80``\x01`@R\x80`'\x81R` \x01aF\xAF`'\x919\x90P`\0[\x85Q\x81\x10\x15a\x1B|W`\0\x80\x87\x83\x81Q\x81\x10a#0Wa#0aA\x94V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x87\x84\x81Q\x81\x10a#SWa#SaA\x94V[` \x02` \x01\x01Q\x87\x85\x81Q\x81\x10a#mWa#maA\x94V[` \x02` \x01\x01Q`@Qa#\x82\x91\x90aF\x02V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a#\xBFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a#\xC4V[``\x91P[P\x91P\x91Pa#\xD4\x82\x82\x86a*\x15V[PPP\x80a#\xE1\x90aE\xE9V[\x90Pa#\x12V[`\0a$\x0B\x85\x85\x85\x85a$\x06`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a\x1F%V[\x95\x94PPPPPV[`\0\x81\x81R`\t` R`@\x81 `\x02\x81\x01T`\x01\x82\x01Ta$6\x91\x90aA\xEBV[`\0\x84\x81R`\x03` R`@\x90 Ta$W\x90`\x01`\x01`@\x1B\x03\x16a\x1C^V[\x11\x15\x93\x92PPPV[`\0\x80a$o\x86\x86\x86\x86a\x1B\x85V[\x90P`\0a$|\x82a\r\xD3V[\x90P`\x02\x81`\x07\x81\x11\x15a$\x92Wa$\x92a;/V[\x14\x15\x80\x15a$\xB2WP`\x06\x81`\x07\x81\x11\x15a$\xAFWa$\xAFa;/V[\x14\x15[\x80\x15a$\xD0WP`\x07\x81`\x07\x81\x11\x15a$\xCDWa$\xCDa;/V[\x14\x15[a%\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: proposal not active\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\0\x82\x81R`\x03` R`@\x90\x81\x90 `\x02\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UQ\x7Fx\x9C\xF5[\xE9\x80s\x9D\xAD\x1D\x06\x99\xB9;X\xE8\x06\xB5\x1C\x9D\x96a\x9B\xFA\x8F\xE0\xA2\x8A\xBA\xA7\xB3\x0C\x90a%l\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1P\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a%\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\x93V[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a&DW`\0a&\x02\x84a1-V[a&\x0C\x90\x85aA\x81V[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a&4W\x80\x91Pa&BV[a&?\x81`\x01aA\xEBV[\x92P[P[`\0a&R\x87\x87\x85\x85a2\x15V[\x90P\x80\x15a&\x8DWa&w\x87a&i`\x01\x84aA\x81V[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\r\xC8V[`\0\x97\x96PPPPPPPV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[\x80Q`\0\x90`4\x81\x10\x15a&\xF3W`\x01\x91PPa\x0F=V[\x82\x81\x01`\x13\x19\x01Q`\x01`\x01`\xA0\x1B\x03\x19\x81\x16k\x04n\x0EM\xEE\r\xEEl\xAEG\xA6\x0F`\xA3\x1B\x14a'&W`\x01\x92PPPa\x0F=V[`\0\x80a'4`(\x85aA\x81V[\x90P[\x83\x81\x10\x15a'\xABW`\0\x80a'k\x88\x84\x81Q\x81\x10a'WWa'WaA\x94V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16a2sV[\x91P\x91P\x81a'\x83W`\x01\x96PPPPPPPa\x0F=V[\x80`\xFF\x16`\x04\x85`\x01`\x01`\xA0\x1B\x03\x16\x90\x1B\x17\x93PPP\x80a'\xA4\x90aE\xE9V[\x90Pa'7V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x93PPPP\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a%\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 6`D\x82\x01Re4 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\x93V[```\xFF\x83\x14a(NWa(G\x83a3\x05V[\x90Pa\x0F=V[\x81\x80Ta(Z\x90a@\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\x86\x90a@\x84V[\x80\x15a(\xD3W\x80`\x1F\x10a(\xA8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xD3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x0F=V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c:F\xB1\xA8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x7F\x91\x90aF\x1EV[\x80T`\0\x90\x80\x15a*\x0CWa)\xF6\x83a&i`\x01\x84aA\x81V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x1A\x82V[`\0\x93\x92PPPV[``\x83\x15a*$WP\x81a\x1A\x82V[a\x1A\x82\x83\x83a3DV[`\0\x81\x11a*\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\x07T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x07UV[`\x08T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x08UV[`\0`da+\x1D\x83a\x12UV[`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8ES\x9E\x8C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xA6\x91\x90aF\x1EV[a+\xB0\x91\x90aF7V[a\x0F=\x91\x90aFdV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a,\x13WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a,=WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\nU`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a-\x1CWP`\0\x90P`\x03a-\xA0V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a-pW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a-\x99W`\0`\x01\x92P\x92PPa-\xA0V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a-\xBDWa-\xBDa;/V[\x03a-\xC5WPV[`\x01\x81`\x04\x81\x11\x15a-\xD9Wa-\xD9a;/V[\x03a.&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[`\x02\x81`\x04\x81\x11\x15a.:Wa.:a;/V[\x03a.\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02\x93V[`\x03\x81`\x04\x81\x11\x15a.\x9BWa.\x9Ba;/V[\x03a\x0B\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0\x85\x81R`\t` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R`\x03\x81\x01\x90\x92R\x90\x91 T`\xFF\x16\x15a/{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorVotingSimple: vote alrea`D\x82\x01Rf\x19\x1EH\x18\xD8\\\xDD`\xCA\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\xFF\x84\x16a/\xC7W\x82\x81`\0\x01`\0\x82\x82Ta/\xBC\x91\x90aA\xEBV[\x90\x91UPa\x10\x11\x90PV[`\0\x19`\xFF\x85\x16\x01a/\xE7W\x82\x81`\x01\x01`\0\x82\x82Ta/\xBC\x91\x90aA\xEBV[`\x01\x19`\xFF\x85\x16\x01a0\x07W\x82\x81`\x02\x01`\0\x82\x82Ta/\xBC\x91\x90aA\xEBV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FGovernorVotingSimple: invalid va`D\x82\x01Rtlue for enum VoteType`X\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a%\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x93V[`\0\x80a0\xE4\x85\x85\x85a3nV[\x91P\x91P[\x93P\x93\x91PPV[\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B`\0\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[`\0\x81`\0\x03a1?WP`\0\x91\x90PV[`\0`\x01a1L\x84a5\rV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a1eWa1eaFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1}Wa1}aFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\x95Wa1\x95aFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\xADWa1\xADaFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\xC5Wa1\xC5aFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\xDDWa1\xDDaFNV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a1\xF5Wa1\xF5aFNV[\x04\x82\x01\x90\x1C\x90Pa\x1A\x82\x81\x82\x85\x81a2\x0FWa2\x0FaFNV[\x04a5\xA1V[`\0[\x81\x83\x10\x15a2kW`\0a2,\x84\x84a5\xB7V[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a2WW\x80\x92Pa2eV[a2b\x81`\x01aA\xEBV[\x93P[Pa2\x18V[P\x93\x92PPPV[`\0\x80`\xF8\x83\x90\x1C`/\x81\x11\x80\x15a2\x8EWP`:\x81`\xFF\x16\x10[\x15a2\xA3W`\x01\x94`/\x19\x90\x91\x01\x93P\x91PPV[\x80`\xFF\x16`@\x10\x80\x15a2\xB9WP`G\x81`\xFF\x16\x10[\x15a2\xCEW`\x01\x94`6\x19\x90\x91\x01\x93P\x91PPV[\x80`\xFF\x16``\x10\x80\x15a2\xE4WP`g\x81`\xFF\x16\x10[\x15a2\xF9W`\x01\x94`V\x19\x90\x91\x01\x93P\x91PPV[P`\0\x93\x84\x93P\x91PPV[```\0a3\x12\x83a5\xD2V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[\x81Q\x15a3TW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x93\x91\x90a8WV[\x82T`\0\x90\x81\x90\x80\x15a4\xB4W`\0a3\x8C\x87a&i`\x01\x85aA\x81V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a4\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x93V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a4UW\x84a4.\x88a&i`\x01\x86aA\x81V[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua4\xA4V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01` \x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa0\xE9\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01` \x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a0\xE9V[`\0\x80`\x80\x83\x90\x1C\x15a5\"W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a54W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a5FW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a5XW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a5jW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a5|W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a5\x8EW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x0F=W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a5\xB0W\x81a\x1A\x82V[P\x90\x91\x90PV[`\0a5\xC6`\x02\x84\x84\x18aFdV[a\x1A\x82\x90\x84\x84\x16aA\xEBV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x0F=W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a6\x0CW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1A\x82W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a\x11\xDDW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a6GW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a6^W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a6vW`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a6\xBBWa6\xBBa6}V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a6\xDCWa6\xDCa6}V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a6\xFDa6\xF8\x84a6\xC3V[a6\x93V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a7\x11W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a79W`\0\x80\xFD[a\x1A\x82\x83\x835` \x85\x01a6\xEAV[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15a7dW`\0\x80\xFD[\x885\x97Pa7t` \x8A\x01a6$V[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7\x90W`\0\x80\xFD[a7\x9C\x8C\x83\x8D\x01a65V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a7\xB5W`\0\x80\xFD[Pa7\xC2\x8B\x82\x8C\x01a7(V[\x94PPa7\xD1`\x80\x8A\x01a6$V[\x92P`\xA0\x89\x015\x91P`\xC0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a8\0W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a8\"W\x81\x81\x01Q\x83\x82\x01R` \x01a8\nV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra8C\x81` \x86\x01` \x86\x01a8\x07V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x1A\x82` \x83\x01\x84a8+V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\xDDW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a8\x97W`\0\x80\xFD[a8\xA0\x85a8jV[\x93Pa8\xAE` \x86\x01a8jV[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xD0W`\0\x80\xFD[a8\xDC\x87\x82\x88\x01a7(V[\x91PP\x92\x95\x91\x94P\x92PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a9\x01Wa9\x01a6}V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a9\x1CW`\0\x80\xFD[\x815` a9,a6\xF8\x83a8\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a9KW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a9mWa9`\x81a8jV[\x83R\x91\x83\x01\x91\x83\x01a9OV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a9\x89W`\0\x80\xFD[\x815` a9\x99a6\xF8\x83a8\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a9\xB8W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a9mW\x805\x83R\x91\x83\x01\x91\x83\x01a9\xBCV[`\0\x82`\x1F\x83\x01\x12a9\xE4W`\0\x80\xFD[\x815` a9\xF4a6\xF8\x83a8\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a:\x13W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a9mW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a:6W`\0\x80\x81\xFD[a:D\x89\x86\x83\x8B\x01\x01a7(V[\x84RP\x91\x83\x01\x91\x83\x01a:\x17V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:hW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\x7FW`\0\x80\xFD[a:\x8B\x88\x83\x89\x01a9\x0BV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a:\xA1W`\0\x80\xFD[a:\xAD\x88\x83\x89\x01a9xV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a:\xC3W`\0\x80\xFD[Pa:\xD0\x87\x82\x88\x01a9\xD3V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a:\xF9W`\0\x80\xFD[\x855\x94Pa;\t` \x87\x01a6$V[\x93Pa;\x17`@\x87\x01a6$V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x08\x83\x10a;gWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`@\x83\x85\x03\x12\x15a;\x80W`\0\x80\xFD[\x825\x91Pa;\x90` \x84\x01a8jV[\x90P\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a;\xACW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a;\xC2W`\0\x80\xFD[a;\xCE\x85\x82\x86\x01a65V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a;\xEDW`\0\x80\xFD[\x825\x91Pa;\x90` \x84\x01a6$V[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a<\x15W`\0\x80\xFD[\x855\x94Pa<%` \x87\x01a6$V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<AW`\0\x80\xFD[a<M\x89\x83\x8A\x01a65V[\x90\x95P\x93P``\x88\x015\x91P\x80\x82\x11\x15a<fW`\0\x80\xFD[Pa<s\x88\x82\x89\x01a7(V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a<\x96W`\0\x80\xFD[\x845\x93Pa<\xA6` \x86\x01a6$V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xC1W`\0\x80\xFD[a<\xCD\x87\x82\x88\x01a65V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a<\xEFW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=\x06W`\0\x80\xFD[a=\x12\x88\x83\x89\x01a9\x0BV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a=(W`\0\x80\xFD[a=4\x88\x83\x89\x01a9xV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a=JW`\0\x80\xFD[a=V\x88\x83\x89\x01a9\xD3V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a=lW`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a=~W`\0\x80\xFD[a8\xDC\x87\x825` \x84\x01a6\xEAV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a=\xBDW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a=\xA1V[P\x94\x95\x94PPPPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a=\xE7`\xE0\x83\x01\x89a8+V[\x82\x81\x03`@\x84\x01Ra=\xF9\x81\x89a8+V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x90Pa\x0BD\x81\x85a=\x8DV[`\0\x80`\0``\x84\x86\x03\x12\x15a>?W`\0\x80\xFD[a>H\x84a8jV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a>jW`\0\x80\xFD[a>v\x86\x82\x87\x01a7(V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a>\x98W`\0\x80\xFD[a>\xA1\x86a8jV[\x94Pa>\xAF` \x87\x01a8jV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>\xCBW`\0\x80\xFD[a>\xD7\x89\x83\x8A\x01a9xV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a>\xEDW`\0\x80\xFD[a>\xF9\x89\x83\x8A\x01a9xV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a<fW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a?%W`\0\x80\xFD[a?.\x85a8jV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xC1W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a?cW`\0\x80\xFD[a?l\x83a8jV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a?\x92W`\0\x80\xFD[a?\x9B\x86a8jV[\x94Pa?\xA9` \x87\x01a8jV[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xD2W`\0\x80\xFD[a<s\x88\x82\x89\x01a7(V[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a?\xF4W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\x0BW`\0\x80\xFD[a@\x17\x88\x83\x89\x01a65V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a@0W`\0\x80\xFD[Pa<\xCD\x87\x82\x88\x01a65V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x80\x82R`\x18\x90\x82\x01R\x7FGovernor: onlyGovernance\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a@\x98W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a@\xB8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aA\x10W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aA&W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aA7W`\0\x80\xFD[\x80QaAEa6\xF8\x82a6\xC3V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aAZW`\0\x80\xFD[a$\x0B\x82` \x83\x01` \x86\x01a8\x07V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0F=Wa\x0F=aAkV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`!\x90\x82\x01R\x7FGovernor: invalid proposal lengt`@\x82\x01R`\r`\xFB\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0F=Wa\x0F=aAkV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a=\xBDW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aB\x12V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aB\x7FW\x82\x84\x03\x89RaBm\x84\x83Qa8+V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aBUV[P\x91\x97\x96PPPPPPPV[`\0a\x01 \x8B\x83R` `\x01\x80`\xA0\x1B\x03\x8C\x16\x81\x85\x01R\x81`@\x85\x01RaB\xB5\x82\x85\x01\x8CaA\xFEV[\x91P\x83\x82\x03``\x85\x01RaB\xC9\x82\x8Ba=\x8DV[\x91P\x83\x82\x03`\x80\x85\x01R\x81\x89Q\x80\x84R\x82\x84\x01\x91P\x82\x81`\x05\x1B\x85\x01\x01\x83\x8C\x01`\0[\x83\x81\x10\x15aC\x1AW`\x1F\x19\x87\x84\x03\x01\x85RaC\x08\x83\x83Qa8+V[\x94\x86\x01\x94\x92P\x90\x85\x01\x90`\x01\x01aB\xECV[PP\x86\x81\x03`\xA0\x88\x01RaC.\x81\x8CaB7V[\x94PPPPP\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01RaCS\x81\x85a8+V[\x9C\x9BPPPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15aCuW`\0\x80\xFD[\x81Qe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x82W`\0\x80\xFD[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R`\0\x84TaC\xAD\x81a@\x84V[\x80`@\x87\x01R```\x01\x80\x84\x16`\0\x81\x14aC\xCFW`\x01\x81\x14aC\xE9WaD\x17V[`\xFF\x19\x85\x16\x89\x84\x01R\x83\x15\x15`\x05\x1B\x89\x01\x83\x01\x95PaD\x17V[\x89`\0R\x86`\0 `\0[\x85\x81\x10\x15aD\x0FW\x81T\x8B\x82\x01\x86\x01R\x90\x83\x01\x90\x88\x01aC\xF4V[\x8A\x01\x84\x01\x96PP[P\x93\x99\x98PPPPPPPPPV[`\x80\x81R`\0aD9`\x80\x83\x01\x87aA\xFEV[\x82\x81\x03` \x84\x01RaDK\x81\x87a=\x8DV[\x90P\x82\x81\x03`@\x84\x01RaD_\x81\x86aB7V[\x91PP\x82``\x83\x01R\x95\x94PPPPPV[`\x1F\x82\x11\x15aD\xB7W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aD\x98WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x10\x11W\x82\x81U`\x01\x01aD\xA4V[PPPV[`\x01`\x01`@\x1B\x03\x83\x11\x15aD\xD3WaD\xD3a6}V[aD\xE7\x83aD\xE1\x83Ta@\x84V[\x83aDqV[`\0`\x1F\x84\x11`\x01\x81\x14aE\x1BW`\0\x85\x15aE\x03WP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\"\xEBV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15aELW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aE,V[P\x86\x82\x10\x15aEiW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x84\x81R`\xFF\x84\x16` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\n@`\x80\x83\x01\x84a8+V[\x85\x81R`\xFF\x85\x16` \x82\x01R\x83`@\x82\x01R`\xA0``\x82\x01R`\0aE\xCB`\xA0\x83\x01\x85a8+V[\x82\x81\x03`\x80\x84\x01RaE\xDD\x81\x85a8+V[\x98\x97PPPPPPPPV[`\0`\x01\x82\x01aE\xFBWaE\xFBaAkV[P`\x01\x01\x90V[`\0\x82QaF\x14\x81\x84` \x87\x01a8\x07V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aF0W`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0F=Wa\x0F=aAkV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aF\x81WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFEGovernor: relay reverted without messageGovernor: call reverted without message\xA2dipfsX\"\x12 \xEF]\xA7\x1E\x1C)Q\x86\xBB\xCB\xC2\x06\x9Bb7V\x96 J~\x87\xAC\x1D3\xC1\xA7\x03\xA8\xC3y\xDC\x8DdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static PROJECT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Project<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Project<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Project<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Project<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Project<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Project)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Project<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PROJECT_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                PROJECT_ABI.clone(),
                PROJECT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BALLOT_TYPEHASH` (0xdeaaa7cc) function
        pub fn ballot_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([222, 170, 167, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CLOCK_MODE` (0x4bf5d7e9) function
        pub fn clock_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([75, 245, 215, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COUNTING_MODE` (0xdd4e2ba5) function
        pub fn counting_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([221, 78, 43, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EXTENDED_BALLOT_TYPEHASH` (0x2fe3e261) function
        pub fn extended_ballot_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([47, 227, 226, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancel` (0x452115d6) function
        pub fn cancel(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [69, 33, 21, 214],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVote` (0x56781388) function
        pub fn cast_vote(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 120, 19, 136], (proposal_id, support))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteBySig` (0x3bccf4fd) function
        pub fn cast_vote_by_sig(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 204, 244, 253], (proposal_id, support, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReason` (0x7b3c71d3) function
        pub fn cast_vote_with_reason(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
            reason: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([123, 60, 113, 211], (proposal_id, support, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReasonAndParams` (0x5f398a14) function
        pub fn cast_vote_with_reason_and_params(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
            reason: ::std::string::String,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([95, 57, 138, 20], (proposal_id, support, reason, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReasonAndParamsBySig` (0x03420181) function
        pub fn cast_vote_with_reason_and_params_by_sig(
            &self,
            proposal_id: ::ethers::core::types::U256,
            support: u8,
            reason: ::std::string::String,
            params: ::ethers::core::types::Bytes,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [3, 66, 1, 129],
                    (proposal_id, support, reason, params, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clock` (0x91ddadf4) function
        pub fn clock(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([145, 221, 173, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x2656227d) function
        pub fn execute(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [38, 86, 34, 125],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProposalIpfsHash` (0xfb049293) function
        pub fn get_proposal_ipfs_hash(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([251, 4, 146, 147], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotes` (0xeb9019d4) function
        pub fn get_votes(
            &self,
            account: ::ethers::core::types::Address,
            timepoint: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 144, 25, 212], (account, timepoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotesWithParams` (0x9a802a6d) function
        pub fn get_votes_with_params(
            &self,
            account: ::ethers::core::types::Address,
            timepoint: ::ethers::core::types::U256,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 128, 42, 109], (account, timepoint, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasVoted` (0x43859632) function
        pub fn has_voted(
            &self,
            proposal_id: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 133, 150, 50], (proposal_id, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashProposal` (0xc59057e4) function
        pub fn hash_proposal(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [197, 144, 87, 228],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x439fab91) function
        pub fn initialize(
            &self,
            ipfs_hash: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 159, 171, 145], ipfs_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inludeContributions` (0x9a020426) function
        pub fn inlude_contributions(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 2, 4, 38], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
            p3: ::std::vec::Vec<::ethers::core::types::U256>,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalDeadline` (0xc01f9e37) function
        pub fn proposal_deadline(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([192, 31, 158, 55], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalProposer` (0x143489d0) function
        pub fn proposal_proposer(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([20, 52, 137, 208], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalSnapshot` (0x2d63f693) function
        pub fn proposal_snapshot(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 99, 246, 147], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalThreshold` (0xb58131b0) function
        pub fn proposal_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([181, 129, 49, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalVotes` (0x544ffc9c) function
        pub fn proposal_votes(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([84, 79, 252, 156], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `propose` (0x7d5e81e2) function
        pub fn propose(
            &self,
            targets: ::std::vec::Vec<::ethers::core::types::Address>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
            description: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [125, 94, 129, 226],
                    (targets, values, calldatas, description),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorum` (0xf8ce560a) function
        pub fn quorum(
            &self,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([248, 206, 86, 10], block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumDenominator` (0x97c3d334) function
        pub fn quorum_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([151, 195, 211, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumNumerator` (0x60c4247f) function
        pub fn quorum_numerator_with_timepoint(
            &self,
            timepoint: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([96, 196, 36, 127], timepoint)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumNumerator` (0xa7713a70) function
        pub fn quorum_numerator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 113, 58, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relay` (0xc28bc2fa) function
        pub fn relay(
            &self,
            target: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 139, 194, 250], (target, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProposalThreshold` (0xece40cc1) function
        pub fn set_proposal_threshold(
            &self,
            new_proposal_threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 228, 12, 193], new_proposal_threshold)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVotingDelay` (0x70b0f660) function
        pub fn set_voting_delay(
            &self,
            new_voting_delay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 176, 246, 96], new_voting_delay)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setVotingPeriod` (0xea0217cf) function
        pub fn set_voting_period(
            &self,
            new_voting_period: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 2, 23, 207], new_voting_period)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `state` (0x3e4f49e6) function
        pub fn state(
            &self,
            proposal_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([62, 79, 73, 230], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitContributionsRequest` (0xfdc1f8d5) function
        pub fn submit_contributions_request(
            &self,
            ipfs_hash: ::ethers::core::types::Bytes,
            description: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([253, 193, 248, 213], (ipfs_hash, description))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trackerContract` (0x8b64bb87) function
        pub fn tracker_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([139, 100, 187, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateQuorumNumerator` (0x06f3f9e6) function
        pub fn update_quorum_numerator(
            &self,
            new_quorum_numerator: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 243, 249, 230], new_quorum_numerator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingDelay` (0x3932abb1) function
        pub fn voting_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([57, 50, 171, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingPeriod` (0x02a251a3) function
        pub fn voting_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([2, 162, 81, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EIP712DomainChanged` event
        pub fn eip712_domain_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Eip712DomainChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalCanceled` event
        pub fn proposal_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalCreated` event
        pub fn proposal_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalExecuted` event
        pub fn proposal_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalThresholdSet` event
        pub fn proposal_threshold_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalThresholdSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QuorumNumeratorUpdated` event
        pub fn quorum_numerator_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuorumNumeratorUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VoteCast` event
        pub fn vote_cast_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoteCastFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VoteCastWithParams` event
        pub fn vote_cast_with_params_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoteCastWithParamsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VotingDelaySet` event
        pub fn voting_delay_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VotingDelaySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VotingPeriodSet` event
        pub fn voting_period_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VotingPeriodSetFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProjectEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Project<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Empty` with signature `Empty()` and selector `0x3db2a12a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Empty", abi = "Empty()")]
    pub struct Empty;
    ///Custom Error type `InvalidShortString` with signature `InvalidShortString()` and selector `0xb3512b0c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidShortString", abi = "InvalidShortString()")]
    pub struct InvalidShortString;
    ///Custom Error type `StringTooLong` with signature `StringTooLong(string)` and selector `0x305a27a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "StringTooLong", abi = "StringTooLong(string)")]
    pub struct StringTooLong {
        pub str: ::std::string::String,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ProjectErrors {
        Empty(Empty),
        InvalidShortString(InvalidShortString),
        StringTooLong(StringTooLong),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ProjectErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <Empty as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Empty(decoded));
            }
            if let Ok(decoded)
                = <InvalidShortString as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidShortString(decoded));
            }
            if let Ok(decoded)
                = <StringTooLong as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StringTooLong(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ProjectErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Empty(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidShortString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringTooLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ProjectErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <Empty as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidShortString as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StringTooLong as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ProjectErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Empty(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidShortString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ProjectErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Empty> for ProjectErrors {
        fn from(value: Empty) -> Self {
            Self::Empty(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for ProjectErrors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for ProjectErrors {
        fn from(value: StringTooLong) -> Self {
            Self::StringTooLong(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "EIP712DomainChanged", abi = "EIP712DomainChanged()")]
    pub struct Eip712DomainChangedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ProposalCanceled", abi = "ProposalCanceled(uint256)")]
    pub struct ProposalCanceledFilter {
        pub proposal_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ProposalCreated",
        abi = "ProposalCreated(uint256,address,address[],uint256[],string[],bytes[],uint256,uint256,string)"
    )]
    pub struct ProposalCreatedFilter {
        pub proposal_id: ::ethers::core::types::U256,
        pub proposer: ::ethers::core::types::Address,
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub signatures: ::std::vec::Vec<::std::string::String>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub vote_start: ::ethers::core::types::U256,
        pub vote_end: ::ethers::core::types::U256,
        pub description: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ProposalExecuted", abi = "ProposalExecuted(uint256)")]
    pub struct ProposalExecutedFilter {
        pub proposal_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ProposalThresholdSet",
        abi = "ProposalThresholdSet(uint256,uint256)"
    )]
    pub struct ProposalThresholdSetFilter {
        pub old_proposal_threshold: ::ethers::core::types::U256,
        pub new_proposal_threshold: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "QuorumNumeratorUpdated",
        abi = "QuorumNumeratorUpdated(uint256,uint256)"
    )]
    pub struct QuorumNumeratorUpdatedFilter {
        pub old_quorum_numerator: ::ethers::core::types::U256,
        pub new_quorum_numerator: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "VoteCast",
        abi = "VoteCast(address,uint256,uint8,uint256,string)"
    )]
    pub struct VoteCastFilter {
        #[ethevent(indexed)]
        pub voter: ::ethers::core::types::Address,
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub weight: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "VoteCastWithParams",
        abi = "VoteCastWithParams(address,uint256,uint8,uint256,string,bytes)"
    )]
    pub struct VoteCastWithParamsFilter {
        #[ethevent(indexed)]
        pub voter: ::ethers::core::types::Address,
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub weight: ::ethers::core::types::U256,
        pub reason: ::std::string::String,
        pub params: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "VotingDelaySet", abi = "VotingDelaySet(uint256,uint256)")]
    pub struct VotingDelaySetFilter {
        pub old_voting_delay: ::ethers::core::types::U256,
        pub new_voting_delay: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "VotingPeriodSet", abi = "VotingPeriodSet(uint256,uint256)")]
    pub struct VotingPeriodSetFilter {
        pub old_voting_period: ::ethers::core::types::U256,
        pub new_voting_period: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ProjectEvents {
        Eip712DomainChangedFilter(Eip712DomainChangedFilter),
        ProposalCanceledFilter(ProposalCanceledFilter),
        ProposalCreatedFilter(ProposalCreatedFilter),
        ProposalExecutedFilter(ProposalExecutedFilter),
        ProposalThresholdSetFilter(ProposalThresholdSetFilter),
        QuorumNumeratorUpdatedFilter(QuorumNumeratorUpdatedFilter),
        VoteCastFilter(VoteCastFilter),
        VoteCastWithParamsFilter(VoteCastWithParamsFilter),
        VotingDelaySetFilter(VotingDelaySetFilter),
        VotingPeriodSetFilter(VotingPeriodSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for ProjectEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = Eip712DomainChangedFilter::decode_log(log) {
                return Ok(ProjectEvents::Eip712DomainChangedFilter(decoded));
            }
            if let Ok(decoded) = ProposalCanceledFilter::decode_log(log) {
                return Ok(ProjectEvents::ProposalCanceledFilter(decoded));
            }
            if let Ok(decoded) = ProposalCreatedFilter::decode_log(log) {
                return Ok(ProjectEvents::ProposalCreatedFilter(decoded));
            }
            if let Ok(decoded) = ProposalExecutedFilter::decode_log(log) {
                return Ok(ProjectEvents::ProposalExecutedFilter(decoded));
            }
            if let Ok(decoded) = ProposalThresholdSetFilter::decode_log(log) {
                return Ok(ProjectEvents::ProposalThresholdSetFilter(decoded));
            }
            if let Ok(decoded) = QuorumNumeratorUpdatedFilter::decode_log(log) {
                return Ok(ProjectEvents::QuorumNumeratorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = VoteCastFilter::decode_log(log) {
                return Ok(ProjectEvents::VoteCastFilter(decoded));
            }
            if let Ok(decoded) = VoteCastWithParamsFilter::decode_log(log) {
                return Ok(ProjectEvents::VoteCastWithParamsFilter(decoded));
            }
            if let Ok(decoded) = VotingDelaySetFilter::decode_log(log) {
                return Ok(ProjectEvents::VotingDelaySetFilter(decoded));
            }
            if let Ok(decoded) = VotingPeriodSetFilter::decode_log(log) {
                return Ok(ProjectEvents::VotingPeriodSetFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ProjectEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Eip712DomainChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalThresholdSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumNumeratorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoteCastFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoteCastWithParamsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VotingDelaySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VotingPeriodSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Eip712DomainChangedFilter> for ProjectEvents {
        fn from(value: Eip712DomainChangedFilter) -> Self {
            Self::Eip712DomainChangedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalCanceledFilter> for ProjectEvents {
        fn from(value: ProposalCanceledFilter) -> Self {
            Self::ProposalCanceledFilter(value)
        }
    }
    impl ::core::convert::From<ProposalCreatedFilter> for ProjectEvents {
        fn from(value: ProposalCreatedFilter) -> Self {
            Self::ProposalCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalExecutedFilter> for ProjectEvents {
        fn from(value: ProposalExecutedFilter) -> Self {
            Self::ProposalExecutedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalThresholdSetFilter> for ProjectEvents {
        fn from(value: ProposalThresholdSetFilter) -> Self {
            Self::ProposalThresholdSetFilter(value)
        }
    }
    impl ::core::convert::From<QuorumNumeratorUpdatedFilter> for ProjectEvents {
        fn from(value: QuorumNumeratorUpdatedFilter) -> Self {
            Self::QuorumNumeratorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<VoteCastFilter> for ProjectEvents {
        fn from(value: VoteCastFilter) -> Self {
            Self::VoteCastFilter(value)
        }
    }
    impl ::core::convert::From<VoteCastWithParamsFilter> for ProjectEvents {
        fn from(value: VoteCastWithParamsFilter) -> Self {
            Self::VoteCastWithParamsFilter(value)
        }
    }
    impl ::core::convert::From<VotingDelaySetFilter> for ProjectEvents {
        fn from(value: VotingDelaySetFilter) -> Self {
            Self::VotingDelaySetFilter(value)
        }
    }
    impl ::core::convert::From<VotingPeriodSetFilter> for ProjectEvents {
        fn from(value: VotingPeriodSetFilter) -> Self {
            Self::VotingPeriodSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `BALLOT_TYPEHASH` function with signature `BALLOT_TYPEHASH()` and selector `0xdeaaa7cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BALLOT_TYPEHASH", abi = "BALLOT_TYPEHASH()")]
    pub struct BallotTypehashCall;
    ///Container type for all input parameters for the `CLOCK_MODE` function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "CLOCK_MODE", abi = "CLOCK_MODE()")]
    pub struct ClockModeCall;
    ///Container type for all input parameters for the `COUNTING_MODE` function with signature `COUNTING_MODE()` and selector `0xdd4e2ba5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "COUNTING_MODE", abi = "COUNTING_MODE()")]
    pub struct CountingModeCall;
    ///Container type for all input parameters for the `EXTENDED_BALLOT_TYPEHASH` function with signature `EXTENDED_BALLOT_TYPEHASH()` and selector `0x2fe3e261`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "EXTENDED_BALLOT_TYPEHASH", abi = "EXTENDED_BALLOT_TYPEHASH()")]
    pub struct ExtendedBallotTypehashCall;
    ///Container type for all input parameters for the `cancel` function with signature `cancel(address[],uint256[],bytes[],bytes32)` and selector `0x452115d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cancel", abi = "cancel(address[],uint256[],bytes[],bytes32)")]
    pub struct CancelCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `castVote` function with signature `castVote(uint256,uint8)` and selector `0x56781388`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "castVote", abi = "castVote(uint256,uint8)")]
    pub struct CastVoteCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
    }
    ///Container type for all input parameters for the `castVoteBySig` function with signature `castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)` and selector `0x3bccf4fd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "castVoteBySig",
        abi = "castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)"
    )]
    pub struct CastVoteBySigCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `castVoteWithReason` function with signature `castVoteWithReason(uint256,uint8,string)` and selector `0x7b3c71d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "castVoteWithReason",
        abi = "castVoteWithReason(uint256,uint8,string)"
    )]
    pub struct CastVoteWithReasonCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `castVoteWithReasonAndParams` function with signature `castVoteWithReasonAndParams(uint256,uint8,string,bytes)` and selector `0x5f398a14`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "castVoteWithReasonAndParams",
        abi = "castVoteWithReasonAndParams(uint256,uint8,string,bytes)"
    )]
    pub struct CastVoteWithReasonAndParamsCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `castVoteWithReasonAndParamsBySig` function with signature `castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)` and selector `0x03420181`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "castVoteWithReasonAndParamsBySig",
        abi = "castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)"
    )]
    pub struct CastVoteWithReasonAndParamsBySigCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
        pub params: ::ethers::core::types::Bytes,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `clock` function with signature `clock()` and selector `0x91ddadf4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clock", abi = "clock()")]
    pub struct ClockCall;
    ///Container type for all input parameters for the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
    ///Container type for all input parameters for the `execute` function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "execute", abi = "execute(address[],uint256[],bytes[],bytes32)")]
    pub struct ExecuteCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getProposalIpfsHash` function with signature `getProposalIpfsHash(uint256)` and selector `0xfb049293`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getProposalIpfsHash", abi = "getProposalIpfsHash(uint256)")]
    pub struct GetProposalIpfsHashCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getVotes` function with signature `getVotes(address,uint256)` and selector `0xeb9019d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getVotes", abi = "getVotes(address,uint256)")]
    pub struct GetVotesCall {
        pub account: ::ethers::core::types::Address,
        pub timepoint: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getVotesWithParams` function with signature `getVotesWithParams(address,uint256,bytes)` and selector `0x9a802a6d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getVotesWithParams",
        abi = "getVotesWithParams(address,uint256,bytes)"
    )]
    pub struct GetVotesWithParamsCall {
        pub account: ::ethers::core::types::Address,
        pub timepoint: ::ethers::core::types::U256,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hasVoted` function with signature `hasVoted(uint256,address)` and selector `0x43859632`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hasVoted", abi = "hasVoted(uint256,address)")]
    pub struct HasVotedCall {
        pub proposal_id: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hashProposal` function with signature `hashProposal(address[],uint256[],bytes[],bytes32)` and selector `0xc59057e4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "hashProposal",
        abi = "hashProposal(address[],uint256[],bytes[],bytes32)"
    )]
    pub struct HashProposalCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(bytes)` and selector `0x439fab91`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(bytes)")]
    pub struct InitializeCall {
        pub ipfs_hash: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `inludeContributions` function with signature `inludeContributions(uint256)` and selector `0x9a020426`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "inludeContributions", abi = "inludeContributions(uint256)")]
    pub struct InludeContributionsCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `proposalDeadline` function with signature `proposalDeadline(uint256)` and selector `0xc01f9e37`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposalDeadline", abi = "proposalDeadline(uint256)")]
    pub struct ProposalDeadlineCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proposalProposer` function with signature `proposalProposer(uint256)` and selector `0x143489d0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposalProposer", abi = "proposalProposer(uint256)")]
    pub struct ProposalProposerCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proposalSnapshot` function with signature `proposalSnapshot(uint256)` and selector `0x2d63f693`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposalSnapshot", abi = "proposalSnapshot(uint256)")]
    pub struct ProposalSnapshotCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `proposalThreshold` function with signature `proposalThreshold()` and selector `0xb58131b0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposalThreshold", abi = "proposalThreshold()")]
    pub struct ProposalThresholdCall;
    ///Container type for all input parameters for the `proposalVotes` function with signature `proposalVotes(uint256)` and selector `0x544ffc9c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposalVotes", abi = "proposalVotes(uint256)")]
    pub struct ProposalVotesCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `propose` function with signature `propose(address[],uint256[],bytes[],string)` and selector `0x7d5e81e2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "propose", abi = "propose(address[],uint256[],bytes[],string)")]
    pub struct ProposeCall {
        pub targets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `quorum` function with signature `quorum(uint256)` and selector `0xf8ce560a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quorum", abi = "quorum(uint256)")]
    pub struct QuorumCall {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quorumDenominator` function with signature `quorumDenominator()` and selector `0x97c3d334`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quorumDenominator", abi = "quorumDenominator()")]
    pub struct QuorumDenominatorCall;
    ///Container type for all input parameters for the `quorumNumerator` function with signature `quorumNumerator(uint256)` and selector `0x60c4247f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quorumNumerator", abi = "quorumNumerator(uint256)")]
    pub struct QuorumNumeratorWithTimepointCall {
        pub timepoint: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quorumNumerator` function with signature `quorumNumerator()` and selector `0xa7713a70`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quorumNumerator", abi = "quorumNumerator()")]
    pub struct QuorumNumeratorCall;
    ///Container type for all input parameters for the `relay` function with signature `relay(address,uint256,bytes)` and selector `0xc28bc2fa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "relay", abi = "relay(address,uint256,bytes)")]
    pub struct RelayCall {
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setProposalThreshold` function with signature `setProposalThreshold(uint256)` and selector `0xece40cc1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setProposalThreshold", abi = "setProposalThreshold(uint256)")]
    pub struct SetProposalThresholdCall {
        pub new_proposal_threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setVotingDelay` function with signature `setVotingDelay(uint256)` and selector `0x70b0f660`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setVotingDelay", abi = "setVotingDelay(uint256)")]
    pub struct SetVotingDelayCall {
        pub new_voting_delay: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setVotingPeriod` function with signature `setVotingPeriod(uint256)` and selector `0xea0217cf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setVotingPeriod", abi = "setVotingPeriod(uint256)")]
    pub struct SetVotingPeriodCall {
        pub new_voting_period: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `state` function with signature `state(uint256)` and selector `0x3e4f49e6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "state", abi = "state(uint256)")]
    pub struct StateCall {
        pub proposal_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `submitContributionsRequest` function with signature `submitContributionsRequest(bytes,string)` and selector `0xfdc1f8d5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "submitContributionsRequest",
        abi = "submitContributionsRequest(bytes,string)"
    )]
    pub struct SubmitContributionsRequestCall {
        pub ipfs_hash: ::ethers::core::types::Bytes,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `trackerContract` function with signature `trackerContract()` and selector `0x8b64bb87`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "trackerContract", abi = "trackerContract()")]
    pub struct TrackerContractCall;
    ///Container type for all input parameters for the `updateQuorumNumerator` function with signature `updateQuorumNumerator(uint256)` and selector `0x06f3f9e6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateQuorumNumerator", abi = "updateQuorumNumerator(uint256)")]
    pub struct UpdateQuorumNumeratorCall {
        pub new_quorum_numerator: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `votingDelay` function with signature `votingDelay()` and selector `0x3932abb1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "votingDelay", abi = "votingDelay()")]
    pub struct VotingDelayCall;
    ///Container type for all input parameters for the `votingPeriod` function with signature `votingPeriod()` and selector `0x02a251a3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "votingPeriod", abi = "votingPeriod()")]
    pub struct VotingPeriodCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ProjectCalls {
        BallotTypehash(BallotTypehashCall),
        ClockMode(ClockModeCall),
        CountingMode(CountingModeCall),
        ExtendedBallotTypehash(ExtendedBallotTypehashCall),
        Cancel(CancelCall),
        CastVote(CastVoteCall),
        CastVoteBySig(CastVoteBySigCall),
        CastVoteWithReason(CastVoteWithReasonCall),
        CastVoteWithReasonAndParams(CastVoteWithReasonAndParamsCall),
        CastVoteWithReasonAndParamsBySig(CastVoteWithReasonAndParamsBySigCall),
        Clock(ClockCall),
        Eip712Domain(Eip712DomainCall),
        Execute(ExecuteCall),
        GetProposalIpfsHash(GetProposalIpfsHashCall),
        GetVotes(GetVotesCall),
        GetVotesWithParams(GetVotesWithParamsCall),
        HasVoted(HasVotedCall),
        HashProposal(HashProposalCall),
        Initialize(InitializeCall),
        InludeContributions(InludeContributionsCall),
        Name(NameCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        ProposalDeadline(ProposalDeadlineCall),
        ProposalProposer(ProposalProposerCall),
        ProposalSnapshot(ProposalSnapshotCall),
        ProposalThreshold(ProposalThresholdCall),
        ProposalVotes(ProposalVotesCall),
        Propose(ProposeCall),
        Quorum(QuorumCall),
        QuorumDenominator(QuorumDenominatorCall),
        QuorumNumeratorWithTimepoint(QuorumNumeratorWithTimepointCall),
        QuorumNumerator(QuorumNumeratorCall),
        Relay(RelayCall),
        SetProposalThreshold(SetProposalThresholdCall),
        SetVotingDelay(SetVotingDelayCall),
        SetVotingPeriod(SetVotingPeriodCall),
        State(StateCall),
        SubmitContributionsRequest(SubmitContributionsRequestCall),
        SupportsInterface(SupportsInterfaceCall),
        Token(TokenCall),
        TrackerContract(TrackerContractCall),
        UpdateQuorumNumerator(UpdateQuorumNumeratorCall),
        Version(VersionCall),
        VotingDelay(VotingDelayCall),
        VotingPeriod(VotingPeriodCall),
    }
    impl ::ethers::core::abi::AbiDecode for ProjectCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BallotTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BallotTypehash(decoded));
            }
            if let Ok(decoded)
                = <ClockModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClockMode(decoded));
            }
            if let Ok(decoded)
                = <CountingModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CountingMode(decoded));
            }
            if let Ok(decoded)
                = <ExtendedBallotTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExtendedBallotTypehash(decoded));
            }
            if let Ok(decoded)
                = <CancelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Cancel(decoded));
            }
            if let Ok(decoded)
                = <CastVoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CastVote(decoded));
            }
            if let Ok(decoded)
                = <CastVoteBySigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CastVoteBySig(decoded));
            }
            if let Ok(decoded)
                = <CastVoteWithReasonCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CastVoteWithReason(decoded));
            }
            if let Ok(decoded)
                = <CastVoteWithReasonAndParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CastVoteWithReasonAndParams(decoded));
            }
            if let Ok(decoded)
                = <CastVoteWithReasonAndParamsBySigCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CastVoteWithReasonAndParamsBySig(decoded));
            }
            if let Ok(decoded)
                = <ClockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Clock(decoded));
            }
            if let Ok(decoded)
                = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded)
                = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded)
                = <GetProposalIpfsHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetProposalIpfsHash(decoded));
            }
            if let Ok(decoded)
                = <GetVotesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVotes(decoded));
            }
            if let Ok(decoded)
                = <GetVotesWithParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVotesWithParams(decoded));
            }
            if let Ok(decoded)
                = <HasVotedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasVoted(decoded));
            }
            if let Ok(decoded)
                = <HashProposalCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashProposal(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <InludeContributionsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InludeContributions(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded)
                = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded)
                = <ProposalDeadlineCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposalDeadline(decoded));
            }
            if let Ok(decoded)
                = <ProposalProposerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposalProposer(decoded));
            }
            if let Ok(decoded)
                = <ProposalSnapshotCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposalSnapshot(decoded));
            }
            if let Ok(decoded)
                = <ProposalThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposalThreshold(decoded));
            }
            if let Ok(decoded)
                = <ProposalVotesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProposalVotes(decoded));
            }
            if let Ok(decoded)
                = <ProposeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Propose(decoded));
            }
            if let Ok(decoded)
                = <QuorumCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Quorum(decoded));
            }
            if let Ok(decoded)
                = <QuorumDenominatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::QuorumDenominator(decoded));
            }
            if let Ok(decoded)
                = <QuorumNumeratorWithTimepointCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::QuorumNumeratorWithTimepoint(decoded));
            }
            if let Ok(decoded)
                = <QuorumNumeratorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QuorumNumerator(decoded));
            }
            if let Ok(decoded)
                = <RelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Relay(decoded));
            }
            if let Ok(decoded)
                = <SetProposalThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetProposalThreshold(decoded));
            }
            if let Ok(decoded)
                = <SetVotingDelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetVotingDelay(decoded));
            }
            if let Ok(decoded)
                = <SetVotingPeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetVotingPeriod(decoded));
            }
            if let Ok(decoded)
                = <StateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::State(decoded));
            }
            if let Ok(decoded)
                = <SubmitContributionsRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SubmitContributionsRequest(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded)
                = <TrackerContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TrackerContract(decoded));
            }
            if let Ok(decoded)
                = <UpdateQuorumNumeratorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateQuorumNumerator(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <VotingDelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VotingDelay(decoded));
            }
            if let Ok(decoded)
                = <VotingPeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VotingPeriod(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ProjectCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BallotTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClockMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CountingMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExtendedBallotTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cancel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CastVote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteBySig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReason(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReasonAndParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReasonAndParamsBySig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Clock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eip712Domain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetProposalIpfsHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVotesWithParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasVoted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InludeContributions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalDeadline(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalSnapshot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Propose(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quorum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuorumDenominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumNumeratorWithTimepoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Relay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProposalThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVotingDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetVotingPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitContributionsRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TrackerContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateQuorumNumerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VotingDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VotingPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ProjectCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BallotTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClockMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::CountingMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtendedBallotTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cancel(element) => ::core::fmt::Display::fmt(element, f),
                Self::CastVote(element) => ::core::fmt::Display::fmt(element, f),
                Self::CastVoteBySig(element) => ::core::fmt::Display::fmt(element, f),
                Self::CastVoteWithReason(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CastVoteWithReasonAndParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CastVoteWithReasonAndParamsBySig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Clock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProposalIpfsHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotesWithParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasVoted(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::InludeContributions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalDeadline(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalProposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalSnapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Propose(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumDenominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumNumeratorWithTimepoint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumNumerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Relay(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProposalThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetVotingDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetVotingPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitContributionsRequest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrackerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateQuorumNumerator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingPeriod(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BallotTypehashCall> for ProjectCalls {
        fn from(value: BallotTypehashCall) -> Self {
            Self::BallotTypehash(value)
        }
    }
    impl ::core::convert::From<ClockModeCall> for ProjectCalls {
        fn from(value: ClockModeCall) -> Self {
            Self::ClockMode(value)
        }
    }
    impl ::core::convert::From<CountingModeCall> for ProjectCalls {
        fn from(value: CountingModeCall) -> Self {
            Self::CountingMode(value)
        }
    }
    impl ::core::convert::From<ExtendedBallotTypehashCall> for ProjectCalls {
        fn from(value: ExtendedBallotTypehashCall) -> Self {
            Self::ExtendedBallotTypehash(value)
        }
    }
    impl ::core::convert::From<CancelCall> for ProjectCalls {
        fn from(value: CancelCall) -> Self {
            Self::Cancel(value)
        }
    }
    impl ::core::convert::From<CastVoteCall> for ProjectCalls {
        fn from(value: CastVoteCall) -> Self {
            Self::CastVote(value)
        }
    }
    impl ::core::convert::From<CastVoteBySigCall> for ProjectCalls {
        fn from(value: CastVoteBySigCall) -> Self {
            Self::CastVoteBySig(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonCall> for ProjectCalls {
        fn from(value: CastVoteWithReasonCall) -> Self {
            Self::CastVoteWithReason(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonAndParamsCall> for ProjectCalls {
        fn from(value: CastVoteWithReasonAndParamsCall) -> Self {
            Self::CastVoteWithReasonAndParams(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonAndParamsBySigCall> for ProjectCalls {
        fn from(value: CastVoteWithReasonAndParamsBySigCall) -> Self {
            Self::CastVoteWithReasonAndParamsBySig(value)
        }
    }
    impl ::core::convert::From<ClockCall> for ProjectCalls {
        fn from(value: ClockCall) -> Self {
            Self::Clock(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for ProjectCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for ProjectCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<GetProposalIpfsHashCall> for ProjectCalls {
        fn from(value: GetProposalIpfsHashCall) -> Self {
            Self::GetProposalIpfsHash(value)
        }
    }
    impl ::core::convert::From<GetVotesCall> for ProjectCalls {
        fn from(value: GetVotesCall) -> Self {
            Self::GetVotes(value)
        }
    }
    impl ::core::convert::From<GetVotesWithParamsCall> for ProjectCalls {
        fn from(value: GetVotesWithParamsCall) -> Self {
            Self::GetVotesWithParams(value)
        }
    }
    impl ::core::convert::From<HasVotedCall> for ProjectCalls {
        fn from(value: HasVotedCall) -> Self {
            Self::HasVoted(value)
        }
    }
    impl ::core::convert::From<HashProposalCall> for ProjectCalls {
        fn from(value: HashProposalCall) -> Self {
            Self::HashProposal(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ProjectCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InludeContributionsCall> for ProjectCalls {
        fn from(value: InludeContributionsCall) -> Self {
            Self::InludeContributions(value)
        }
    }
    impl ::core::convert::From<NameCall> for ProjectCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for ProjectCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for ProjectCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for ProjectCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<ProposalDeadlineCall> for ProjectCalls {
        fn from(value: ProposalDeadlineCall) -> Self {
            Self::ProposalDeadline(value)
        }
    }
    impl ::core::convert::From<ProposalProposerCall> for ProjectCalls {
        fn from(value: ProposalProposerCall) -> Self {
            Self::ProposalProposer(value)
        }
    }
    impl ::core::convert::From<ProposalSnapshotCall> for ProjectCalls {
        fn from(value: ProposalSnapshotCall) -> Self {
            Self::ProposalSnapshot(value)
        }
    }
    impl ::core::convert::From<ProposalThresholdCall> for ProjectCalls {
        fn from(value: ProposalThresholdCall) -> Self {
            Self::ProposalThreshold(value)
        }
    }
    impl ::core::convert::From<ProposalVotesCall> for ProjectCalls {
        fn from(value: ProposalVotesCall) -> Self {
            Self::ProposalVotes(value)
        }
    }
    impl ::core::convert::From<ProposeCall> for ProjectCalls {
        fn from(value: ProposeCall) -> Self {
            Self::Propose(value)
        }
    }
    impl ::core::convert::From<QuorumCall> for ProjectCalls {
        fn from(value: QuorumCall) -> Self {
            Self::Quorum(value)
        }
    }
    impl ::core::convert::From<QuorumDenominatorCall> for ProjectCalls {
        fn from(value: QuorumDenominatorCall) -> Self {
            Self::QuorumDenominator(value)
        }
    }
    impl ::core::convert::From<QuorumNumeratorWithTimepointCall> for ProjectCalls {
        fn from(value: QuorumNumeratorWithTimepointCall) -> Self {
            Self::QuorumNumeratorWithTimepoint(value)
        }
    }
    impl ::core::convert::From<QuorumNumeratorCall> for ProjectCalls {
        fn from(value: QuorumNumeratorCall) -> Self {
            Self::QuorumNumerator(value)
        }
    }
    impl ::core::convert::From<RelayCall> for ProjectCalls {
        fn from(value: RelayCall) -> Self {
            Self::Relay(value)
        }
    }
    impl ::core::convert::From<SetProposalThresholdCall> for ProjectCalls {
        fn from(value: SetProposalThresholdCall) -> Self {
            Self::SetProposalThreshold(value)
        }
    }
    impl ::core::convert::From<SetVotingDelayCall> for ProjectCalls {
        fn from(value: SetVotingDelayCall) -> Self {
            Self::SetVotingDelay(value)
        }
    }
    impl ::core::convert::From<SetVotingPeriodCall> for ProjectCalls {
        fn from(value: SetVotingPeriodCall) -> Self {
            Self::SetVotingPeriod(value)
        }
    }
    impl ::core::convert::From<StateCall> for ProjectCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    impl ::core::convert::From<SubmitContributionsRequestCall> for ProjectCalls {
        fn from(value: SubmitContributionsRequestCall) -> Self {
            Self::SubmitContributionsRequest(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ProjectCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TokenCall> for ProjectCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<TrackerContractCall> for ProjectCalls {
        fn from(value: TrackerContractCall) -> Self {
            Self::TrackerContract(value)
        }
    }
    impl ::core::convert::From<UpdateQuorumNumeratorCall> for ProjectCalls {
        fn from(value: UpdateQuorumNumeratorCall) -> Self {
            Self::UpdateQuorumNumerator(value)
        }
    }
    impl ::core::convert::From<VersionCall> for ProjectCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<VotingDelayCall> for ProjectCalls {
        fn from(value: VotingDelayCall) -> Self {
            Self::VotingDelay(value)
        }
    }
    impl ::core::convert::From<VotingPeriodCall> for ProjectCalls {
        fn from(value: VotingPeriodCall) -> Self {
            Self::VotingPeriod(value)
        }
    }
    ///Container type for all return fields from the `BALLOT_TYPEHASH` function with signature `BALLOT_TYPEHASH()` and selector `0xdeaaa7cc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BallotTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CLOCK_MODE` function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ClockModeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `COUNTING_MODE` function with signature `COUNTING_MODE()` and selector `0xdd4e2ba5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CountingModeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `EXTENDED_BALLOT_TYPEHASH` function with signature `EXTENDED_BALLOT_TYPEHASH()` and selector `0x2fe3e261`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExtendedBallotTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `cancel` function with signature `cancel(address[],uint256[],bytes[],bytes32)` and selector `0x452115d6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CancelReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVote` function with signature `castVote(uint256,uint8)` and selector `0x56781388`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CastVoteReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVoteBySig` function with signature `castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)` and selector `0x3bccf4fd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CastVoteBySigReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVoteWithReason` function with signature `castVoteWithReason(uint256,uint8,string)` and selector `0x7b3c71d3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CastVoteWithReasonReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVoteWithReasonAndParams` function with signature `castVoteWithReasonAndParams(uint256,uint8,string,bytes)` and selector `0x5f398a14`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CastVoteWithReasonAndParamsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `castVoteWithReasonAndParamsBySig` function with signature `castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)` and selector `0x03420181`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CastVoteWithReasonAndParamsBySigReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `clock` function with signature `clock()` and selector `0x91ddadf4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ClockReturn(pub u64);
    ///Container type for all return fields from the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `execute` function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExecuteReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getProposalIpfsHash` function with signature `getProposalIpfsHash(uint256)` and selector `0xfb049293`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetProposalIpfsHashReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getVotes` function with signature `getVotes(address,uint256)` and selector `0xeb9019d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetVotesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVotesWithParams` function with signature `getVotesWithParams(address,uint256,bytes)` and selector `0x9a802a6d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetVotesWithParamsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasVoted` function with signature `hasVoted(uint256,address)` and selector `0x43859632`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HasVotedReturn(pub bool);
    ///Container type for all return fields from the `hashProposal` function with signature `hashProposal(address[],uint256[],bytes[],bytes32)` and selector `0xc59057e4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HashProposalReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `proposalDeadline` function with signature `proposalDeadline(uint256)` and selector `0xc01f9e37`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProposalDeadlineReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalProposer` function with signature `proposalProposer(uint256)` and selector `0x143489d0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProposalProposerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proposalSnapshot` function with signature `proposalSnapshot(uint256)` and selector `0x2d63f693`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProposalSnapshotReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalThreshold` function with signature `proposalThreshold()` and selector `0xb58131b0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProposalThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposalVotes` function with signature `proposalVotes(uint256)` and selector `0x544ffc9c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProposalVotesReturn {
        pub against_votes: ::ethers::core::types::U256,
        pub for_votes: ::ethers::core::types::U256,
        pub abstain_votes: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `propose` function with signature `propose(address[],uint256[],bytes[],string)` and selector `0x7d5e81e2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProposeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quorum` function with signature `quorum(uint256)` and selector `0xf8ce560a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuorumReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quorumDenominator` function with signature `quorumDenominator()` and selector `0x97c3d334`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuorumDenominatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quorumNumerator` function with signature `quorumNumerator(uint256)` and selector `0x60c4247f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuorumNumeratorWithTimepointReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quorumNumerator` function with signature `quorumNumerator()` and selector `0xa7713a70`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuorumNumeratorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `state` function with signature `state(uint256)` and selector `0x3e4f49e6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct StateReturn(pub u8);
    ///Container type for all return fields from the `submitContributionsRequest` function with signature `submitContributionsRequest(bytes,string)` and selector `0xfdc1f8d5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SubmitContributionsRequestReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `trackerContract` function with signature `trackerContract()` and selector `0x8b64bb87`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TrackerContractReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `votingDelay` function with signature `votingDelay()` and selector `0x3932abb1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VotingDelayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `votingPeriod` function with signature `votingPeriod()` and selector `0x02a251a3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VotingPeriodReturn(pub ::ethers::core::types::U256);
}
