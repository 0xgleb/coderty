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
                    ::std::borrow::ToOwned::to_owned("submitContributionRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "submitContributionRequest",
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
                            outputs: ::std::vec![],
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
                                        ::std::borrow::ToOwned::to_owned("contract Contribution"),
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
    const __BYTECODE: &[u8] = b"a\x01\x80`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0~28\x03\x80b\0~2\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\t.V[`B`@Qb\0\0E\x90b\0\x08\xE4V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0bW=`\0\x80>=`\0\xFD[Pa\x1C a\xC4\xE0`\0\x85\x80b\0\0\x8C`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01R\x90V[b\0\0\x99\x82`\0b\0\x01\xC0V[a\x01 Rb\0\0\xAA\x81`\x01b\0\x01\xC0V[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Rb\0\x018`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0R`\x02b\0\x01O\x82\x82b\0\nuV[Pb\0\x01]\x90P\x83b\0\x01\xF9V[b\0\x01h\x82b\0\x02:V[b\0\x01s\x81b\0\x02\xE1V[PPP`\x01`\x01`\xA0\x1B\x03\x16a\x01`Rb\0\x01\x8E\x81b\0\x03\"V[PPa\x01`Q`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x0B\xE7V[`\0` \x83Q\x10\x15b\0\x01\xE0Wb\0\x01\xD8\x83b\0\x04\xA1V[\x90Pb\0\x01\xF3V[\x81b\0\x01\xED\x84\x82b\0\nuV[P`\xFF\x90P[\x92\x91PPV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[`\0\x81\x11b\0\x02\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x07T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x07UV[`\x08T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x08UV[`d\x81\x11\x15b\0\x03\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01b\0\x02\x97V[`\0b\0\x03\xB3b\0\x04\xE4V[\x90P\x80\x15\x80\x15\x90b\0\x03\xC5WP`\x0BT\x15[\x15b\0\x04-W`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R`\x0B\x90` \x81\x01b\0\x03\xEB\x84b\0\x05\x13V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[b\0\x04bb\0\x04Mb\0\x04?b\0\x05\x82V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16b\0\x05\xF9V[b\0\x04X\x84b\0\x05\x13V[`\x0B\x91\x90b\0\x06`V[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15b\0\x04\xCFW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01b\0\x02\x97\x91\x90b\0\x0BAV[\x80Qb\0\x04\xDC\x82b\0\x0BvV[\x17\x93\x92PPPV[`\x0BT`\0\x90\x15b\0\x05\nWb\0\x04\xFC`\x0Bb\0\x06}V[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[`\nT[\x90P\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15b\0\x05~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01b\0\x02\x97V[P\x90V[`\0a\x01`Q`\x01`\x01`\xA0\x1B\x03\x16c\x91\xDD\xAD\xF4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15b\0\x05\xE4WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Rb\0\x05\xE1\x91\x81\x01\x90b\0\x0B\x9BV[`\x01[b\0\x05\xF4Wb\0\x05\x0ECb\0\x06\xCBV[\x91\x90PV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x05~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x02\x97V[`\0\x80b\0\x06p\x85\x85\x85b\0\x074V[\x91P\x91P[\x93P\x93\x91PPV[\x80T`\0\x90\x80\x15b\0\x06\xC1Wb\0\x06\xA9\x83b\0\x06\x9B`\x01\x84b\0\x0B\xC5V[`\0\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16b\0\x06\xC4V[`\0[\x93\x92PPPV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x05~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x02\x97V[\x82T`\0\x90\x81\x90\x80\x15b\0\x08\x89W`\0b\0\x07V\x87b\0\x06\x9B`\x01\x85b\0\x0B\xC5V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15b\0\x07\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01b\0\x02\x97V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03b\0\x08(W\x84b\0\x07\xFF\x88b\0\x06\x9B`\x01\x86b\0\x0B\xC5V[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x08xV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16d\x01\0\0\0\0\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pb\0\x06u\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16d\x01\0\0\0\0\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81b\0\x06uV[a/\x8F\x80b\0N\xA3\x839\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\t%W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\t\x0BV[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15b\0\tAW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\tYW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12b\0\tnW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\t\x83Wb\0\t\x83b\0\x08\xF2V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\t\xAEWb\0\t\xAEb\0\x08\xF2V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15b\0\t\xC8W`\0\x80\xFD[b\0\t\xDB\x83` \x83\x01` \x88\x01b\0\t\x08V[\x97\x96PPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\t\xFBW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\n\x1CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\npW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\nKWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\nlW\x82\x81U`\x01\x01b\0\nWV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\n\x91Wb\0\n\x91b\0\x08\xF2V[b\0\n\xA9\x81b\0\n\xA2\x84Tb\0\t\xE6V[\x84b\0\n\"V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\n\xE1W`\0\x84\x15b\0\n\xC8WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\nlV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x0B\x12W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\n\xF1V[P\x85\x82\x10\x15b\0\x0B1W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R`\0\x82Q\x80` \x84\x01Rb\0\x0Bb\x81`@\x85\x01` \x87\x01b\0\t\x08V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15b\0\n\x1CW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x0B\xAEW`\0\x80\xFD[\x81Qe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x06\xC4W`\0\x80\xFD[\x81\x81\x03\x81\x81\x11\x15b\0\x01\xF3WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`QaB3b\0\x0Cp`\09`\0\x81\x81a\t:\x01R\x81\x81a\x10\xCC\x01R\x81\x81a\x17\x9A\x01R\x81\x81a\x18\x8F\x01R\x81\x81a&<\x01Ra(\0\x01R`\0a\x17F\x01R`\0a\x17\x1C\x01R`\0a)a\x01R`\0a)9\x01R`\0a(\x94\x01R`\0a(\xBE\x01R`\0a(\xE8\x01RaB3`\0\xF3\xFE`\x80`@R`\x046\x10a\x02vW`\x005`\xE0\x1C\x80cp\xB0\xF6`\x11a\x01OW\x80c\xBC\xCFZ\xE4\x11a\0\xC1W\x80c\xEA\x02\x17\xCF\x11a\0zW\x80c\xEA\x02\x17\xCF\x14a\x08|W\x80c\xEB\x90\x19\xD4\x14a\x08\x9CW\x80c\xEC\xE4\x0C\xC1\x14a\x08\xBCW\x80c\xF2:na\x14a\x08\xDCW\x80c\xF8\xCEV\n\x14a\t\x08W\x80c\xFC\x0CTj\x14a\t(W`\0\x80\xFD[\x80c\xBC\xCFZ\xE4\x14a\x07vW\x80c\xC0\x1F\x9E7\x14a\x07\x96W\x80c\xC2\x8B\xC2\xFA\x14a\x07\xCFW\x80c\xC5\x90W\xE4\x14a\x07\xE2W\x80c\xDDN+\xA5\x14a\x08\x02W\x80c\xDE\xAA\xA7\xCC\x14a\x08HW`\0\x80\xFD[\x80c\x91\xDD\xAD\xF4\x11a\x01\x13W\x80c\x91\xDD\xAD\xF4\x14a\x06\xC0W\x80c\x97\xC3\xD34\x14a\x06\xECW\x80c\x9A\x80*m\x14a\x07\0W\x80c\xA7q:p\x14a\x07 W\x80c\xB5\x811\xB0\x14a\x075W\x80c\xBC\x19|\x81\x14a\x07JW`\0\x80\xFD[\x80cp\xB0\xF6`\x14a\x06\x13W\x80c{<q\xD3\x14a\x063W\x80c}^\x81\xE2\x14a\x06SW\x80c\x84\xB0\x19n\x14a\x06sW\x80c\x8Bd\xBB\x87\x14a\x06\x9BW`\0\x80\xFD[\x80c;\xCC\xF4\xFD\x11a\x01\xE8W\x80cK\xF5\xD7\xE9\x11a\x01\xACW\x80cK\xF5\xD7\xE9\x14a\x05\x1FW\x80cTO\xFC\x9C\x14a\x054W\x80cT\xFDMP\x14a\x05\x89W\x80cVx\x13\x88\x14a\x05\xB3W\x80c_9\x8A\x14\x14a\x05\xD3W\x80c`\xC4$\x7F\x14a\x05\xF3W`\0\x80\xFD[\x80c;\xCC\xF4\xFD\x14a\x04rW\x80c>OI\xE6\x14a\x04\x92W\x80cC\x85\x962\x14a\x04\xBFW\x80cC\x9F\xAB\x91\x14a\x04\xDFW\x80cE!\x15\xD6\x14a\x04\xFFW`\0\x80\xFD[\x80c\x144\x89\xD0\x11a\x02:W\x80c\x144\x89\xD0\x14a\x03GW\x80c\x15\x0Bz\x02\x14a\x03\x9CW\x80c&V\"}\x14a\x03\xE0W\x80c-c\xF6\x93\x14a\x03\xF3W\x80c/\xE3\xE2a\x14a\x04)W\x80c92\xAB\xB1\x14a\x04]W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02\x8DW\x80c\x02\xA2Q\xA3\x14a\x02\xC2W\x80c\x03B\x01\x81\x14a\x02\xE5W\x80c\x06\xF3\xF9\xE6\x14a\x03\x05W\x80c\x06\xFD\xDE\x03\x14a\x03%W`\0\x80\xFD[6a\x02\x88W\0[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xADa\x02\xA86`\x04a2\xC7V[a\t\\V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xCEW`\0\x80\xFD[Pa\x02\xD7a\t\xF9V[`@Q\x90\x81R` \x01a\x02\xB9V[4\x80\x15a\x02\xF1W`\0\x80\xFD[Pa\x02\xD7a\x03\x006`\x04a4\x15V[a\n\tV[4\x80\x15a\x03\x11W`\0\x80\xFD[Pa\x02\x86a\x03 6`\x04a4\xBBV[a\x0B\x01V[4\x80\x15a\x031W`\0\x80\xFD[Pa\x03:a\x0BCV[`@Qa\x02\xB9\x91\x90a5$V[4\x80\x15a\x03SW`\0\x80\xFD[Pa\x03\x84a\x03b6`\x04a4\xBBV[`\0\x90\x81R`\x03` R`@\x90 T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xB9V[4\x80\x15a\x03\xA8W`\0\x80\xFD[Pa\x03\xC7a\x03\xB76`\x04a5NV[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\xB9V[a\x02\xD7a\x03\xEE6`\x04a7\x1FV[a\x0B\xD5V[4\x80\x15a\x03\xFFW`\0\x80\xFD[Pa\x02\xD7a\x04\x0E6`\x04a4\xBBV[`\0\x90\x81R`\x03` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[4\x80\x15a\x045W`\0\x80\xFD[Pa\x02\xD7\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x81V[4\x80\x15a\x04iW`\0\x80\xFD[Pa\x02\xD7a\r\x01V[4\x80\x15a\x04~W`\0\x80\xFD[Pa\x02\xD7a\x04\x8D6`\x04a7\xAEV[a\r\x0CV[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x04\xB2a\x04\xAD6`\x04a4\xBBV[a\r\x82V[`@Qa\x02\xB9\x91\x90a8\x12V[4\x80\x15a\x04\xCBW`\0\x80\xFD[Pa\x02\xADa\x04\xDA6`\x04a8:V[a\x0E\xC2V[4\x80\x15a\x04\xEBW`\0\x80\xFD[Pa\x02\x86a\x04\xFA6`\x04a8fV[a\x0E\xF2V[4\x80\x15a\x05\x0BW`\0\x80\xFD[Pa\x02\xD7a\x05\x1A6`\x04a7\x1FV[a\x0F\xC1V[4\x80\x15a\x05+W`\0\x80\xFD[Pa\x03:a\x10\xC8V[4\x80\x15a\x05@W`\0\x80\xFD[Pa\x05na\x05O6`\x04a4\xBBV[`\0\x90\x81R`\t` R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xB9V[4\x80\x15a\x05\x95W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01Ra\x03:V[4\x80\x15a\x05\xBFW`\0\x80\xFD[Pa\x02\xD7a\x05\xCE6`\x04a8\xA7V[a\x11\x8AV[4\x80\x15a\x05\xDFW`\0\x80\xFD[Pa\x02\xD7a\x05\xEE6`\x04a8\xCAV[a\x11\xB3V[4\x80\x15a\x05\xFFW`\0\x80\xFD[Pa\x02\xD7a\x06\x0E6`\x04a4\xBBV[a\x11\xFDV[4\x80\x15a\x06\x1FW`\0\x80\xFD[Pa\x02\x86a\x06.6`\x04a4\xBBV[a\x12\xAEV[4\x80\x15a\x06?W`\0\x80\xFD[Pa\x02\xD7a\x06N6`\x04a9MV[a\x12\xEDV[4\x80\x15a\x06_W`\0\x80\xFD[Pa\x02\xD7a\x06n6`\x04a9\xA6V[a\x135V[4\x80\x15a\x06\x7FW`\0\x80\xFD[Pa\x06\x88a\x17\x0EV[`@Qa\x02\xB9\x97\x96\x95\x94\x93\x92\x91\x90a:\x95V[4\x80\x15a\x06\xA7W`\0\x80\xFD[P`\x0CTa\x03\x84\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xCCW`\0\x80\xFD[Pa\x06\xD5a\x17\x96V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xB9V[4\x80\x15a\x06\xF8W`\0\x80\xFD[P`da\x02\xD7V[4\x80\x15a\x07\x0CW`\0\x80\xFD[Pa\x02\xD7a\x07\x1B6`\x04a:\xF7V[a\x18\x1FV[4\x80\x15a\x07,W`\0\x80\xFD[Pa\x02\xD7a\x186V[4\x80\x15a\x07AW`\0\x80\xFD[Pa\x02\xD7a\x18`V[4\x80\x15a\x07VW`\0\x80\xFD[Pa\x03\xC7a\x07e6`\x04a;MV[c\xBC\x19|\x81`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x07\x82W`\0\x80\xFD[Pa\x02\x86a\x07\x916`\x04a;\xDCV[a\x18kV[4\x80\x15a\x07\xA2W`\0\x80\xFD[Pa\x02\xD7a\x07\xB16`\x04a4\xBBV[`\0\x90\x81R`\x03` R`@\x90 `\x01\x01T`\x01`\x01`@\x1B\x03\x16\x90V[a\x02\x86a\x07\xDD6`\x04a<;V[a\x19\xFCV[4\x80\x15a\x07\xEEW`\0\x80\xFD[Pa\x02\xD7a\x07\xFD6`\x04a7\x1FV[a\x1A\xC3V[4\x80\x15a\x08\x0EW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R` \x80\x82R\x7Fsupport=bravo&quorum=for,abstain\x90\x82\x01Ra\x03:V[4\x80\x15a\x08TW`\0\x80\xFD[Pa\x02\xD7\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F\x81V[4\x80\x15a\x08\x88W`\0\x80\xFD[Pa\x02\x86a\x08\x976`\x04a4\xBBV[a\x1A\xFDV[4\x80\x15a\x08\xA8W`\0\x80\xFD[Pa\x02\xD7a\x08\xB76`\x04a<|V[a\x1B<V[4\x80\x15a\x08\xC8W`\0\x80\xFD[Pa\x02\x86a\x08\xD76`\x04a4\xBBV[a\x1B]V[4\x80\x15a\x08\xE8W`\0\x80\xFD[Pa\x03\xC7a\x08\xF76`\x04a<\xA6V[c\xF2:na`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\t\x14W`\0\x80\xFD[Pa\x02\xD7a\t#6`\x04a4\xBBV[a\x1B\x9CV[4\x80\x15a\t4W`\0\x80\xFD[Pa\x03\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0c(\x8A\xCE\x03`\xE1\x1Bc\x18\xDFt?`\xE3\x1Bc\xBF&\xD8\x97`\xE0\x1Bcy\xDDyo`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x82\x14\x80a\t\xA2WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x81\x16\x90\x82\x16\x14[\x80a\t\xB9WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x81\x16\x90\x85\x16\x14[\x80a\t\xD4WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x16c\x02q\x18\x97`\xE5\x1B\x14[\x80a\t\xEFWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x87\x16\x14[\x96\x95PPPPPPV[`\0a\n\x04`\x07T\x90V[\x90P\x90V[`\0\x80a\n\xADa\n\xA5\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x8C\x8C\x8C\x8C`@Qa\nE\x92\x91\x90a=\nV[`@Q\x80\x91\x03\x90 \x8B\x80Q\x90` \x01 `@Q` \x01a\n\x8A\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R`\xFF\x91\x90\x91\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1B\xA7V[\x86\x86\x86a\x1B\xD4V[\x90Pa\n\xF3\x8A\x82\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8D\x92Pa\x1B\xF2\x91PPV[\x9A\x99PPPPPPPPPPV[30\x14a\x0B W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x0B7V[\x80a\x0B0`\x04a\x1DGV[\x03a\x0B%WP[a\x0B@\x81a\x1D\xC6V[PV[```\x02\x80Ta\x0BR\x90a=QV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B~\x90a=QV[\x80\x15a\x0B\xCBW\x80`\x1F\x10a\x0B\xA0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xCBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xAEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80a\x0B\xE4\x86\x86\x86\x86a\x1A\xC3V[\x90P`\0a\x0B\xF1\x82a\r\x82V[\x90P`\x04\x81`\x07\x81\x11\x15a\x0C\x07Wa\x0C\x07a7\xFCV[\x14\x80a\x0C$WP`\x05\x81`\x07\x81\x11\x15a\x0C\"Wa\x0C\"a7\xFCV[\x14[a\x0CzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal not successfu`D\x82\x01R`\x1B`\xFA\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0\x82\x81R`\x03` R`@\x90\x81\x90 `\x02\x01\x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fq*\xE18?y\xAC\x85?\x8D\x88!Sw\x8E\x02`\xEF\x8F\x03\xB5\x04\xE2\x86n\x05\x93\xE0M+)\x1F\x90a\x0C\xC8\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x0C\xDD\x82\x88\x88\x88\x88a\x1F4V[a\x0C\xEA\x82\x88\x88\x88\x88a\x1F\xBFV[a\x0C\xF7\x82\x88\x88\x88\x88a\x1F4V[P\x95\x94PPPPPV[`\0a\n\x04`\x06T\x90V[`@\x80Q\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F` \x82\x01R\x90\x81\x01\x86\x90R`\xFF\x85\x16``\x82\x01R`\0\x90\x81\x90a\rZ\x90a\n\xA5\x90`\x80\x01a\n\x8AV[\x90Pa\rw\x87\x82\x88`@Q\x80` \x01`@R\x80`\0\x81RPa \xB5V[\x97\x96PPPPPPPV[`\0\x81\x81R`\x03` R`@\x81 `\x02\x81\x01T`\xFF\x16\x15a\r\xA6WP`\x07\x92\x91PPV[`\x02\x81\x01Ta\x01\0\x90\x04`\xFF\x16\x15a\r\xC1WP`\x02\x92\x91PPV[`\0\x83\x81R`\x03` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90\x03a\x0E)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: unknown proposal id\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0a\x0E3a\x17\x96V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x82\x10a\x0EOWP`\0\x94\x93PPPPV[`\0\x85\x81R`\x03` R`@\x90 `\x01\x01T`\x01`\x01`@\x1B\x03\x16\x81\x81\x10a\x0E}WP`\x01\x95\x94PPPPPV[a\x0E\x86\x86a \xE1V[\x80\x15a\x0E\xA5WP`\0\x86\x81R`\t` R`@\x90 \x80T`\x01\x90\x91\x01T\x11[\x15a\x0E\xB6WP`\x04\x95\x94PPPPPV[P`\x03\x95\x94PPPPPV[`\0\x82\x81R`\t` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R`\x03\x01\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[`\x0CT`\xFF\x16\x15a\x0FEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FProject: already initialized\0\0\0\0`D\x82\x01R`d\x01a\x02}V[`\x0C\x80T`\x01`\xFF\x19\x90\x91\x16\x17\x90\x81\x90U`@Qc<-\xD8\x1F`\xE1\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cx[\xB0>\x90a\x0F\x8B\x903\x90\x86\x90\x86\x90`\x04\x01a=\x8BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xB9W=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x80a\x0F\xD0\x86\x86\x86\x86a\x1A\xC3V[\x90P`\0a\x0F\xDD\x82a\r\x82V[`\x07\x81\x11\x15a\x0F\xEEWa\x0F\xEEa7\xFCV[\x14a\x10;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FGovernor: too late to cancel\0\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FGovernor: only proposer can canc`D\x82\x01Ra\x19[`\xF2\x1B`d\x82\x01R`\x84\x01a\x02}V[a\t\xEF\x86\x86\x86\x86a!-V[``\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cK\xF5\xD7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x11IWP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11F\x91\x90\x81\x01\x90a=\xCBV[`\x01[a\x11\x85WP`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R\x90V[\x91\x90PV[`\0\x803\x90Pa\x11\xAB\x84\x82\x85`@Q\x80` \x01`@R\x80`\0\x81RPa \xB5V[\x94\x93PPPPV[`\0\x803\x90Pa\rw\x87\x82\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92Pa\x1B\xF2\x91PPV[`\x0BT`\0\x90\x80\x82\x03a\x12\x14WPP`\nT\x91\x90PV[`\0`\x0Ba\x12#`\x01\x84a>NV[\x81T\x81\x10a\x123Wa\x123a>aV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`\x01` \x1B\x90\x91\x04`\x01`\x01`\xE0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R\x91P\x84\x10a\x12\x89W` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x12\x9Da\x12\x95\x85a\"KV[`\x0B\x90a\"\xB4V[`\x01`\x01`\xE0\x1B\x03\x16\x94\x93PPPPV[30\x14a\x12\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x12\xE4V[\x80a\x12\xDD`\x04a\x1DGV[\x03a\x12\xD2WP[a\x0B@\x81a#gV[`\0\x803\x90Pa\t\xEF\x86\x82\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa \xB5\x92PPPV[`\x003a\x13B\x81\x84a#\xA8V[a\x13\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: proposer restricted\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0a\x13\x98a\x17\x96V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x13\xAAa\x18`V[a\x13\xB9\x83a\x08\xB7`\x01\x85a>NV[\x10\x15a\x14!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FGovernor: proposer votes below p`D\x82\x01Rp\x1C\x9B\xDC\x1B\xDC\xD8[\x08\x1D\x1A\x1C\x99\\\xDA\x1B\xDB\x19`z\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0a\x146\x88\x88\x88\x88\x80Q\x90` \x01 a\x1A\xC3V[\x90P\x86Q\x88Q\x14a\x14YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a>wV[\x85Q\x88Q\x14a\x14zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a>wV[`\0\x88Q\x11a\x14\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FGovernor: empty proposal\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15a\x15:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal already exist`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0a\x15Da\r\x01V[a\x15N\x90\x84a>\xB8V[\x90P`\0a\x15Za\t\xF9V[a\x15d\x90\x83a>\xB8V[\x90P`@Q\x80`\xE0\x01`@R\x80a\x15z\x84a$\x99V[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R`\0`@\x82\x01R``\x01a\x15\xA7\x83a$\x99V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\0` \x80\x84\x01\x82\x90R`@\x80\x85\x01\x83\x90R``\x94\x85\x01\x83\x90R\x88\x83R`\x03\x82R\x91\x82\x90 \x85Q\x81T\x92\x87\x01Q\x87\x85\x01Q\x91\x86\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84\x02\x17`\x01`\x01`\xE0\x1B\x03\x16`\x01`\xE0\x1B`\xE0\x92\x90\x92\x1C\x91\x90\x91\x02\x17\x81U\x93\x85\x01Q`\x80\x86\x01Q\x90\x84\x16\x92\x1C\x02\x17`\x01\x83\x01U`\xA0\x83\x01Q`\x02\x90\x92\x01\x80T`\xC0\x90\x94\x01Qa\xFF\xFF\x19\x90\x94\x16\x92\x15\x15a\xFF\0\x19\x16\x92\x90\x92\x17a\x01\0\x93\x15\x15\x93\x90\x93\x02\x92\x90\x92\x17\x90U\x8AQ\x7F}\x84\xA6&:\xE0\xD9\x8D3)\xBD{F\xBBN\x8Do\x98\xCD5\xA7\xAD\xB4\\'L\x8B\x7F\xD5\xEB\xD5\xE0\x91\x85\x91\x88\x91\x8E\x91\x8E\x91\x81\x11\x15a\x16\xABWa\x16\xABa3JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xDEW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\xC9W\x90P[P\x8D\x88\x88\x8F`@Qa\x16\xF8\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a?YV[`@Q\x80\x91\x03\x90\xA1P\x90\x98\x97PPPPPPPPV[`\0``\x80\x82\x80\x80\x83a\x17A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a%\x01V[a\x17l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a%\x01V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x91\xDD\xAD\xF4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x18\x12WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x18\x0F\x91\x81\x01\x90a@0V[`\x01[a\x11\x85Wa\n\x04Ca%\xACV[`\0a\x18,\x84\x84\x84a&\x13V[\x90P[\x93\x92PPPV[`\x0BT`\0\x90\x15a\x18YWa\x18K`\x0Ba&\xA9V[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[P`\nT\x90V[`\0a\n\x04`\x08T\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x18\xC1Wa\x18\xC1a>aV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x19\x13Wa\x19\x13a>aV[` \x02` \x01\x01\x81\x81RPP`\x003\x87\x87`@Q`$\x01a\x196\x93\x92\x91\x90a=\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c<-\xD8\x1F`\xE1\x1B\x17\x90R\x81Q`\x01\x80\x82R\x81\x84\x01\x90\x93R\x92\x93P`\0\x92\x91\x90\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19yW\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x19\xA4Wa\x19\xA4a>aV[` \x02` \x01\x01\x81\x90RPa\x19\xF1\x84\x84\x83\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x135\x92PPPV[PPPPPPPPPV[30\x14a\x1A\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x1A2V[\x80a\x1A+`\x04a\x1DGV[\x03a\x1A WP[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85\x85`@Qa\x1AP\x92\x91\x90a=\nV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1A\x8DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A\x92V[``\x91P[P\x91P\x91Pa\x1A\xBA\x82\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01aA\xAF`(\x919a&\xE2V[PPPPPPPV[`\0\x84\x84\x84\x84`@Q` \x01a\x1A\xDC\x94\x93\x92\x91\x90a@XV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[30\x14a\x1B\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x1B3V[\x80a\x1B,`\x04a\x1DGV[\x03a\x1B!WP[a\x0B@\x81a&\xFBV[`\0a\x18/\x83\x83a\x1BX`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a&\x13V[30\x14a\x1B|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x1B\x93V[\x80a\x1B\x8C`\x04a\x1DGV[\x03a\x1B\x81WP[a\x0B@\x81a'\x9CV[`\0a\x0E\xEC\x82a'\xDDV[`\0a\x0E\xECa\x1B\xB4a(\x87V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x1B\xE5\x87\x87\x87\x87a)\xB2V[\x91P\x91Pa\x0C\xF7\x81a*vV[`\0\x85\x81R`\x03` R`@\x81 `\x01a\x1C\x0B\x88a\r\x82V[`\x07\x81\x11\x15a\x1C\x1CWa\x1C\x1Ca7\xFCV[\x14a\x1CuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FGovernor: vote not currently act`D\x82\x01Rbive`\xE8\x1B`d\x82\x01R`\x84\x01a\x02}V[\x80T`\0\x90a\x1C\x8F\x90\x88\x90`\x01`\x01`@\x1B\x03\x16\x86a&\x13V[\x90Pa\x1C\x9E\x88\x88\x88\x84\x88a+\xC0V[\x83Q`\0\x03a\x1C\xF3W\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB8\xE18\x88}\n\xA1;\xABD~\x82\xDE\x9D\\\x17w\x04\x1E\xCD!\xCA6\xBA\x82O\xF1\xE6\xC0}\xDD\xA4\x89\x88\x84\x89`@Qa\x1C\xE6\x94\x93\x92\x91\x90a@\xA3V[`@Q\x80\x91\x03\x90\xA2a\rwV[\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE2\xBA\xBF\xBA\xC5\x88\x9Ap\x9Bc\xBB\x7FY\x8B2N\x08\xBCZO\xB9\xECd\x7F\xB3\xCB\xC9\xEC\x07\xEB\x87\x12\x89\x88\x84\x89\x89`@Qa\x1D4\x95\x94\x93\x92\x91\x90a@\xCBV[`@Q\x80\x91\x03\x90\xA2\x97\x96PPPPPPPV[`\0a\x1Db\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a\x1D\x80W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`d\x81\x11\x15a\x1EIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x02}V[`\0a\x1ESa\x186V[\x90P\x80\x15\x80\x15\x90a\x1EdWP`\x0BT\x15[\x15a\x1E\xC8W`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R`\x0B\x90` \x81\x01a\x1E\x87\x84a-:V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[a\x1E\xF5a\x1E\xE3a\x1E\xD6a\x17\x96V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\"KV[a\x1E\xEC\x84a-:V[`\x0B\x91\x90a-\xA3V[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x1F\xB8V[\x84Q\x81\x10\x15a\x0F\xB9W0`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\x1F^Wa\x1F^a>aV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1F\xA8Wa\x1F\xA8\x83\x82\x81Q\x81\x10a\x1F\x89Wa\x1F\x89a>aV[` \x02` \x01\x01Q\x80Q\x90` \x01 `\x04a-\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1F\xB1\x81aA\x11V[\x90Pa\x1F9V[PPPPPV[`\0`@Q\x80``\x01`@R\x80`'\x81R` \x01aA\xD7`'\x919\x90P`\0[\x85Q\x81\x10\x15a\x1A\xBAW`\0\x80\x87\x83\x81Q\x81\x10a\x1F\xFDWa\x1F\xFDa>aV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x87\x84\x81Q\x81\x10a  Wa  a>aV[` \x02` \x01\x01Q\x87\x85\x81Q\x81\x10a :Wa :a>aV[` \x02` \x01\x01Q`@Qa O\x91\x90aA*V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a \x8CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a \x91V[``\x91P[P\x91P\x91Pa \xA1\x82\x82\x86a&\xE2V[PPP\x80a \xAE\x90aA\x11V[\x90Pa\x1F\xDFV[`\0a \xD8\x85\x85\x85\x85a \xD3`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a\x1B\xF2V[\x95\x94PPPPPV[`\0\x81\x81R`\t` R`@\x81 `\x02\x81\x01T`\x01\x82\x01Ta!\x03\x91\x90a>\xB8V[`\0\x84\x81R`\x03` R`@\x90 Ta!$\x90`\x01`\x01`@\x1B\x03\x16a\x1B\x9CV[\x11\x15\x93\x92PPPV[`\0\x80a!<\x86\x86\x86\x86a\x1A\xC3V[\x90P`\0a!I\x82a\r\x82V[\x90P`\x02\x81`\x07\x81\x11\x15a!_Wa!_a7\xFCV[\x14\x15\x80\x15a!\x7FWP`\x06\x81`\x07\x81\x11\x15a!|Wa!|a7\xFCV[\x14\x15[\x80\x15a!\x9DWP`\x07\x81`\x07\x81\x11\x15a!\x9AWa!\x9Aa7\xFCV[\x14\x15[a!\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: proposal not active\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0\x82\x81R`\x03` R`@\x90\x81\x90 `\x02\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UQ\x7Fx\x9C\xF5[\xE9\x80s\x9D\xAD\x1D\x06\x99\xB9;X\xE8\x06\xB5\x1C\x9D\x96a\x9B\xFA\x8F\xE0\xA2\x8A\xBA\xA7\xB3\x0C\x90a\"9\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1P\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\"\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02}V[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a#\x11W`\0a\"\xCF\x84a-\xFAV[a\"\xD9\x90\x85a>NV[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a#\x01W\x80\x91Pa#\x0FV[a#\x0C\x81`\x01a>\xB8V[\x92P[P[`\0a#\x1F\x87\x87\x85\x85a.\xE2V[\x90P\x80\x15a#ZWa#D\x87a#6`\x01\x84a>NV[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\rwV[`\0\x97\x96PPPPPPPV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[\x80Q`\0\x90`4\x81\x10\x15a#\xC0W`\x01\x91PPa\x0E\xECV[\x82\x81\x01`\x13\x19\x01Q`\x01`\x01`\xA0\x1B\x03\x19\x81\x16k\x04n\x0EM\xEE\r\xEEl\xAEG\xA6\x0F`\xA3\x1B\x14a#\xF3W`\x01\x92PPPa\x0E\xECV[`\0\x80a$\x01`(\x85a>NV[\x90P[\x83\x81\x10\x15a$xW`\0\x80a$8\x88\x84\x81Q\x81\x10a$$Wa$$a>aV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16a/@V[\x91P\x91P\x81a$PW`\x01\x96PPPPPPPa\x0E\xECV[\x80`\xFF\x16`\x04\x85`\x01`\x01`\xA0\x1B\x03\x16\x90\x1B\x17\x93PPP\x80a$q\x90aA\x11V[\x90Pa$\x04V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x93PPPP\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\"\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 6`D\x82\x01Re4 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02}V[```\xFF\x83\x14a%\x1BWa%\x14\x83a/\xD2V[\x90Pa\x0E\xECV[\x81\x80Ta%'\x90a=QV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%S\x90a=QV[\x80\x15a%\xA0W\x80`\x1F\x10a%uWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\xA0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%\x83W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x0E\xECV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\"\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02}V[`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c:F\xB1\xA8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18,\x91\x90aAFV[\x80T`\0\x90\x80\x15a&\xD9Wa&\xC3\x83a#6`\x01\x84a>NV[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x18/V[`\0\x93\x92PPPV[``\x83\x15a&\xF1WP\x81a\x18/V[a\x18/\x83\x83a0\x11V[`\0\x81\x11a'[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01a\x02}V[`\x07T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x07UV[`\x08T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x08UV[`\0`da'\xEA\x83a\x11\xFDV[`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8ES\x9E\x8C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(s\x91\x90aAFV[a(}\x91\x90aA_V[a\x0E\xEC\x91\x90aA\x8CV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a(\xE0WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a)\nWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\n\x04`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a)\xE9WP`\0\x90P`\x03a*mV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a*=W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a*fW`\0`\x01\x92P\x92PPa*mV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a*\x8AWa*\x8Aa7\xFCV[\x03a*\x92WPV[`\x01\x81`\x04\x81\x11\x15a*\xA6Wa*\xA6a7\xFCV[\x03a*\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02}V[`\x02\x81`\x04\x81\x11\x15a+\x07Wa+\x07a7\xFCV[\x03a+TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02}V[`\x03\x81`\x04\x81\x11\x15a+hWa+ha7\xFCV[\x03a\x0B@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0\x85\x81R`\t` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R`\x03\x81\x01\x90\x92R\x90\x91 T`\xFF\x16\x15a,HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorVotingSimple: vote alrea`D\x82\x01Rf\x19\x1EH\x18\xD8\\\xDD`\xCA\x1B`d\x82\x01R`\x84\x01a\x02}V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\xFF\x84\x16a,\x94W\x82\x81`\0\x01`\0\x82\x82Ta,\x89\x91\x90a>\xB8V[\x90\x91UPa\x0F\xB9\x90PV[`\0\x19`\xFF\x85\x16\x01a,\xB4W\x82\x81`\x01\x01`\0\x82\x82Ta,\x89\x91\x90a>\xB8V[`\x01\x19`\xFF\x85\x16\x01a,\xD4W\x82\x81`\x02\x01`\0\x82\x82Ta,\x89\x91\x90a>\xB8V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FGovernorVotingSimple: invalid va`D\x82\x01Rtlue for enum VoteType`X\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\"\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0\x80a-\xB1\x85\x85\x85a0;V[\x91P\x91P[\x93P\x93\x91PPV[\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B`\0\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[`\0\x81`\0\x03a.\x0CWP`\0\x91\x90PV[`\0`\x01a.\x19\x84a1\xDAV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a.2Wa.2aAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.JWa.JaAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.bWa.baAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.zWa.zaAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.\x92Wa.\x92aAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.\xAAWa.\xAAaAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.\xC2Wa.\xC2aAvV[\x04\x82\x01\x90\x1C\x90Pa\x18/\x81\x82\x85\x81a.\xDCWa.\xDCaAvV[\x04a2nV[`\0[\x81\x83\x10\x15a/8W`\0a.\xF9\x84\x84a2\x84V[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a/$W\x80\x92Pa/2V[a//\x81`\x01a>\xB8V[\x93P[Pa.\xE5V[P\x93\x92PPPV[`\0\x80`\xF8\x83\x90\x1C`/\x81\x11\x80\x15a/[WP`:\x81`\xFF\x16\x10[\x15a/pW`\x01\x94`/\x19\x90\x91\x01\x93P\x91PPV[\x80`\xFF\x16`@\x10\x80\x15a/\x86WP`G\x81`\xFF\x16\x10[\x15a/\x9BW`\x01\x94`6\x19\x90\x91\x01\x93P\x91PPV[\x80`\xFF\x16``\x10\x80\x15a/\xB1WP`g\x81`\xFF\x16\x10[\x15a/\xC6W`\x01\x94`V\x19\x90\x91\x01\x93P\x91PPV[P`\0\x93\x84\x93P\x91PPV[```\0a/\xDF\x83a2\x9FV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[\x81Q\x15a0!W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x91\x90a5$V[\x82T`\0\x90\x81\x90\x80\x15a1\x81W`\0a0Y\x87a#6`\x01\x85a>NV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a0\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01a\x02}V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a1\"W\x84a0\xFB\x88a#6`\x01\x86a>NV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua1qV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01` \x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa-\xB6\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01` \x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a-\xB6V[`\0\x80`\x80\x83\x90\x1C\x15a1\xEFW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a2\x01W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a2\x13W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a2%W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a27W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a2IW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a2[W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x0E\xECW`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a2}W\x81a\x18/V[P\x90\x91\x90PV[`\0a2\x93`\x02\x84\x84\x18aA\x8CV[a\x18/\x90\x84\x84\x16a>\xB8V[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x0E\xECW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a2\xD9W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x18/W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a\x11\x85W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a3\x14W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a3+W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a3CW`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a3\x88Wa3\x88a3JV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a3\xA9Wa3\xA9a3JV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a3\xCAa3\xC5\x84a3\x90V[a3`V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a3\xDEW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a4\x06W`\0\x80\xFD[a\x18/\x83\x835` \x85\x01a3\xB7V[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15a41W`\0\x80\xFD[\x885\x97Pa4A` \x8A\x01a2\xF1V[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a4]W`\0\x80\xFD[a4i\x8C\x83\x8D\x01a3\x02V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a4\x82W`\0\x80\xFD[Pa4\x8F\x8B\x82\x8C\x01a3\xF5V[\x94PPa4\x9E`\x80\x8A\x01a2\xF1V[\x92P`\xA0\x89\x015\x91P`\xC0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a4\xCDW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a4\xEFW\x81\x81\x01Q\x83\x82\x01R` \x01a4\xD7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra5\x10\x81` \x86\x01` \x86\x01a4\xD4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x18/` \x83\x01\x84a4\xF8V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\x85W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a5dW`\0\x80\xFD[a5m\x85a57V[\x93Pa5{` \x86\x01a57V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x9DW`\0\x80\xFD[a5\xA9\x87\x82\x88\x01a3\xF5V[\x91PP\x92\x95\x91\x94P\x92PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a5\xCEWa5\xCEa3JV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a5\xE9W`\0\x80\xFD[\x815` a5\xF9a3\xC5\x83a5\xB5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a6\x18W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a6:Wa6-\x81a57V[\x83R\x91\x83\x01\x91\x83\x01a6\x1CV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a6VW`\0\x80\xFD[\x815` a6fa3\xC5\x83a5\xB5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a6\x85W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a6:W\x805\x83R\x91\x83\x01\x91\x83\x01a6\x89V[`\0\x82`\x1F\x83\x01\x12a6\xB1W`\0\x80\xFD[\x815` a6\xC1a3\xC5\x83a5\xB5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a6\xE0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a6:W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x03W`\0\x80\x81\xFD[a7\x11\x89\x86\x83\x8B\x01\x01a3\xF5V[\x84RP\x91\x83\x01\x91\x83\x01a6\xE4V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a75W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7LW`\0\x80\xFD[a7X\x88\x83\x89\x01a5\xD8V[\x95P` \x87\x015\x91P\x80\x82\x11\x15a7nW`\0\x80\xFD[a7z\x88\x83\x89\x01a6EV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a7\x90W`\0\x80\xFD[Pa7\x9D\x87\x82\x88\x01a6\xA0V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a7\xC6W`\0\x80\xFD[\x855\x94Pa7\xD6` \x87\x01a2\xF1V[\x93Pa7\xE4`@\x87\x01a2\xF1V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x08\x83\x10a84WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`@\x83\x85\x03\x12\x15a8MW`\0\x80\xFD[\x825\x91Pa8]` \x84\x01a57V[\x90P\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a8yW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a8\x8FW`\0\x80\xFD[a8\x9B\x85\x82\x86\x01a3\x02V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a8\xBAW`\0\x80\xFD[\x825\x91Pa8]` \x84\x01a2\xF1V[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a8\xE2W`\0\x80\xFD[\x855\x94Pa8\xF2` \x87\x01a2\xF1V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a9\x0EW`\0\x80\xFD[a9\x1A\x89\x83\x8A\x01a3\x02V[\x90\x95P\x93P``\x88\x015\x91P\x80\x82\x11\x15a93W`\0\x80\xFD[Pa9@\x88\x82\x89\x01a3\xF5V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a9cW`\0\x80\xFD[\x845\x93Pa9s` \x86\x01a2\xF1V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x8EW`\0\x80\xFD[a9\x9A\x87\x82\x88\x01a3\x02V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a9\xBCW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a9\xD3W`\0\x80\xFD[a9\xDF\x88\x83\x89\x01a5\xD8V[\x95P` \x87\x015\x91P\x80\x82\x11\x15a9\xF5W`\0\x80\xFD[a:\x01\x88\x83\x89\x01a6EV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a:\x17W`\0\x80\xFD[a:#\x88\x83\x89\x01a6\xA0V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a:9W`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a:KW`\0\x80\xFD[a5\xA9\x87\x825` \x84\x01a3\xB7V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a:\x8AW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a:nV[P\x94\x95\x94PPPPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a:\xB4`\xE0\x83\x01\x89a4\xF8V[\x82\x81\x03`@\x84\x01Ra:\xC6\x81\x89a4\xF8V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x90Pa\n\xF3\x81\x85a:ZV[`\0\x80`\0``\x84\x86\x03\x12\x15a;\x0CW`\0\x80\xFD[a;\x15\x84a57V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a;7W`\0\x80\xFD[a;C\x86\x82\x87\x01a3\xF5V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a;eW`\0\x80\xFD[a;n\x86a57V[\x94Pa;|` \x87\x01a57V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\x98W`\0\x80\xFD[a;\xA4\x89\x83\x8A\x01a6EV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a;\xBAW`\0\x80\xFD[a;\xC6\x89\x83\x8A\x01a6EV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a93W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a;\xF2W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\tW`\0\x80\xFD[a<\x15\x88\x83\x89\x01a3\x02V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a<.W`\0\x80\xFD[Pa9\x9A\x87\x82\x88\x01a3\x02V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a<QW`\0\x80\xFD[a<Z\x85a57V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x8EW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a<\x8FW`\0\x80\xFD[a<\x98\x83a57V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a<\xBEW`\0\x80\xFD[a<\xC7\x86a57V[\x94Pa<\xD5` \x87\x01a57V[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xFEW`\0\x80\xFD[a9@\x88\x82\x89\x01a3\xF5V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x80\x82R`\x18\x90\x82\x01R\x7FGovernor: onlyGovernance\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a=eW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a=\x85WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a=\xDDW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a=\xF3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a>\x04W`\0\x80\xFD[\x80Qa>\x12a3\xC5\x82a3\x90V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a>'W`\0\x80\xFD[a \xD8\x82` \x83\x01` \x86\x01a4\xD4V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\xECWa\x0E\xECa>8V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`!\x90\x82\x01R\x7FGovernor: invalid proposal lengt`@\x82\x01R`\r`\xFB\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0E\xECWa\x0E\xECa>8V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a:\x8AW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a>\xDFV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15a?LW\x82\x84\x03\x89Ra?:\x84\x83Qa4\xF8V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01a?\"V[P\x91\x97\x96PPPPPPPV[`\0a\x01 \x8B\x83R` `\x01\x80`\xA0\x1B\x03\x8C\x16\x81\x85\x01R\x81`@\x85\x01Ra?\x82\x82\x85\x01\x8Ca>\xCBV[\x91P\x83\x82\x03``\x85\x01Ra?\x96\x82\x8Ba:ZV[\x91P\x83\x82\x03`\x80\x85\x01R\x81\x89Q\x80\x84R\x82\x84\x01\x91P\x82\x81`\x05\x1B\x85\x01\x01\x83\x8C\x01`\0[\x83\x81\x10\x15a?\xE7W`\x1F\x19\x87\x84\x03\x01\x85Ra?\xD5\x83\x83Qa4\xF8V[\x94\x86\x01\x94\x92P\x90\x85\x01\x90`\x01\x01a?\xB9V[PP\x86\x81\x03`\xA0\x88\x01Ra?\xFB\x81\x8Ca?\x04V[\x94PPPPP\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01Ra@ \x81\x85a4\xF8V[\x9C\x9BPPPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a@BW`\0\x80\xFD[\x81Qe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18/W`\0\x80\xFD[`\x80\x81R`\0a@k`\x80\x83\x01\x87a>\xCBV[\x82\x81\x03` \x84\x01Ra@}\x81\x87a:ZV[\x90P\x82\x81\x03`@\x84\x01Ra@\x91\x81\x86a?\x04V[\x91PP\x82``\x83\x01R\x95\x94PPPPPV[\x84\x81R`\xFF\x84\x16` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\t\xEF`\x80\x83\x01\x84a4\xF8V[\x85\x81R`\xFF\x85\x16` \x82\x01R\x83`@\x82\x01R`\xA0``\x82\x01R`\0a@\xF3`\xA0\x83\x01\x85a4\xF8V[\x82\x81\x03`\x80\x84\x01RaA\x05\x81\x85a4\xF8V[\x98\x97PPPPPPPPV[`\0`\x01\x82\x01aA#WaA#a>8V[P`\x01\x01\x90V[`\0\x82QaA<\x81\x84` \x87\x01a4\xD4V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aAXW`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E\xECWa\x0E\xECa>8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aA\xA9WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFEGovernor: relay reverted without messageGovernor: call reverted without message\xA2dipfsX\"\x12 \"\xC0=~\xB3\xB2\xD3^+\xD8\xCA\xE75\x0E)\xABV\xBA\x14\xC7e2$#\x10\xA7\xC0\"P\xACubdsolcC\0\x08\x14\x003a\x01``@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\x0C\x80\x82Rk!\xB7\xB7:94\xB1:\xBA4\xB7\xB7`\xA1\x1B` \x80\x84\x01\x82\x90R\x84Q\x80\x86\x01\x86R`\x01\x81R`1`\xF8\x1B\x81\x83\x01R\x85Q\x80\x87\x01\x87R\x93\x84R\x83\x82\x01\x92\x90\x92R\x84Q\x80\x86\x01\x90\x95R`\x04\x85Rc!\xA7*)`\xE1\x1B\x90\x85\x01R\x91\x92`\0b\0\0\x86\x83\x82b\0\x02\xEDV[P`\x01b\0\0\x95\x82\x82b\0\x02\xEDV[PPPb\0\0\xB2b\0\0\xACb\0\x01m` \x1B` \x1CV[b\0\x01qV[b\0\0\xBF\x82`\x07b\0\x01\xC3V[a\x01 Rb\0\0\xD0\x81`\x08b\0\x01\xC3V[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Rb\0\x01^`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0Rb\0\x04.V[3\x90V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0` \x83Q\x10\x15b\0\x01\xE3Wb\0\x01\xDB\x83b\0\x01\xFCV[\x90Pb\0\x01\xF6V[\x81b\0\x01\xF0\x84\x82b\0\x02\xEDV[P`\xFF\x90P[\x92\x91PPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15b\0\x023W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01b\0\x02*\x91\x90b\0\x03\xB9V[`@Q\x80\x91\x03\x90\xFD[\x80Qb\0\x02@\x82b\0\x04\tV[\x17\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02sW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x02\x94WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\xE8W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\xC3WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\xE4W\x82\x81U`\x01\x01b\0\x02\xCFV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\tWb\0\x03\tb\0\x02HV[b\0\x03!\x81b\0\x03\x1A\x84Tb\0\x02^V[\x84b\0\x02\x9AV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03YW`\0\x84\x15b\0\x03@WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\xE4V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03\x8AW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03iV[P\x85\x82\x10\x15b\0\x03\xA9W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15b\0\x03\xE8W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01b\0\x03\xCAV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15b\0\x02\x94W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa+\x06b\0\x04\x89`\09`\0a\t\x9A\x01R`\0a\to\x01R`\0a\x10\x88\x01R`\0a\x10`\x01R`\0a\x0F\xBB\x01R`\0a\x0F\xE5\x01R`\0a\x10\x0F\x01Ra+\x06`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80cx[\xB0>\x11a\0\xF9W\x80c\x9A\xB2N\xB0\x11a\0\x97W\x80c\xC3\xCD\xA5 \x11a\0qW\x80c\xC3\xCD\xA5 \x14a\x03\xCFW\x80c\xC8{V\xDD\x14a\x03\xE2W\x80c\xE9\x85\xE9\xC5\x14a\x03\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\x041W`\0\x80\xFD[\x80c\x9A\xB2N\xB0\x14a\x03\x96W\x80c\xA2,\xB4e\x14a\x03\xA9W\x80c\xB8\x8DO\xDE\x14a\x03\xBCW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xD3W\x80c\x8D\xA5\xCB[\x14a\x03KW\x80c\x8ES\x9E\x8C\x14a\x03\\W\x80c\x91\xDD\xAD\xF4\x14a\x03oW\x80c\x95\xD8\x9BA\x14a\x03\x8EW`\0\x80\xFD[\x80cx[\xB0>\x14a\x03\nW\x80c~\xCE\xBE\0\x14a\x03\x1DW\x80c\x84\xB0\x19n\x14a\x030W`\0\x80\xFD[\x80cB\x84.\x0E\x11a\x01fW\x80c\\\x19\xA9\\\x11a\x01@W\x80c\\\x19\xA9\\\x14a\x02\xC9W\x80ccR!\x1E\x14a\x02\xDCW\x80cp\xA0\x821\x14a\x02\xEFW\x80cqP\x18\xA6\x14a\x03\x02W`\0\x80\xFD[\x80cB\x84.\x0E\x14a\x02\x82W\x80cK\xF5\xD7\xE9\x14a\x02\x95W\x80cX|\xDE\x1E\x14a\x02\x9DW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x01\xA2W\x80c\t^\xA7\xB3\x14a\x021W\x80c#\xB8r\xDD\x14a\x02FW\x80c6D\xE5\x15\x14a\x02YW\x80c:F\xB1\xA8\x14a\x02oW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xC9W\x80c\x06\xFD\xDE\x03\x14a\x01\xF1W\x80c\x08\x18\x12\xFC\x14a\x02\x06W[`\0\x80\xFD[a\x01\xDCa\x01\xD76`\x04a#?V[a\x04DV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9a\x04\x96V[`@Qa\x01\xE8\x91\x90a#\xACV[a\x02\x19a\x02\x146`\x04a#\xBFV[a\x05(V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE8V[a\x02Da\x02?6`\x04a#\xF4V[a\x05OV[\0[a\x02Da\x02T6`\x04a$\x1EV[a\x06iV[a\x02aa\x06\x9AV[`@Q\x90\x81R` \x01a\x01\xE8V[a\x02aa\x02}6`\x04a#\xF4V[a\x06\xA9V[a\x02Da\x02\x906`\x04a$\x1EV[a\x07:V[a\x01\xF9a\x07UV[a\x02\x19a\x02\xAB6`\x04a$ZV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\t` R`@\x90 T\x16\x90V[a\x02Da\x02\xD76`\x04a$ZV[a\x07\xEDV[a\x02\x19a\x02\xEA6`\x04a#\xBFV[a\x07\xFCV[a\x02aa\x02\xFD6`\x04a$ZV[a\x08\\V[a\x02Da\x08\xE2V[a\x02Da\x03\x186`\x04a$uV[a\x08\xF6V[a\x02aa\x03+6`\x04a$ZV[a\tCV[a\x038a\taV[`@Qa\x01\xE8\x97\x96\x95\x94\x93\x92\x91\x90a$\xF8V[`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x19V[a\x02aa\x03j6`\x04a#\xBFV[a\t\xEAV[a\x03wa\ndV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE8V[a\x01\xF9a\noV[a\x02aa\x03\xA46`\x04a$ZV[a\n~V[a\x02Da\x03\xB76`\x04a%\x8EV[a\n\x9FV[a\x02Da\x03\xCA6`\x04a%\xE0V[a\n\xAAV[a\x02Da\x03\xDD6`\x04a&\xBCV[a\n\xDCV[a\x01\xF9a\x03\xF06`\x04a#\xBFV[a\x0C\tV[a\x01\xDCa\x04\x036`\x04a'\x1CV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x02Da\x04?6`\x04a$ZV[a\x0C}V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x04uWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x04\x90WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\0\x80Ta\x04\xA5\x90a'OV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xD1\x90a'OV[\x80\x15a\x05\x1EW\x80`\x1F\x10a\x04\xF3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x1EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x01W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x053\x82a\x0C\xF6V[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x05Z\x82a\x07\xFCV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x05\xE8WPa\x05\xE8\x813a\x04\x03V[a\x06ZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\x05\xC3V[a\x06d\x83\x83a\rUV[PPPV[a\x06s3\x82a\r\xC3V[a\x06\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a'\x83V[a\x06d\x83\x83\x83a\x0EBV[`\0a\x06\xA4a\x0F\xAEV[\x90P\x90V[`\0a\x06\xB3a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x07\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`d\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\x07*a\x07\x0C\x83a\x10\xD9V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\n` R`@\x90 \x90a\x11BV[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x06d\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\n\xAAV[``Ca\x07`a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FVotes: broken clock mode\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[P`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R\x90V[3a\x07\xF8\x81\x83a\x11\xF7V[PPV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x04\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x05\xC3V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01Rh64\xB2\x107\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x08\xEAa\x12iV[a\x08\xF4`\0a\x12\xC3V[V[a\x08\xFEa\x12iV[`\0a\t\t`\rT\x90V[\x90Pa\t\x19`\r\x80T`\x01\x01\x90UV[`\0\x81\x81R`\x0E` R`@\x90 a\t2\x83\x85\x83a(\x1EV[Pa\t=\x84\x82a\x13\x15V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x81 Ta\x04\x90V[`\0``\x80\x82\x80\x80\x83a\t\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x07a\x13/V[a\t\xC0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08a\x13/V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0a\t\xF4a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`d\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\nUa\nM\x83a\x10\xD9V[`\x0B\x90a\x11BV[`\x01`\x01`\xE0\x1B\x03\x16\x92\x91PPV[`\0a\x06\xA4Ca\x13\xDAV[```\x01\x80Ta\x04\xA5\x90a'OV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x81 a\nU\x90a\x14AV[a\x07\xF83\x83\x83a\x14{V[a\n\xB43\x83a\r\xC3V[a\n\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a'\x83V[a\t=\x84\x84\x84\x84a\x15IV[\x83B\x11\x15a\x0B,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FVotes: signature expired\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x0B\xA6\x90a\x0B\x9E\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x15|V[\x85\x85\x85a\x15\xA9V[\x90Pa\x0B\xB1\x81a\x15\xD1V[\x86\x14a\x0B\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsVotes: invalid nonce``\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\x0C\0\x81\x88a\x11\xF7V[PPPPPPPV[``a\x0C\x14\x82a\x0C\xF6V[`\0a\x0C+`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\x0CKW`@Q\x80` \x01`@R\x80`\0\x81RPa\x0CvV[\x80a\x0CU\x84a\x15\xF9V[`@Q` \x01a\x0Cf\x92\x91\x90a(\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[a\x0C\x85a\x12iV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[a\x0C\xF3\x81a\x12\xC3V[PV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\r\x8A\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0\x80a\r\xCF\x83a\x07\xFCV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0E\x16WP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x0E:WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x0E/\x84a\x05(V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0EU\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x0EV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xF0\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x0EV[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4a\x06d\x83\x83\x83`\x01a\x16\x8CV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x10\x07WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x101WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x06\xA4`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x11\x9FW`\0a\x11]\x84a\x16\x98V[a\x11g\x90\x85a)iV[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x11\x8FW\x80\x91Pa\x11\x9DV[a\x11\x9A\x81`\x01a)|V[\x92P[P[`\0a\x11\xAD\x87\x87\x85\x85a\x17\x80V[\x90P\x80\x15a\x11\xE9Wa\x11\xD2\x87a\x11\xC4`\x01\x84a)iV[`\0\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x11\xECV[`\0[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\t` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x06d\x81\x83a\x12d\x86a\x17\xDEV[a\x17\xE9V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xC3V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[a\x07\xF8\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x19UV[```\xFF\x83\x14a\x13IWa\x13B\x83a\x19\x88V[\x90Pa\x04\x90V[\x81\x80Ta\x13U\x90a'OV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x81\x90a'OV[\x80\x15a\x13\xCEW\x80`\x1F\x10a\x13\xA3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xCEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x04\x90V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[\x80T`\0\x90\x80\x15a\x14rWa\x14[\x83a\x11\xC4`\x01\x84a)iV[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0CvV[`\0\x93\x92PPPV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x14\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x15T\x84\x84\x84a\x0EBV[a\x15`\x84\x84\x84\x84a\x19\xC7V[a\t=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[`\0a\x04\x90a\x15\x89a\x0F\xAEV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x15\xBA\x87\x87\x87\x87a\x1A\xC5V[\x91P\x91Pa\x15\xC7\x81a\x1B\x89V[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[```\0a\x16\x06\x83a\x1C\xD3V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16&Wa\x16&a%\xCAV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x16PW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x16ZWP\x93\x92PPPV[a\t=\x84\x84\x84\x84a\x1D\xABV[`\0\x81`\0\x03a\x16\xAAWP`\0\x91\x90PV[`\0`\x01a\x16\xB7\x84a\x1D\xBBV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x16\xD0Wa\x16\xD0a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x16\xE8Wa\x16\xE8a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17\0Wa\x17\0a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17\x18Wa\x17\x18a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x170Wa\x170a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17HWa\x17Ha)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17`Wa\x17`a)\xE1V[\x04\x82\x01\x90\x1C\x90Pa\x0Cv\x81\x82\x85\x81a\x17zWa\x17za)\xE1V[\x04a\x1EOV[`\0[\x81\x83\x10\x15a\x17\xD6W`\0a\x17\x97\x84\x84a\x1EeV[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xC2W\x80\x92Pa\x17\xD0V[a\x17\xCD\x81`\x01a)|V[\x93P[Pa\x17\x83V[P\x93\x92PPPV[`\0a\x04\x90\x82a\x08\\V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x18\x0BWP`\0\x81\x11[\x15a\x06dW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x18\xB3W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\n` R`@\x81 \x81\x90a\x18N\x90a\x1E\x80a\x18I\x86a\x1E\x8CV[a\x1E\xF5V[`\x01`\x01`\xE0\x1B\x03\x16\x91P`\x01`\x01`\xE0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x18\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x06dW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\n` R`@\x81 \x81\x90a\x18\xEC\x90a\x1F>a\x18I\x86a\x1E\x8CV[`\x01`\x01`\xE0\x1B\x03\x16\x91P`\x01`\x01`\xE0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x19F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x19_\x83\x83a\x1FJV[a\x19l`\0\x84\x84\x84a\x19\xC7V[a\x06dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[```\0a\x19\x95\x83a \xDFV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x1A\xBDW`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x1A\x0B\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a)\xF7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x1AFWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1AC\x91\x81\x01\x90a*4V[`\x01[a\x1A\xA3W=\x80\x80\x15a\x1AtW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1AyV[``\x91P[P\x80Q`\0\x03a\x1A\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x0E:V[P`\x01a\x0E:V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1A\xFCWP`\0\x90P`\x03a\x1B\x80V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1BPW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1ByW`\0`\x01\x92P\x92PPa\x1B\x80V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a\x1B\x9DWa\x1B\x9Da*QV[\x03a\x1B\xA5WPV[`\x01\x81`\x04\x81\x11\x15a\x1B\xB9Wa\x1B\xB9a*QV[\x03a\x1C\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x02\x81`\x04\x81\x11\x15a\x1C\x1AWa\x1C\x1Aa*QV[\x03a\x1CgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\xC3V[`\x03\x81`\x04\x81\x11\x15a\x1C{Wa\x1C{a*QV[\x03a\x0C\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x1D\x12Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x1D>Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x1D\\Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x1DtWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x1D\x88Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x1D\x9AW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x04\x90W`\x01\x01\x92\x91PPV[a\x1D\xB6\x84\x84\x83a!\x07V[a\t=V[`\0\x80`\x80\x83\x90\x1C\x15a\x1D\xD0W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a\x1D\xE2W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a\x1D\xF4W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a\x1E\x06W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a\x1E\x18W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a\x1E*W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a\x1E<W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x04\x90W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a\x1E^W\x81a\x0CvV[P\x90\x91\x90PV[`\0a\x1Et`\x02\x84\x84\x18a*gV[a\x0Cv\x90\x84\x84\x16a)|V[`\0a\x0Cv\x82\x84a*\x89V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[`\0\x80a\x1F1a\x1F\x13a\x1F\x06a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x10\xD9V[a\x1F)a\x1F\x1F\x88a\x14AV[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a!}V[\x91P\x91P[\x93P\x93\x91PPV[`\0a\x0Cv\x82\x84a*\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1F\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a \x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4a\x07\xF8`\0\x83\x83`\x01a\x16\x8CV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x04\x90W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a!)Wa!&`\x0Ba\x1F>a\x18I\x84a\x1E\x8CV[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a!KWa!H`\x0Ba\x1E\x80a\x18I\x84a\x1E\x8CV[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\t` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x06d\x92\x91\x82\x16\x91\x16\x83a\x17\xE9V[`\0\x80a\x1F1\x85\x85\x85\x82T`\0\x90\x81\x90\x80\x15a\"\xCFW`\0a!\xA4\x87a\x11\xC4`\x01\x85a)iV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a\"&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a\"oW\x84a\"G\x88a\x11\xC4`\x01\x86a)iV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\"\xBFV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16d\x01\0\0\0\0\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa\x1F6\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16d\x01\0\0\0\0\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a\x1F6V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C\xF3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#QW`\0\x80\xFD[\x815a\x0Cv\x81a#)V[`\0[\x83\x81\x10\x15a#wW\x81\x81\x01Q\x83\x82\x01R` \x01a#_V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra#\x98\x81` \x86\x01` \x86\x01a#\\V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0Cv` \x83\x01\x84a#\x80V[`\0` \x82\x84\x03\x12\x15a#\xD1W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a#\xEFW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\x07W`\0\x80\xFD[a$\x10\x83a#\xD8V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a$3W`\0\x80\xFD[a$<\x84a#\xD8V[\x92Pa$J` \x85\x01a#\xD8V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a$lW`\0\x80\xFD[a\x0Cv\x82a#\xD8V[`\0\x80`\0`@\x84\x86\x03\x12\x15a$\x8AW`\0\x80\xFD[a$\x93\x84a#\xD8V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\xB0W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a$\xC4W`\0\x80\xFD[\x815\x81\x81\x11\x15a$\xD3W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a$\xE5W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\xFF`\xF8\x1B\x88\x16\x81R`\0` `\xE0\x81\x84\x01Ra%\x18`\xE0\x84\x01\x8Aa#\x80V[\x83\x81\x03`@\x85\x01Ra%*\x81\x8Aa#\x80V[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R\x83\x87\x01\x92P\x90\x83\x01\x90`\0[\x81\x81\x10\x15a%|W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a%`V[P\x90\x9C\x9BPPPPPPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a%\xA1W`\0\x80\xFD[a%\xAA\x83a#\xD8V[\x91P` \x83\x015\x80\x15\x15\x81\x14a%\xBFW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a%\xF6W`\0\x80\xFD[a%\xFF\x85a#\xD8V[\x93Pa&\r` \x86\x01a#\xD8V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&1W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a&EW`\0\x80\xFD[\x815\x81\x81\x11\x15a&WWa&Wa%\xCAV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a&\x7FWa&\x7Fa%\xCAV[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a&\x98W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a&\xD5W`\0\x80\xFD[a&\xDE\x87a#\xD8V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a'\x02W`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a'/W`\0\x80\xFD[a'8\x83a#\xD8V[\x91Pa'F` \x84\x01a#\xD8V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a'cW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\xF3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`-\x90\x82\x01R\x7FERC721: caller is not token owne`@\x82\x01Rl\x1C\x88\x1B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\x9A\x1B``\x82\x01R`\x80\x01\x90V[`\x1F\x82\x11\x15a\x06dW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'\xF7WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a(\x16W\x82\x81U`\x01\x01a(\x03V[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a(6Wa(6a%\xCAV[a(J\x83a(D\x83Ta'OV[\x83a'\xD0V[`\0`\x1F\x84\x11`\x01\x81\x14a(~W`\0\x85\x15a(fWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua(\xD8V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a(\xAFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a(\x8FV[P\x86\x82\x10\x15a(\xCCW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x83Qa(\xF1\x81\x84` \x88\x01a#\\V[\x83Q\x90\x83\x01\x90a)\x05\x81\x83` \x88\x01a#\\V[\x01\x94\x93PPPPV[` \x80\x82R`%\x90\x82\x01R\x7FERC721: transfer from incorrect `@\x82\x01Rd7\xBB\xB72\xB9`\xD9\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\x90Wa\x04\x90a)SV[\x80\x82\x01\x80\x82\x11\x15a\x04\x90Wa\x04\x90a)SV[` \x80\x82R`2\x90\x82\x01R\x7FERC721: transfer to non ERC721Re`@\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a**\x90\x83\x01\x84a#\x80V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a*FW`\0\x80\xFD[\x81Qa\x0Cv\x81a#)V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82a*\x84WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01`\x01`\xE0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a*\xA9Wa*\xA9a)SV[P\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a*\xA9Wa*\xA9a)SV\xFE\xA2dipfsX\"\x12 v\xB5\x89\x12\x0C\xB0(\xC78\xCA\xA4\xD5\x8B\xEE\xA20@\xD7\x1B\xEEL\x85'%#`|\xED\xF2\x1A\\VdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static PROJECT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02vW`\x005`\xE0\x1C\x80cp\xB0\xF6`\x11a\x01OW\x80c\xBC\xCFZ\xE4\x11a\0\xC1W\x80c\xEA\x02\x17\xCF\x11a\0zW\x80c\xEA\x02\x17\xCF\x14a\x08|W\x80c\xEB\x90\x19\xD4\x14a\x08\x9CW\x80c\xEC\xE4\x0C\xC1\x14a\x08\xBCW\x80c\xF2:na\x14a\x08\xDCW\x80c\xF8\xCEV\n\x14a\t\x08W\x80c\xFC\x0CTj\x14a\t(W`\0\x80\xFD[\x80c\xBC\xCFZ\xE4\x14a\x07vW\x80c\xC0\x1F\x9E7\x14a\x07\x96W\x80c\xC2\x8B\xC2\xFA\x14a\x07\xCFW\x80c\xC5\x90W\xE4\x14a\x07\xE2W\x80c\xDDN+\xA5\x14a\x08\x02W\x80c\xDE\xAA\xA7\xCC\x14a\x08HW`\0\x80\xFD[\x80c\x91\xDD\xAD\xF4\x11a\x01\x13W\x80c\x91\xDD\xAD\xF4\x14a\x06\xC0W\x80c\x97\xC3\xD34\x14a\x06\xECW\x80c\x9A\x80*m\x14a\x07\0W\x80c\xA7q:p\x14a\x07 W\x80c\xB5\x811\xB0\x14a\x075W\x80c\xBC\x19|\x81\x14a\x07JW`\0\x80\xFD[\x80cp\xB0\xF6`\x14a\x06\x13W\x80c{<q\xD3\x14a\x063W\x80c}^\x81\xE2\x14a\x06SW\x80c\x84\xB0\x19n\x14a\x06sW\x80c\x8Bd\xBB\x87\x14a\x06\x9BW`\0\x80\xFD[\x80c;\xCC\xF4\xFD\x11a\x01\xE8W\x80cK\xF5\xD7\xE9\x11a\x01\xACW\x80cK\xF5\xD7\xE9\x14a\x05\x1FW\x80cTO\xFC\x9C\x14a\x054W\x80cT\xFDMP\x14a\x05\x89W\x80cVx\x13\x88\x14a\x05\xB3W\x80c_9\x8A\x14\x14a\x05\xD3W\x80c`\xC4$\x7F\x14a\x05\xF3W`\0\x80\xFD[\x80c;\xCC\xF4\xFD\x14a\x04rW\x80c>OI\xE6\x14a\x04\x92W\x80cC\x85\x962\x14a\x04\xBFW\x80cC\x9F\xAB\x91\x14a\x04\xDFW\x80cE!\x15\xD6\x14a\x04\xFFW`\0\x80\xFD[\x80c\x144\x89\xD0\x11a\x02:W\x80c\x144\x89\xD0\x14a\x03GW\x80c\x15\x0Bz\x02\x14a\x03\x9CW\x80c&V\"}\x14a\x03\xE0W\x80c-c\xF6\x93\x14a\x03\xF3W\x80c/\xE3\xE2a\x14a\x04)W\x80c92\xAB\xB1\x14a\x04]W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02\x8DW\x80c\x02\xA2Q\xA3\x14a\x02\xC2W\x80c\x03B\x01\x81\x14a\x02\xE5W\x80c\x06\xF3\xF9\xE6\x14a\x03\x05W\x80c\x06\xFD\xDE\x03\x14a\x03%W`\0\x80\xFD[6a\x02\x88W\0[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xADa\x02\xA86`\x04a2\xC7V[a\t\\V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xCEW`\0\x80\xFD[Pa\x02\xD7a\t\xF9V[`@Q\x90\x81R` \x01a\x02\xB9V[4\x80\x15a\x02\xF1W`\0\x80\xFD[Pa\x02\xD7a\x03\x006`\x04a4\x15V[a\n\tV[4\x80\x15a\x03\x11W`\0\x80\xFD[Pa\x02\x86a\x03 6`\x04a4\xBBV[a\x0B\x01V[4\x80\x15a\x031W`\0\x80\xFD[Pa\x03:a\x0BCV[`@Qa\x02\xB9\x91\x90a5$V[4\x80\x15a\x03SW`\0\x80\xFD[Pa\x03\x84a\x03b6`\x04a4\xBBV[`\0\x90\x81R`\x03` R`@\x90 T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xB9V[4\x80\x15a\x03\xA8W`\0\x80\xFD[Pa\x03\xC7a\x03\xB76`\x04a5NV[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x02\xB9V[a\x02\xD7a\x03\xEE6`\x04a7\x1FV[a\x0B\xD5V[4\x80\x15a\x03\xFFW`\0\x80\xFD[Pa\x02\xD7a\x04\x0E6`\x04a4\xBBV[`\0\x90\x81R`\x03` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[4\x80\x15a\x045W`\0\x80\xFD[Pa\x02\xD7\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x81V[4\x80\x15a\x04iW`\0\x80\xFD[Pa\x02\xD7a\r\x01V[4\x80\x15a\x04~W`\0\x80\xFD[Pa\x02\xD7a\x04\x8D6`\x04a7\xAEV[a\r\x0CV[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x04\xB2a\x04\xAD6`\x04a4\xBBV[a\r\x82V[`@Qa\x02\xB9\x91\x90a8\x12V[4\x80\x15a\x04\xCBW`\0\x80\xFD[Pa\x02\xADa\x04\xDA6`\x04a8:V[a\x0E\xC2V[4\x80\x15a\x04\xEBW`\0\x80\xFD[Pa\x02\x86a\x04\xFA6`\x04a8fV[a\x0E\xF2V[4\x80\x15a\x05\x0BW`\0\x80\xFD[Pa\x02\xD7a\x05\x1A6`\x04a7\x1FV[a\x0F\xC1V[4\x80\x15a\x05+W`\0\x80\xFD[Pa\x03:a\x10\xC8V[4\x80\x15a\x05@W`\0\x80\xFD[Pa\x05na\x05O6`\x04a4\xBBV[`\0\x90\x81R`\t` R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x92V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\xB9V[4\x80\x15a\x05\x95W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`1`\xF8\x1B` \x82\x01Ra\x03:V[4\x80\x15a\x05\xBFW`\0\x80\xFD[Pa\x02\xD7a\x05\xCE6`\x04a8\xA7V[a\x11\x8AV[4\x80\x15a\x05\xDFW`\0\x80\xFD[Pa\x02\xD7a\x05\xEE6`\x04a8\xCAV[a\x11\xB3V[4\x80\x15a\x05\xFFW`\0\x80\xFD[Pa\x02\xD7a\x06\x0E6`\x04a4\xBBV[a\x11\xFDV[4\x80\x15a\x06\x1FW`\0\x80\xFD[Pa\x02\x86a\x06.6`\x04a4\xBBV[a\x12\xAEV[4\x80\x15a\x06?W`\0\x80\xFD[Pa\x02\xD7a\x06N6`\x04a9MV[a\x12\xEDV[4\x80\x15a\x06_W`\0\x80\xFD[Pa\x02\xD7a\x06n6`\x04a9\xA6V[a\x135V[4\x80\x15a\x06\x7FW`\0\x80\xFD[Pa\x06\x88a\x17\x0EV[`@Qa\x02\xB9\x97\x96\x95\x94\x93\x92\x91\x90a:\x95V[4\x80\x15a\x06\xA7W`\0\x80\xFD[P`\x0CTa\x03\x84\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xCCW`\0\x80\xFD[Pa\x06\xD5a\x17\x96V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xB9V[4\x80\x15a\x06\xF8W`\0\x80\xFD[P`da\x02\xD7V[4\x80\x15a\x07\x0CW`\0\x80\xFD[Pa\x02\xD7a\x07\x1B6`\x04a:\xF7V[a\x18\x1FV[4\x80\x15a\x07,W`\0\x80\xFD[Pa\x02\xD7a\x186V[4\x80\x15a\x07AW`\0\x80\xFD[Pa\x02\xD7a\x18`V[4\x80\x15a\x07VW`\0\x80\xFD[Pa\x03\xC7a\x07e6`\x04a;MV[c\xBC\x19|\x81`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\x07\x82W`\0\x80\xFD[Pa\x02\x86a\x07\x916`\x04a;\xDCV[a\x18kV[4\x80\x15a\x07\xA2W`\0\x80\xFD[Pa\x02\xD7a\x07\xB16`\x04a4\xBBV[`\0\x90\x81R`\x03` R`@\x90 `\x01\x01T`\x01`\x01`@\x1B\x03\x16\x90V[a\x02\x86a\x07\xDD6`\x04a<;V[a\x19\xFCV[4\x80\x15a\x07\xEEW`\0\x80\xFD[Pa\x02\xD7a\x07\xFD6`\x04a7\x1FV[a\x1A\xC3V[4\x80\x15a\x08\x0EW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R` \x80\x82R\x7Fsupport=bravo&quorum=for,abstain\x90\x82\x01Ra\x03:V[4\x80\x15a\x08TW`\0\x80\xFD[Pa\x02\xD7\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F\x81V[4\x80\x15a\x08\x88W`\0\x80\xFD[Pa\x02\x86a\x08\x976`\x04a4\xBBV[a\x1A\xFDV[4\x80\x15a\x08\xA8W`\0\x80\xFD[Pa\x02\xD7a\x08\xB76`\x04a<|V[a\x1B<V[4\x80\x15a\x08\xC8W`\0\x80\xFD[Pa\x02\x86a\x08\xD76`\x04a4\xBBV[a\x1B]V[4\x80\x15a\x08\xE8W`\0\x80\xFD[Pa\x03\xC7a\x08\xF76`\x04a<\xA6V[c\xF2:na`\xE0\x1B\x95\x94PPPPPV[4\x80\x15a\t\x14W`\0\x80\xFD[Pa\x02\xD7a\t#6`\x04a4\xBBV[a\x1B\x9CV[4\x80\x15a\t4W`\0\x80\xFD[Pa\x03\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0c(\x8A\xCE\x03`\xE1\x1Bc\x18\xDFt?`\xE3\x1Bc\xBF&\xD8\x97`\xE0\x1Bcy\xDDyo`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x82\x14\x80a\t\xA2WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x81\x16\x90\x82\x16\x14[\x80a\t\xB9WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x81\x16\x90\x85\x16\x14[\x80a\t\xD4WP`\x01`\x01`\xE0\x1B\x03\x19\x86\x16c\x02q\x18\x97`\xE5\x1B\x14[\x80a\t\xEFWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x87\x16\x14[\x96\x95PPPPPPV[`\0a\n\x04`\x07T\x90V[\x90P\x90V[`\0\x80a\n\xADa\n\xA5\x7F\xB3\xB3\xF3\xB7\x03\xCD\x84\xCE5!\x97\xDC\xFF#+\x1B]<\xFB %\xCEG\xCF\x04t-\x06Q\xF1\xAF\x88\x8C\x8C\x8C\x8C`@Qa\nE\x92\x91\x90a=\nV[`@Q\x80\x91\x03\x90 \x8B\x80Q\x90` \x01 `@Q` \x01a\n\x8A\x95\x94\x93\x92\x91\x90\x94\x85R` \x85\x01\x93\x90\x93R`\xFF\x91\x90\x91\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1B\xA7V[\x86\x86\x86a\x1B\xD4V[\x90Pa\n\xF3\x8A\x82\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8D\x92Pa\x1B\xF2\x91PPV[\x9A\x99PPPPPPPPPPV[30\x14a\x0B W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x0B7V[\x80a\x0B0`\x04a\x1DGV[\x03a\x0B%WP[a\x0B@\x81a\x1D\xC6V[PV[```\x02\x80Ta\x0BR\x90a=QV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B~\x90a=QV[\x80\x15a\x0B\xCBW\x80`\x1F\x10a\x0B\xA0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xCBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xAEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0\x80a\x0B\xE4\x86\x86\x86\x86a\x1A\xC3V[\x90P`\0a\x0B\xF1\x82a\r\x82V[\x90P`\x04\x81`\x07\x81\x11\x15a\x0C\x07Wa\x0C\x07a7\xFCV[\x14\x80a\x0C$WP`\x05\x81`\x07\x81\x11\x15a\x0C\"Wa\x0C\"a7\xFCV[\x14[a\x0CzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal not successfu`D\x82\x01R`\x1B`\xFA\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0\x82\x81R`\x03` R`@\x90\x81\x90 `\x02\x01\x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fq*\xE18?y\xAC\x85?\x8D\x88!Sw\x8E\x02`\xEF\x8F\x03\xB5\x04\xE2\x86n\x05\x93\xE0M+)\x1F\x90a\x0C\xC8\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x0C\xDD\x82\x88\x88\x88\x88a\x1F4V[a\x0C\xEA\x82\x88\x88\x88\x88a\x1F\xBFV[a\x0C\xF7\x82\x88\x88\x88\x88a\x1F4V[P\x95\x94PPPPPV[`\0a\n\x04`\x06T\x90V[`@\x80Q\x7F\x15\x02\x14\xD7MY\xB7\xD1\xE9\x0Cs\xFC\"\xEF=\x99\x1D\xD0\xA7k\x04eC\xD4\xD8\n\xB9-*P2\x8F` \x82\x01R\x90\x81\x01\x86\x90R`\xFF\x85\x16``\x82\x01R`\0\x90\x81\x90a\rZ\x90a\n\xA5\x90`\x80\x01a\n\x8AV[\x90Pa\rw\x87\x82\x88`@Q\x80` \x01`@R\x80`\0\x81RPa \xB5V[\x97\x96PPPPPPPV[`\0\x81\x81R`\x03` R`@\x81 `\x02\x81\x01T`\xFF\x16\x15a\r\xA6WP`\x07\x92\x91PPV[`\x02\x81\x01Ta\x01\0\x90\x04`\xFF\x16\x15a\r\xC1WP`\x02\x92\x91PPV[`\0\x83\x81R`\x03` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x81\x90\x03a\x0E)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: unknown proposal id\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0a\x0E3a\x17\x96V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x80\x82\x10a\x0EOWP`\0\x94\x93PPPPV[`\0\x85\x81R`\x03` R`@\x90 `\x01\x01T`\x01`\x01`@\x1B\x03\x16\x81\x81\x10a\x0E}WP`\x01\x95\x94PPPPPV[a\x0E\x86\x86a \xE1V[\x80\x15a\x0E\xA5WP`\0\x86\x81R`\t` R`@\x90 \x80T`\x01\x90\x91\x01T\x11[\x15a\x0E\xB6WP`\x04\x95\x94PPPPPV[P`\x03\x95\x94PPPPPV[`\0\x82\x81R`\t` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R`\x03\x01\x90\x91R\x90 T`\xFF\x16[\x92\x91PPV[`\x0CT`\xFF\x16\x15a\x0FEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FProject: already initialized\0\0\0\0`D\x82\x01R`d\x01a\x02}V[`\x0C\x80T`\x01`\xFF\x19\x90\x91\x16\x17\x90\x81\x90U`@Qc<-\xD8\x1F`\xE1\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cx[\xB0>\x90a\x0F\x8B\x903\x90\x86\x90\x86\x90`\x04\x01a=\x8BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xB9W=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x80a\x0F\xD0\x86\x86\x86\x86a\x1A\xC3V[\x90P`\0a\x0F\xDD\x82a\r\x82V[`\x07\x81\x11\x15a\x0F\xEEWa\x0F\xEEa7\xFCV[\x14a\x10;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FGovernor: too late to cancel\0\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FGovernor: only proposer can canc`D\x82\x01Ra\x19[`\xF2\x1B`d\x82\x01R`\x84\x01a\x02}V[a\t\xEF\x86\x86\x86\x86a!-V[``\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cK\xF5\xD7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x11IWP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11F\x91\x90\x81\x01\x90a=\xCBV[`\x01[a\x11\x85WP`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R\x90V[\x91\x90PV[`\0\x803\x90Pa\x11\xAB\x84\x82\x85`@Q\x80` \x01`@R\x80`\0\x81RPa \xB5V[\x94\x93PPPPV[`\0\x803\x90Pa\rw\x87\x82\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92Pa\x1B\xF2\x91PPV[`\x0BT`\0\x90\x80\x82\x03a\x12\x14WPP`\nT\x91\x90PV[`\0`\x0Ba\x12#`\x01\x84a>NV[\x81T\x81\x10a\x123Wa\x123a>aV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`\x01` \x1B\x90\x91\x04`\x01`\x01`\xE0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R\x91P\x84\x10a\x12\x89W` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x12\x9Da\x12\x95\x85a\"KV[`\x0B\x90a\"\xB4V[`\x01`\x01`\xE0\x1B\x03\x16\x94\x93PPPPV[30\x14a\x12\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x12\xE4V[\x80a\x12\xDD`\x04a\x1DGV[\x03a\x12\xD2WP[a\x0B@\x81a#gV[`\0\x803\x90Pa\t\xEF\x86\x82\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa \xB5\x92PPPV[`\x003a\x13B\x81\x84a#\xA8V[a\x13\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: proposer restricted\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0a\x13\x98a\x17\x96V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x13\xAAa\x18`V[a\x13\xB9\x83a\x08\xB7`\x01\x85a>NV[\x10\x15a\x14!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FGovernor: proposer votes below p`D\x82\x01Rp\x1C\x9B\xDC\x1B\xDC\xD8[\x08\x1D\x1A\x1C\x99\\\xDA\x1B\xDB\x19`z\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0a\x146\x88\x88\x88\x88\x80Q\x90` \x01 a\x1A\xC3V[\x90P\x86Q\x88Q\x14a\x14YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a>wV[\x85Q\x88Q\x14a\x14zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a>wV[`\0\x88Q\x11a\x14\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FGovernor: empty proposal\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0\x81\x81R`\x03` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15a\x15:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FGovernor: proposal already exist`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0a\x15Da\r\x01V[a\x15N\x90\x84a>\xB8V[\x90P`\0a\x15Za\t\xF9V[a\x15d\x90\x83a>\xB8V[\x90P`@Q\x80`\xE0\x01`@R\x80a\x15z\x84a$\x99V[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R`\0`@\x82\x01R``\x01a\x15\xA7\x83a$\x99V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\0` \x80\x84\x01\x82\x90R`@\x80\x85\x01\x83\x90R``\x94\x85\x01\x83\x90R\x88\x83R`\x03\x82R\x91\x82\x90 \x85Q\x81T\x92\x87\x01Q\x87\x85\x01Q\x91\x86\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84\x02\x17`\x01`\x01`\xE0\x1B\x03\x16`\x01`\xE0\x1B`\xE0\x92\x90\x92\x1C\x91\x90\x91\x02\x17\x81U\x93\x85\x01Q`\x80\x86\x01Q\x90\x84\x16\x92\x1C\x02\x17`\x01\x83\x01U`\xA0\x83\x01Q`\x02\x90\x92\x01\x80T`\xC0\x90\x94\x01Qa\xFF\xFF\x19\x90\x94\x16\x92\x15\x15a\xFF\0\x19\x16\x92\x90\x92\x17a\x01\0\x93\x15\x15\x93\x90\x93\x02\x92\x90\x92\x17\x90U\x8AQ\x7F}\x84\xA6&:\xE0\xD9\x8D3)\xBD{F\xBBN\x8Do\x98\xCD5\xA7\xAD\xB4\\'L\x8B\x7F\xD5\xEB\xD5\xE0\x91\x85\x91\x88\x91\x8E\x91\x8E\x91\x81\x11\x15a\x16\xABWa\x16\xABa3JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xDEW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\xC9W\x90P[P\x8D\x88\x88\x8F`@Qa\x16\xF8\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a?YV[`@Q\x80\x91\x03\x90\xA1P\x90\x98\x97PPPPPPPPV[`\0``\x80\x82\x80\x80\x83a\x17A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a%\x01V[a\x17l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a%\x01V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x91\xDD\xAD\xF4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x18\x12WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x18\x0F\x91\x81\x01\x90a@0V[`\x01[a\x11\x85Wa\n\x04Ca%\xACV[`\0a\x18,\x84\x84\x84a&\x13V[\x90P[\x93\x92PPPV[`\x0BT`\0\x90\x15a\x18YWa\x18K`\x0Ba&\xA9V[`\x01`\x01`\xE0\x1B\x03\x16\x90P\x90V[P`\nT\x90V[`\0a\n\x04`\x08T\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x18\xC1Wa\x18\xC1a>aV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x19\x13Wa\x19\x13a>aV[` \x02` \x01\x01\x81\x81RPP`\x003\x87\x87`@Q`$\x01a\x196\x93\x92\x91\x90a=\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c<-\xD8\x1F`\xE1\x1B\x17\x90R\x81Q`\x01\x80\x82R\x81\x84\x01\x90\x93R\x92\x93P`\0\x92\x91\x90\x82\x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19yW\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a\x19\xA4Wa\x19\xA4a>aV[` \x02` \x01\x01\x81\x90RPa\x19\xF1\x84\x84\x83\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x135\x92PPPV[PPPPPPPPPV[30\x14a\x1A\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x1A2V[\x80a\x1A+`\x04a\x1DGV[\x03a\x1A WP[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85\x85`@Qa\x1AP\x92\x91\x90a=\nV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1A\x8DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A\x92V[``\x91P[P\x91P\x91Pa\x1A\xBA\x82\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01aA\xAF`(\x919a&\xE2V[PPPPPPPV[`\0\x84\x84\x84\x84`@Q` \x01a\x1A\xDC\x94\x93\x92\x91\x90a@XV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[30\x14a\x1B\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x1B3V[\x80a\x1B,`\x04a\x1DGV[\x03a\x1B!WP[a\x0B@\x81a&\xFBV[`\0a\x18/\x83\x83a\x1BX`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a&\x13V[30\x14a\x1B|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x90a=\x1AV[a\x1B\x93V[\x80a\x1B\x8C`\x04a\x1DGV[\x03a\x1B\x81WP[a\x0B@\x81a'\x9CV[`\0a\x0E\xEC\x82a'\xDDV[`\0a\x0E\xECa\x1B\xB4a(\x87V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x1B\xE5\x87\x87\x87\x87a)\xB2V[\x91P\x91Pa\x0C\xF7\x81a*vV[`\0\x85\x81R`\x03` R`@\x81 `\x01a\x1C\x0B\x88a\r\x82V[`\x07\x81\x11\x15a\x1C\x1CWa\x1C\x1Ca7\xFCV[\x14a\x1CuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FGovernor: vote not currently act`D\x82\x01Rbive`\xE8\x1B`d\x82\x01R`\x84\x01a\x02}V[\x80T`\0\x90a\x1C\x8F\x90\x88\x90`\x01`\x01`@\x1B\x03\x16\x86a&\x13V[\x90Pa\x1C\x9E\x88\x88\x88\x84\x88a+\xC0V[\x83Q`\0\x03a\x1C\xF3W\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB8\xE18\x88}\n\xA1;\xABD~\x82\xDE\x9D\\\x17w\x04\x1E\xCD!\xCA6\xBA\x82O\xF1\xE6\xC0}\xDD\xA4\x89\x88\x84\x89`@Qa\x1C\xE6\x94\x93\x92\x91\x90a@\xA3V[`@Q\x80\x91\x03\x90\xA2a\rwV[\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE2\xBA\xBF\xBA\xC5\x88\x9Ap\x9Bc\xBB\x7FY\x8B2N\x08\xBCZO\xB9\xECd\x7F\xB3\xCB\xC9\xEC\x07\xEB\x87\x12\x89\x88\x84\x89\x89`@Qa\x1D4\x95\x94\x93\x92\x91\x90a@\xCBV[`@Q\x80\x91\x03\x90\xA2\x97\x96PPPPPPPV[`\0a\x1Db\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a\x1D\x80W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`d\x81\x11\x15a\x1EIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FGovernorVotesQuorumFraction: quo`D\x82\x01R\x7FrumNumerator over quorumDenomina`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x02}V[`\0a\x1ESa\x186V[\x90P\x80\x15\x80\x15\x90a\x1EdWP`\x0BT\x15[\x15a\x1E\xC8W`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R`\x0B\x90` \x81\x01a\x1E\x87\x84a-:V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[a\x1E\xF5a\x1E\xE3a\x1E\xD6a\x17\x96V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\"KV[a\x1E\xEC\x84a-:V[`\x0B\x91\x90a-\xA3V[PP`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\x05SGk\xF0.\xF2rn\x8C\xE5\xCE\xD7\x8Dc\xE2n`.J\"W\xB1\xF5YA\x8E$\xB4c9\x97\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x1F\xB8V[\x84Q\x81\x10\x15a\x0F\xB9W0`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\x1F^Wa\x1F^a>aV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1F\xA8Wa\x1F\xA8\x83\x82\x81Q\x81\x10a\x1F\x89Wa\x1F\x89a>aV[` \x02` \x01\x01Q\x80Q\x90` \x01 `\x04a-\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1F\xB1\x81aA\x11V[\x90Pa\x1F9V[PPPPPV[`\0`@Q\x80``\x01`@R\x80`'\x81R` \x01aA\xD7`'\x919\x90P`\0[\x85Q\x81\x10\x15a\x1A\xBAW`\0\x80\x87\x83\x81Q\x81\x10a\x1F\xFDWa\x1F\xFDa>aV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x87\x84\x81Q\x81\x10a  Wa  a>aV[` \x02` \x01\x01Q\x87\x85\x81Q\x81\x10a :Wa :a>aV[` \x02` \x01\x01Q`@Qa O\x91\x90aA*V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a \x8CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a \x91V[``\x91P[P\x91P\x91Pa \xA1\x82\x82\x86a&\xE2V[PPP\x80a \xAE\x90aA\x11V[\x90Pa\x1F\xDFV[`\0a \xD8\x85\x85\x85\x85a \xD3`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[a\x1B\xF2V[\x95\x94PPPPPV[`\0\x81\x81R`\t` R`@\x81 `\x02\x81\x01T`\x01\x82\x01Ta!\x03\x91\x90a>\xB8V[`\0\x84\x81R`\x03` R`@\x90 Ta!$\x90`\x01`\x01`@\x1B\x03\x16a\x1B\x9CV[\x11\x15\x93\x92PPPV[`\0\x80a!<\x86\x86\x86\x86a\x1A\xC3V[\x90P`\0a!I\x82a\r\x82V[\x90P`\x02\x81`\x07\x81\x11\x15a!_Wa!_a7\xFCV[\x14\x15\x80\x15a!\x7FWP`\x06\x81`\x07\x81\x11\x15a!|Wa!|a7\xFCV[\x14\x15[\x80\x15a!\x9DWP`\x07\x81`\x07\x81\x11\x15a!\x9AWa!\x9Aa7\xFCV[\x14\x15[a!\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FGovernor: proposal not active\0\0\0`D\x82\x01R`d\x01a\x02}V[`\0\x82\x81R`\x03` R`@\x90\x81\x90 `\x02\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UQ\x7Fx\x9C\xF5[\xE9\x80s\x9D\xAD\x1D\x06\x99\xB9;X\xE8\x06\xB5\x1C\x9D\x96a\x9B\xFA\x8F\xE0\xA2\x8A\xBA\xA7\xB3\x0C\x90a\"9\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1P\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\"\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02}V[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a#\x11W`\0a\"\xCF\x84a-\xFAV[a\"\xD9\x90\x85a>NV[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a#\x01W\x80\x91Pa#\x0FV[a#\x0C\x81`\x01a>\xB8V[\x92P[P[`\0a#\x1F\x87\x87\x85\x85a.\xE2V[\x90P\x80\x15a#ZWa#D\x87a#6`\x01\x84a>NV[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\rwV[`\0\x97\x96PPPPPPPV[`\x06T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xC5e\xB0E@=\xC0<.\xEA\x82\xB8\x1A\x04e\xED\xAD\x9E.\x7F\xC4\xD9~\x11B\x1C \x9D\xA9=z\x93\x91\x01`@Q\x80\x91\x03\x90\xA1`\x06UV[\x80Q`\0\x90`4\x81\x10\x15a#\xC0W`\x01\x91PPa\x0E\xECV[\x82\x81\x01`\x13\x19\x01Q`\x01`\x01`\xA0\x1B\x03\x19\x81\x16k\x04n\x0EM\xEE\r\xEEl\xAEG\xA6\x0F`\xA3\x1B\x14a#\xF3W`\x01\x92PPPa\x0E\xECV[`\0\x80a$\x01`(\x85a>NV[\x90P[\x83\x81\x10\x15a$xW`\0\x80a$8\x88\x84\x81Q\x81\x10a$$Wa$$a>aV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16a/@V[\x91P\x91P\x81a$PW`\x01\x96PPPPPPPa\x0E\xECV[\x80`\xFF\x16`\x04\x85`\x01`\x01`\xA0\x1B\x03\x16\x90\x1B\x17\x93PPP\x80a$q\x90aA\x11V[\x90Pa$\x04V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x93PPPP\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\"\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 6`D\x82\x01Re4 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02}V[```\xFF\x83\x14a%\x1BWa%\x14\x83a/\xD2V[\x90Pa\x0E\xECV[\x81\x80Ta%'\x90a=QV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%S\x90a=QV[\x80\x15a%\xA0W\x80`\x1F\x10a%uWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\xA0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%\x83W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x0E\xECV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\"\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x02}V[`@Qc\x07H\xD65`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c:F\xB1\xA8\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18,\x91\x90aAFV[\x80T`\0\x90\x80\x15a&\xD9Wa&\xC3\x83a#6`\x01\x84a>NV[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x18/V[`\0\x93\x92PPPV[``\x83\x15a&\xF1WP\x81a\x18/V[a\x18/\x83\x83a0\x11V[`\0\x81\x11a'[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorSettings: voting period `D\x82\x01Rftoo low`\xC8\x1B`d\x82\x01R`\x84\x01a\x02}V[`\x07T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F~?\x7F\x07\x08\xA8M\xE9 06\xAB\xAAE\r\xCC\xC8Z\xD5\xFFR\xF7\x8C\x17\x0F>\xDBU\xCF^\x88(\x91\x01`@Q\x80\x91\x03\x90\xA1`\x07UV[`\x08T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCC\xB4]\xA8\xD5q~lEDiB\x97\xC4\xBA\\\xF1Q\xD4U\xC9\xBB\x0E\xD4\xFCz8A\x1B\xC0Ta\x91\x01`@Q\x80\x91\x03\x90\xA1`\x08UV[`\0`da'\xEA\x83a\x11\xFDV[`@Qc#\x94\xE7\xA3`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8ES\x9E\x8C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(s\x91\x90aAFV[a(}\x91\x90aA_V[a\x0E\xEC\x91\x90aA\x8CV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a(\xE0WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a)\nWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\n\x04`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a)\xE9WP`\0\x90P`\x03a*mV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a*=W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a*fW`\0`\x01\x92P\x92PPa*mV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a*\x8AWa*\x8Aa7\xFCV[\x03a*\x92WPV[`\x01\x81`\x04\x81\x11\x15a*\xA6Wa*\xA6a7\xFCV[\x03a*\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02}V[`\x02\x81`\x04\x81\x11\x15a+\x07Wa+\x07a7\xFCV[\x03a+TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02}V[`\x03\x81`\x04\x81\x11\x15a+hWa+ha7\xFCV[\x03a\x0B@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0\x85\x81R`\t` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R`\x03\x81\x01\x90\x92R\x90\x91 T`\xFF\x16\x15a,HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FGovernorVotingSimple: vote alrea`D\x82\x01Rf\x19\x1EH\x18\xD8\\\xDD`\xCA\x1B`d\x82\x01R`\x84\x01a\x02}V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03\x82\x01` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\xFF\x84\x16a,\x94W\x82\x81`\0\x01`\0\x82\x82Ta,\x89\x91\x90a>\xB8V[\x90\x91UPa\x0F\xB9\x90PV[`\0\x19`\xFF\x85\x16\x01a,\xB4W\x82\x81`\x01\x01`\0\x82\x82Ta,\x89\x91\x90a>\xB8V[`\x01\x19`\xFF\x85\x16\x01a,\xD4W\x82\x81`\x02\x01`\0\x82\x82Ta,\x89\x91\x90a>\xB8V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FGovernorVotingSimple: invalid va`D\x82\x01Rtlue for enum VoteType`X\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\"\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x02}V[`\0\x80a-\xB1\x85\x85\x85a0;V[\x91P\x91P[\x93P\x93\x91PPV[\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B`\0\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[`\0\x81`\0\x03a.\x0CWP`\0\x91\x90PV[`\0`\x01a.\x19\x84a1\xDAV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a.2Wa.2aAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.JWa.JaAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.bWa.baAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.zWa.zaAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.\x92Wa.\x92aAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.\xAAWa.\xAAaAvV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a.\xC2Wa.\xC2aAvV[\x04\x82\x01\x90\x1C\x90Pa\x18/\x81\x82\x85\x81a.\xDCWa.\xDCaAvV[\x04a2nV[`\0[\x81\x83\x10\x15a/8W`\0a.\xF9\x84\x84a2\x84V[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a/$W\x80\x92Pa/2V[a//\x81`\x01a>\xB8V[\x93P[Pa.\xE5V[P\x93\x92PPPV[`\0\x80`\xF8\x83\x90\x1C`/\x81\x11\x80\x15a/[WP`:\x81`\xFF\x16\x10[\x15a/pW`\x01\x94`/\x19\x90\x91\x01\x93P\x91PPV[\x80`\xFF\x16`@\x10\x80\x15a/\x86WP`G\x81`\xFF\x16\x10[\x15a/\x9BW`\x01\x94`6\x19\x90\x91\x01\x93P\x91PPV[\x80`\xFF\x16``\x10\x80\x15a/\xB1WP`g\x81`\xFF\x16\x10[\x15a/\xC6W`\x01\x94`V\x19\x90\x91\x01\x93P\x91PPV[P`\0\x93\x84\x93P\x91PPV[```\0a/\xDF\x83a2\x9FV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[\x81Q\x15a0!W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02}\x91\x90a5$V[\x82T`\0\x90\x81\x90\x80\x15a1\x81W`\0a0Y\x87a#6`\x01\x85a>NV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a0\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01a\x02}V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a1\"W\x84a0\xFB\x88a#6`\x01\x86a>NV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua1qV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01` \x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa-\xB6\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01` \x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a-\xB6V[`\0\x80`\x80\x83\x90\x1C\x15a1\xEFW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a2\x01W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a2\x13W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a2%W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a27W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a2IW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a2[W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x0E\xECW`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a2}W\x81a\x18/V[P\x90\x91\x90PV[`\0a2\x93`\x02\x84\x84\x18aA\x8CV[a\x18/\x90\x84\x84\x16a>\xB8V[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x0E\xECW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a2\xD9W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x18/W`\0\x80\xFD[\x805`\xFF\x81\x16\x81\x14a\x11\x85W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a3\x14W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a3+W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a3CW`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a3\x88Wa3\x88a3JV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a3\xA9Wa3\xA9a3JV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0a3\xCAa3\xC5\x84a3\x90V[a3`V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a3\xDEW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a4\x06W`\0\x80\xFD[a\x18/\x83\x835` \x85\x01a3\xB7V[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15a41W`\0\x80\xFD[\x885\x97Pa4A` \x8A\x01a2\xF1V[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a4]W`\0\x80\xFD[a4i\x8C\x83\x8D\x01a3\x02V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a4\x82W`\0\x80\xFD[Pa4\x8F\x8B\x82\x8C\x01a3\xF5V[\x94PPa4\x9E`\x80\x8A\x01a2\xF1V[\x92P`\xA0\x89\x015\x91P`\xC0\x89\x015\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a4\xCDW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a4\xEFW\x81\x81\x01Q\x83\x82\x01R` \x01a4\xD7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra5\x10\x81` \x86\x01` \x86\x01a4\xD4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x18/` \x83\x01\x84a4\xF8V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\x85W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a5dW`\0\x80\xFD[a5m\x85a57V[\x93Pa5{` \x86\x01a57V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x9DW`\0\x80\xFD[a5\xA9\x87\x82\x88\x01a3\xF5V[\x91PP\x92\x95\x91\x94P\x92PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a5\xCEWa5\xCEa3JV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a5\xE9W`\0\x80\xFD[\x815` a5\xF9a3\xC5\x83a5\xB5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a6\x18W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a6:Wa6-\x81a57V[\x83R\x91\x83\x01\x91\x83\x01a6\x1CV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a6VW`\0\x80\xFD[\x815` a6fa3\xC5\x83a5\xB5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a6\x85W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a6:W\x805\x83R\x91\x83\x01\x91\x83\x01a6\x89V[`\0\x82`\x1F\x83\x01\x12a6\xB1W`\0\x80\xFD[\x815` a6\xC1a3\xC5\x83a5\xB5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a6\xE0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a6:W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x03W`\0\x80\x81\xFD[a7\x11\x89\x86\x83\x8B\x01\x01a3\xF5V[\x84RP\x91\x83\x01\x91\x83\x01a6\xE4V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a75W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7LW`\0\x80\xFD[a7X\x88\x83\x89\x01a5\xD8V[\x95P` \x87\x015\x91P\x80\x82\x11\x15a7nW`\0\x80\xFD[a7z\x88\x83\x89\x01a6EV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a7\x90W`\0\x80\xFD[Pa7\x9D\x87\x82\x88\x01a6\xA0V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a7\xC6W`\0\x80\xFD[\x855\x94Pa7\xD6` \x87\x01a2\xF1V[\x93Pa7\xE4`@\x87\x01a2\xF1V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x08\x83\x10a84WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`@\x83\x85\x03\x12\x15a8MW`\0\x80\xFD[\x825\x91Pa8]` \x84\x01a57V[\x90P\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a8yW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a8\x8FW`\0\x80\xFD[a8\x9B\x85\x82\x86\x01a3\x02V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a8\xBAW`\0\x80\xFD[\x825\x91Pa8]` \x84\x01a2\xF1V[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a8\xE2W`\0\x80\xFD[\x855\x94Pa8\xF2` \x87\x01a2\xF1V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a9\x0EW`\0\x80\xFD[a9\x1A\x89\x83\x8A\x01a3\x02V[\x90\x95P\x93P``\x88\x015\x91P\x80\x82\x11\x15a93W`\0\x80\xFD[Pa9@\x88\x82\x89\x01a3\xF5V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a9cW`\0\x80\xFD[\x845\x93Pa9s` \x86\x01a2\xF1V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x8EW`\0\x80\xFD[a9\x9A\x87\x82\x88\x01a3\x02V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a9\xBCW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a9\xD3W`\0\x80\xFD[a9\xDF\x88\x83\x89\x01a5\xD8V[\x95P` \x87\x015\x91P\x80\x82\x11\x15a9\xF5W`\0\x80\xFD[a:\x01\x88\x83\x89\x01a6EV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a:\x17W`\0\x80\xFD[a:#\x88\x83\x89\x01a6\xA0V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a:9W`\0\x80\xFD[P\x85\x01`\x1F\x81\x01\x87\x13a:KW`\0\x80\xFD[a5\xA9\x87\x825` \x84\x01a3\xB7V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a:\x8AW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a:nV[P\x94\x95\x94PPPPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a:\xB4`\xE0\x83\x01\x89a4\xF8V[\x82\x81\x03`@\x84\x01Ra:\xC6\x81\x89a4\xF8V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x90Pa\n\xF3\x81\x85a:ZV[`\0\x80`\0``\x84\x86\x03\x12\x15a;\x0CW`\0\x80\xFD[a;\x15\x84a57V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a;7W`\0\x80\xFD[a;C\x86\x82\x87\x01a3\xF5V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a;eW`\0\x80\xFD[a;n\x86a57V[\x94Pa;|` \x87\x01a57V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\x98W`\0\x80\xFD[a;\xA4\x89\x83\x8A\x01a6EV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a;\xBAW`\0\x80\xFD[a;\xC6\x89\x83\x8A\x01a6EV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a93W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a;\xF2W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\tW`\0\x80\xFD[a<\x15\x88\x83\x89\x01a3\x02V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a<.W`\0\x80\xFD[Pa9\x9A\x87\x82\x88\x01a3\x02V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a<QW`\0\x80\xFD[a<Z\x85a57V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x8EW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a<\x8FW`\0\x80\xFD[a<\x98\x83a57V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a<\xBEW`\0\x80\xFD[a<\xC7\x86a57V[\x94Pa<\xD5` \x87\x01a57V[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xFEW`\0\x80\xFD[a9@\x88\x82\x89\x01a3\xF5V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x80\x82R`\x18\x90\x82\x01R\x7FGovernor: onlyGovernance\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a=eW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a=\x85WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a=\xDDW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a=\xF3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a>\x04W`\0\x80\xFD[\x80Qa>\x12a3\xC5\x82a3\x90V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a>'W`\0\x80\xFD[a \xD8\x82` \x83\x01` \x86\x01a4\xD4V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\xECWa\x0E\xECa>8V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`!\x90\x82\x01R\x7FGovernor: invalid proposal lengt`@\x82\x01R`\r`\xFB\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0E\xECWa\x0E\xECa>8V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a:\x8AW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a>\xDFV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15a?LW\x82\x84\x03\x89Ra?:\x84\x83Qa4\xF8V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01a?\"V[P\x91\x97\x96PPPPPPPV[`\0a\x01 \x8B\x83R` `\x01\x80`\xA0\x1B\x03\x8C\x16\x81\x85\x01R\x81`@\x85\x01Ra?\x82\x82\x85\x01\x8Ca>\xCBV[\x91P\x83\x82\x03``\x85\x01Ra?\x96\x82\x8Ba:ZV[\x91P\x83\x82\x03`\x80\x85\x01R\x81\x89Q\x80\x84R\x82\x84\x01\x91P\x82\x81`\x05\x1B\x85\x01\x01\x83\x8C\x01`\0[\x83\x81\x10\x15a?\xE7W`\x1F\x19\x87\x84\x03\x01\x85Ra?\xD5\x83\x83Qa4\xF8V[\x94\x86\x01\x94\x92P\x90\x85\x01\x90`\x01\x01a?\xB9V[PP\x86\x81\x03`\xA0\x88\x01Ra?\xFB\x81\x8Ca?\x04V[\x94PPPPP\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01Ra@ \x81\x85a4\xF8V[\x9C\x9BPPPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a@BW`\0\x80\xFD[\x81Qe\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18/W`\0\x80\xFD[`\x80\x81R`\0a@k`\x80\x83\x01\x87a>\xCBV[\x82\x81\x03` \x84\x01Ra@}\x81\x87a:ZV[\x90P\x82\x81\x03`@\x84\x01Ra@\x91\x81\x86a?\x04V[\x91PP\x82``\x83\x01R\x95\x94PPPPPV[\x84\x81R`\xFF\x84\x16` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\t\xEF`\x80\x83\x01\x84a4\xF8V[\x85\x81R`\xFF\x85\x16` \x82\x01R\x83`@\x82\x01R`\xA0``\x82\x01R`\0a@\xF3`\xA0\x83\x01\x85a4\xF8V[\x82\x81\x03`\x80\x84\x01RaA\x05\x81\x85a4\xF8V[\x98\x97PPPPPPPPV[`\0`\x01\x82\x01aA#WaA#a>8V[P`\x01\x01\x90V[`\0\x82QaA<\x81\x84` \x87\x01a4\xD4V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aAXW`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E\xECWa\x0E\xECa>8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aA\xA9WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFEGovernor: relay reverted without messageGovernor: call reverted without message\xA2dipfsX\"\x12 \"\xC0=~\xB3\xB2\xD3^+\xD8\xCA\xE75\x0E)\xABV\xBA\x14\xC7e2$#\x10\xA7\xC0\"P\xACubdsolcC\0\x08\x14\x003";
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
        ///Calls the contract's `submitContributionRequest` (0xbccf5ae4) function
        pub fn submit_contribution_request(
            &self,
            ipfs_hash: ::ethers::core::types::Bytes,
            description: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 207, 90, 228], (ipfs_hash, description))
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
    ///Container type for all input parameters for the `submitContributionRequest` function with signature `submitContributionRequest(bytes,string)` and selector `0xbccf5ae4`
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
        name = "submitContributionRequest",
        abi = "submitContributionRequest(bytes,string)"
    )]
    pub struct SubmitContributionRequestCall {
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
        GetVotes(GetVotesCall),
        GetVotesWithParams(GetVotesWithParamsCall),
        HasVoted(HasVotedCall),
        HashProposal(HashProposalCall),
        Initialize(InitializeCall),
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
        SubmitContributionRequest(SubmitContributionRequestCall),
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
                = <SubmitContributionRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SubmitContributionRequest(decoded));
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
                Self::SubmitContributionRequest(element) => {
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
                Self::GetVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotesWithParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasVoted(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::SubmitContributionRequest(element) => {
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
    impl ::core::convert::From<SubmitContributionRequestCall> for ProjectCalls {
        fn from(value: SubmitContributionRequestCall) -> Self {
            Self::SubmitContributionRequest(value)
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
