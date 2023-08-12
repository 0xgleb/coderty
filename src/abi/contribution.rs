pub use contribution::*;
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
pub mod contribution {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("delegate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegatee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("delegateBySig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegateBySig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegatee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegates"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("getPastTotalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPastTotalSupply"),
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
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPastVotes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPastVotes"),
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
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DelegateChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delegator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fromDelegate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("toDelegate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateVotesChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DelegateVotesChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delegate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newBalance"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CONTRIBUTION_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01``@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\x0C\x80\x82Rk!\xB7\xB7:94\xB1:\xBA4\xB7\xB7`\xA1\x1B` \x80\x84\x01\x82\x90R\x84Q\x80\x86\x01\x86R`\x01\x81R`1`\xF8\x1B\x81\x83\x01R\x85Q\x80\x87\x01\x87R\x93\x84R\x83\x82\x01\x92\x90\x92R\x84Q\x80\x86\x01\x90\x95R`\x04\x85Rc!\xA7*)`\xE1\x1B\x90\x85\x01R\x91\x92`\0b\0\0\x86\x83\x82b\0\x02\xEDV[P`\x01b\0\0\x95\x82\x82b\0\x02\xEDV[PPPb\0\0\xB2b\0\0\xACb\0\x01m` \x1B` \x1CV[b\0\x01qV[b\0\0\xBF\x82`\x07b\0\x01\xC3V[a\x01 Rb\0\0\xD0\x81`\x08b\0\x01\xC3V[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Rb\0\x01^`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0Rb\0\x04.V[3\x90V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0` \x83Q\x10\x15b\0\x01\xE3Wb\0\x01\xDB\x83b\0\x01\xFCV[\x90Pb\0\x01\xF6V[\x81b\0\x01\xF0\x84\x82b\0\x02\xEDV[P`\xFF\x90P[\x92\x91PPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15b\0\x023W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01b\0\x02*\x91\x90b\0\x03\xB9V[`@Q\x80\x91\x03\x90\xFD[\x80Qb\0\x02@\x82b\0\x04\tV[\x17\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02sW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x02\x94WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\xE8W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02\xC3WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\xE4W\x82\x81U`\x01\x01b\0\x02\xCFV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x03\tWb\0\x03\tb\0\x02HV[b\0\x03!\x81b\0\x03\x1A\x84Tb\0\x02^V[\x84b\0\x02\x9AV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03YW`\0\x84\x15b\0\x03@WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\xE4V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03\x8AW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03iV[P\x85\x82\x10\x15b\0\x03\xA9W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15b\0\x03\xE8W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01b\0\x03\xCAV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15b\0\x02\x94W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa+\x06b\0\x04\x89`\09`\0a\t\x9A\x01R`\0a\to\x01R`\0a\x10\x88\x01R`\0a\x10`\x01R`\0a\x0F\xBB\x01R`\0a\x0F\xE5\x01R`\0a\x10\x0F\x01Ra+\x06`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80cx[\xB0>\x11a\0\xF9W\x80c\x9A\xB2N\xB0\x11a\0\x97W\x80c\xC3\xCD\xA5 \x11a\0qW\x80c\xC3\xCD\xA5 \x14a\x03\xCFW\x80c\xC8{V\xDD\x14a\x03\xE2W\x80c\xE9\x85\xE9\xC5\x14a\x03\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\x041W`\0\x80\xFD[\x80c\x9A\xB2N\xB0\x14a\x03\x96W\x80c\xA2,\xB4e\x14a\x03\xA9W\x80c\xB8\x8DO\xDE\x14a\x03\xBCW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xD3W\x80c\x8D\xA5\xCB[\x14a\x03KW\x80c\x8ES\x9E\x8C\x14a\x03\\W\x80c\x91\xDD\xAD\xF4\x14a\x03oW\x80c\x95\xD8\x9BA\x14a\x03\x8EW`\0\x80\xFD[\x80cx[\xB0>\x14a\x03\nW\x80c~\xCE\xBE\0\x14a\x03\x1DW\x80c\x84\xB0\x19n\x14a\x030W`\0\x80\xFD[\x80cB\x84.\x0E\x11a\x01fW\x80c\\\x19\xA9\\\x11a\x01@W\x80c\\\x19\xA9\\\x14a\x02\xC9W\x80ccR!\x1E\x14a\x02\xDCW\x80cp\xA0\x821\x14a\x02\xEFW\x80cqP\x18\xA6\x14a\x03\x02W`\0\x80\xFD[\x80cB\x84.\x0E\x14a\x02\x82W\x80cK\xF5\xD7\xE9\x14a\x02\x95W\x80cX|\xDE\x1E\x14a\x02\x9DW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x01\xA2W\x80c\t^\xA7\xB3\x14a\x021W\x80c#\xB8r\xDD\x14a\x02FW\x80c6D\xE5\x15\x14a\x02YW\x80c:F\xB1\xA8\x14a\x02oW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xC9W\x80c\x06\xFD\xDE\x03\x14a\x01\xF1W\x80c\x08\x18\x12\xFC\x14a\x02\x06W[`\0\x80\xFD[a\x01\xDCa\x01\xD76`\x04a#?V[a\x04DV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9a\x04\x96V[`@Qa\x01\xE8\x91\x90a#\xACV[a\x02\x19a\x02\x146`\x04a#\xBFV[a\x05(V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE8V[a\x02Da\x02?6`\x04a#\xF4V[a\x05OV[\0[a\x02Da\x02T6`\x04a$\x1EV[a\x06iV[a\x02aa\x06\x9AV[`@Q\x90\x81R` \x01a\x01\xE8V[a\x02aa\x02}6`\x04a#\xF4V[a\x06\xA9V[a\x02Da\x02\x906`\x04a$\x1EV[a\x07:V[a\x01\xF9a\x07UV[a\x02\x19a\x02\xAB6`\x04a$ZV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\t` R`@\x90 T\x16\x90V[a\x02Da\x02\xD76`\x04a$ZV[a\x07\xEDV[a\x02\x19a\x02\xEA6`\x04a#\xBFV[a\x07\xFCV[a\x02aa\x02\xFD6`\x04a$ZV[a\x08\\V[a\x02Da\x08\xE2V[a\x02Da\x03\x186`\x04a$uV[a\x08\xF6V[a\x02aa\x03+6`\x04a$ZV[a\tCV[a\x038a\taV[`@Qa\x01\xE8\x97\x96\x95\x94\x93\x92\x91\x90a$\xF8V[`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x19V[a\x02aa\x03j6`\x04a#\xBFV[a\t\xEAV[a\x03wa\ndV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE8V[a\x01\xF9a\noV[a\x02aa\x03\xA46`\x04a$ZV[a\n~V[a\x02Da\x03\xB76`\x04a%\x8EV[a\n\x9FV[a\x02Da\x03\xCA6`\x04a%\xE0V[a\n\xAAV[a\x02Da\x03\xDD6`\x04a&\xBCV[a\n\xDCV[a\x01\xF9a\x03\xF06`\x04a#\xBFV[a\x0C\tV[a\x01\xDCa\x04\x036`\x04a'\x1CV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x02Da\x04?6`\x04a$ZV[a\x0C}V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x04uWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x04\x90WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\0\x80Ta\x04\xA5\x90a'OV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xD1\x90a'OV[\x80\x15a\x05\x1EW\x80`\x1F\x10a\x04\xF3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x1EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x01W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x053\x82a\x0C\xF6V[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x05Z\x82a\x07\xFCV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x05\xE8WPa\x05\xE8\x813a\x04\x03V[a\x06ZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\x05\xC3V[a\x06d\x83\x83a\rUV[PPPV[a\x06s3\x82a\r\xC3V[a\x06\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a'\x83V[a\x06d\x83\x83\x83a\x0EBV[`\0a\x06\xA4a\x0F\xAEV[\x90P\x90V[`\0a\x06\xB3a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x07\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`d\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\x07*a\x07\x0C\x83a\x10\xD9V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\n` R`@\x90 \x90a\x11BV[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x06d\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\n\xAAV[``Ca\x07`a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FVotes: broken clock mode\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[P`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R\x90V[3a\x07\xF8\x81\x83a\x11\xF7V[PPV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x04\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x05\xC3V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01Rh64\xB2\x107\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x08\xEAa\x12iV[a\x08\xF4`\0a\x12\xC3V[V[a\x08\xFEa\x12iV[`\0a\t\t`\rT\x90V[\x90Pa\t\x19`\r\x80T`\x01\x01\x90UV[`\0\x81\x81R`\x0E` R`@\x90 a\t2\x83\x85\x83a(\x1EV[Pa\t=\x84\x82a\x13\x15V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x81 Ta\x04\x90V[`\0``\x80\x82\x80\x80\x83a\t\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x07a\x13/V[a\t\xC0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08a\x13/V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0a\t\xF4a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`d\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\nUa\nM\x83a\x10\xD9V[`\x0B\x90a\x11BV[`\x01`\x01`\xE0\x1B\x03\x16\x92\x91PPV[`\0a\x06\xA4Ca\x13\xDAV[```\x01\x80Ta\x04\xA5\x90a'OV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x81 a\nU\x90a\x14AV[a\x07\xF83\x83\x83a\x14{V[a\n\xB43\x83a\r\xC3V[a\n\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a'\x83V[a\t=\x84\x84\x84\x84a\x15IV[\x83B\x11\x15a\x0B,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FVotes: signature expired\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x0B\xA6\x90a\x0B\x9E\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x15|V[\x85\x85\x85a\x15\xA9V[\x90Pa\x0B\xB1\x81a\x15\xD1V[\x86\x14a\x0B\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsVotes: invalid nonce``\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\x0C\0\x81\x88a\x11\xF7V[PPPPPPPV[``a\x0C\x14\x82a\x0C\xF6V[`\0a\x0C+`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\x0CKW`@Q\x80` \x01`@R\x80`\0\x81RPa\x0CvV[\x80a\x0CU\x84a\x15\xF9V[`@Q` \x01a\x0Cf\x92\x91\x90a(\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[a\x0C\x85a\x12iV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[a\x0C\xF3\x81a\x12\xC3V[PV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\r\x8A\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0\x80a\r\xCF\x83a\x07\xFCV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0E\x16WP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x0E:WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x0E/\x84a\x05(V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0EU\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x0EV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xF0\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x0EV[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4a\x06d\x83\x83\x83`\x01a\x16\x8CV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x10\x07WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x101WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x06\xA4`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x11\x9FW`\0a\x11]\x84a\x16\x98V[a\x11g\x90\x85a)iV[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x11\x8FW\x80\x91Pa\x11\x9DV[a\x11\x9A\x81`\x01a)|V[\x92P[P[`\0a\x11\xAD\x87\x87\x85\x85a\x17\x80V[\x90P\x80\x15a\x11\xE9Wa\x11\xD2\x87a\x11\xC4`\x01\x84a)iV[`\0\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x11\xECV[`\0[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\t` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x06d\x81\x83a\x12d\x86a\x17\xDEV[a\x17\xE9V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xC3V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[a\x07\xF8\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x19UV[```\xFF\x83\x14a\x13IWa\x13B\x83a\x19\x88V[\x90Pa\x04\x90V[\x81\x80Ta\x13U\x90a'OV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x81\x90a'OV[\x80\x15a\x13\xCEW\x80`\x1F\x10a\x13\xA3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xCEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x04\x90V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[\x80T`\0\x90\x80\x15a\x14rWa\x14[\x83a\x11\xC4`\x01\x84a)iV[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0CvV[`\0\x93\x92PPPV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x14\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x15T\x84\x84\x84a\x0EBV[a\x15`\x84\x84\x84\x84a\x19\xC7V[a\t=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[`\0a\x04\x90a\x15\x89a\x0F\xAEV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x15\xBA\x87\x87\x87\x87a\x1A\xC5V[\x91P\x91Pa\x15\xC7\x81a\x1B\x89V[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[```\0a\x16\x06\x83a\x1C\xD3V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16&Wa\x16&a%\xCAV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x16PW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x16ZWP\x93\x92PPPV[a\t=\x84\x84\x84\x84a\x1D\xABV[`\0\x81`\0\x03a\x16\xAAWP`\0\x91\x90PV[`\0`\x01a\x16\xB7\x84a\x1D\xBBV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x16\xD0Wa\x16\xD0a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x16\xE8Wa\x16\xE8a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17\0Wa\x17\0a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17\x18Wa\x17\x18a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x170Wa\x170a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17HWa\x17Ha)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17`Wa\x17`a)\xE1V[\x04\x82\x01\x90\x1C\x90Pa\x0Cv\x81\x82\x85\x81a\x17zWa\x17za)\xE1V[\x04a\x1EOV[`\0[\x81\x83\x10\x15a\x17\xD6W`\0a\x17\x97\x84\x84a\x1EeV[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xC2W\x80\x92Pa\x17\xD0V[a\x17\xCD\x81`\x01a)|V[\x93P[Pa\x17\x83V[P\x93\x92PPPV[`\0a\x04\x90\x82a\x08\\V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x18\x0BWP`\0\x81\x11[\x15a\x06dW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x18\xB3W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\n` R`@\x81 \x81\x90a\x18N\x90a\x1E\x80a\x18I\x86a\x1E\x8CV[a\x1E\xF5V[`\x01`\x01`\xE0\x1B\x03\x16\x91P`\x01`\x01`\xE0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x18\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x06dW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\n` R`@\x81 \x81\x90a\x18\xEC\x90a\x1F>a\x18I\x86a\x1E\x8CV[`\x01`\x01`\xE0\x1B\x03\x16\x91P`\x01`\x01`\xE0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x19F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x19_\x83\x83a\x1FJV[a\x19l`\0\x84\x84\x84a\x19\xC7V[a\x06dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[```\0a\x19\x95\x83a \xDFV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x1A\xBDW`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x1A\x0B\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a)\xF7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x1AFWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1AC\x91\x81\x01\x90a*4V[`\x01[a\x1A\xA3W=\x80\x80\x15a\x1AtW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1AyV[``\x91P[P\x80Q`\0\x03a\x1A\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x0E:V[P`\x01a\x0E:V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1A\xFCWP`\0\x90P`\x03a\x1B\x80V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1BPW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1ByW`\0`\x01\x92P\x92PPa\x1B\x80V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a\x1B\x9DWa\x1B\x9Da*QV[\x03a\x1B\xA5WPV[`\x01\x81`\x04\x81\x11\x15a\x1B\xB9Wa\x1B\xB9a*QV[\x03a\x1C\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x02\x81`\x04\x81\x11\x15a\x1C\x1AWa\x1C\x1Aa*QV[\x03a\x1CgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\xC3V[`\x03\x81`\x04\x81\x11\x15a\x1C{Wa\x1C{a*QV[\x03a\x0C\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x1D\x12Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x1D>Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x1D\\Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x1DtWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x1D\x88Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x1D\x9AW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x04\x90W`\x01\x01\x92\x91PPV[a\x1D\xB6\x84\x84\x83a!\x07V[a\t=V[`\0\x80`\x80\x83\x90\x1C\x15a\x1D\xD0W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a\x1D\xE2W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a\x1D\xF4W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a\x1E\x06W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a\x1E\x18W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a\x1E*W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a\x1E<W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x04\x90W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a\x1E^W\x81a\x0CvV[P\x90\x91\x90PV[`\0a\x1Et`\x02\x84\x84\x18a*gV[a\x0Cv\x90\x84\x84\x16a)|V[`\0a\x0Cv\x82\x84a*\x89V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[`\0\x80a\x1F1a\x1F\x13a\x1F\x06a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x10\xD9V[a\x1F)a\x1F\x1F\x88a\x14AV[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a!}V[\x91P\x91P[\x93P\x93\x91PPV[`\0a\x0Cv\x82\x84a*\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1F\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a \x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4a\x07\xF8`\0\x83\x83`\x01a\x16\x8CV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x04\x90W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a!)Wa!&`\x0Ba\x1F>a\x18I\x84a\x1E\x8CV[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a!KWa!H`\x0Ba\x1E\x80a\x18I\x84a\x1E\x8CV[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\t` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x06d\x92\x91\x82\x16\x91\x16\x83a\x17\xE9V[`\0\x80a\x1F1\x85\x85\x85\x82T`\0\x90\x81\x90\x80\x15a\"\xCFW`\0a!\xA4\x87a\x11\xC4`\x01\x85a)iV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a\"&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a\"oW\x84a\"G\x88a\x11\xC4`\x01\x86a)iV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\"\xBFV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16d\x01\0\0\0\0\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa\x1F6\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16d\x01\0\0\0\0\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a\x1F6V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C\xF3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#QW`\0\x80\xFD[\x815a\x0Cv\x81a#)V[`\0[\x83\x81\x10\x15a#wW\x81\x81\x01Q\x83\x82\x01R` \x01a#_V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra#\x98\x81` \x86\x01` \x86\x01a#\\V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0Cv` \x83\x01\x84a#\x80V[`\0` \x82\x84\x03\x12\x15a#\xD1W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a#\xEFW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\x07W`\0\x80\xFD[a$\x10\x83a#\xD8V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a$3W`\0\x80\xFD[a$<\x84a#\xD8V[\x92Pa$J` \x85\x01a#\xD8V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a$lW`\0\x80\xFD[a\x0Cv\x82a#\xD8V[`\0\x80`\0`@\x84\x86\x03\x12\x15a$\x8AW`\0\x80\xFD[a$\x93\x84a#\xD8V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\xB0W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a$\xC4W`\0\x80\xFD[\x815\x81\x81\x11\x15a$\xD3W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a$\xE5W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\xFF`\xF8\x1B\x88\x16\x81R`\0` `\xE0\x81\x84\x01Ra%\x18`\xE0\x84\x01\x8Aa#\x80V[\x83\x81\x03`@\x85\x01Ra%*\x81\x8Aa#\x80V[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R\x83\x87\x01\x92P\x90\x83\x01\x90`\0[\x81\x81\x10\x15a%|W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a%`V[P\x90\x9C\x9BPPPPPPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a%\xA1W`\0\x80\xFD[a%\xAA\x83a#\xD8V[\x91P` \x83\x015\x80\x15\x15\x81\x14a%\xBFW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a%\xF6W`\0\x80\xFD[a%\xFF\x85a#\xD8V[\x93Pa&\r` \x86\x01a#\xD8V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&1W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a&EW`\0\x80\xFD[\x815\x81\x81\x11\x15a&WWa&Wa%\xCAV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a&\x7FWa&\x7Fa%\xCAV[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a&\x98W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a&\xD5W`\0\x80\xFD[a&\xDE\x87a#\xD8V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a'\x02W`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a'/W`\0\x80\xFD[a'8\x83a#\xD8V[\x91Pa'F` \x84\x01a#\xD8V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a'cW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\xF3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`-\x90\x82\x01R\x7FERC721: caller is not token owne`@\x82\x01Rl\x1C\x88\x1B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\x9A\x1B``\x82\x01R`\x80\x01\x90V[`\x1F\x82\x11\x15a\x06dW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'\xF7WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a(\x16W\x82\x81U`\x01\x01a(\x03V[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a(6Wa(6a%\xCAV[a(J\x83a(D\x83Ta'OV[\x83a'\xD0V[`\0`\x1F\x84\x11`\x01\x81\x14a(~W`\0\x85\x15a(fWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua(\xD8V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a(\xAFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a(\x8FV[P\x86\x82\x10\x15a(\xCCW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x83Qa(\xF1\x81\x84` \x88\x01a#\\V[\x83Q\x90\x83\x01\x90a)\x05\x81\x83` \x88\x01a#\\V[\x01\x94\x93PPPPV[` \x80\x82R`%\x90\x82\x01R\x7FERC721: transfer from incorrect `@\x82\x01Rd7\xBB\xB72\xB9`\xD9\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\x90Wa\x04\x90a)SV[\x80\x82\x01\x80\x82\x11\x15a\x04\x90Wa\x04\x90a)SV[` \x80\x82R`2\x90\x82\x01R\x7FERC721: transfer to non ERC721Re`@\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a**\x90\x83\x01\x84a#\x80V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a*FW`\0\x80\xFD[\x81Qa\x0Cv\x81a#)V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82a*\x84WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01`\x01`\xE0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a*\xA9Wa*\xA9a)SV[P\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a*\xA9Wa*\xA9a)SV\xFE\xA2dipfsX\"\x12 v\xB5\x89\x12\x0C\xB0(\xC78\xCA\xA4\xD5\x8B\xEE\xA20@\xD7\x1B\xEEL\x85'%#`|\xED\xF2\x1A\\VdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static CONTRIBUTION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80cx[\xB0>\x11a\0\xF9W\x80c\x9A\xB2N\xB0\x11a\0\x97W\x80c\xC3\xCD\xA5 \x11a\0qW\x80c\xC3\xCD\xA5 \x14a\x03\xCFW\x80c\xC8{V\xDD\x14a\x03\xE2W\x80c\xE9\x85\xE9\xC5\x14a\x03\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\x041W`\0\x80\xFD[\x80c\x9A\xB2N\xB0\x14a\x03\x96W\x80c\xA2,\xB4e\x14a\x03\xA9W\x80c\xB8\x8DO\xDE\x14a\x03\xBCW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xD3W\x80c\x8D\xA5\xCB[\x14a\x03KW\x80c\x8ES\x9E\x8C\x14a\x03\\W\x80c\x91\xDD\xAD\xF4\x14a\x03oW\x80c\x95\xD8\x9BA\x14a\x03\x8EW`\0\x80\xFD[\x80cx[\xB0>\x14a\x03\nW\x80c~\xCE\xBE\0\x14a\x03\x1DW\x80c\x84\xB0\x19n\x14a\x030W`\0\x80\xFD[\x80cB\x84.\x0E\x11a\x01fW\x80c\\\x19\xA9\\\x11a\x01@W\x80c\\\x19\xA9\\\x14a\x02\xC9W\x80ccR!\x1E\x14a\x02\xDCW\x80cp\xA0\x821\x14a\x02\xEFW\x80cqP\x18\xA6\x14a\x03\x02W`\0\x80\xFD[\x80cB\x84.\x0E\x14a\x02\x82W\x80cK\xF5\xD7\xE9\x14a\x02\x95W\x80cX|\xDE\x1E\x14a\x02\x9DW`\0\x80\xFD[\x80c\t^\xA7\xB3\x11a\x01\xA2W\x80c\t^\xA7\xB3\x14a\x021W\x80c#\xB8r\xDD\x14a\x02FW\x80c6D\xE5\x15\x14a\x02YW\x80c:F\xB1\xA8\x14a\x02oW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xC9W\x80c\x06\xFD\xDE\x03\x14a\x01\xF1W\x80c\x08\x18\x12\xFC\x14a\x02\x06W[`\0\x80\xFD[a\x01\xDCa\x01\xD76`\x04a#?V[a\x04DV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9a\x04\x96V[`@Qa\x01\xE8\x91\x90a#\xACV[a\x02\x19a\x02\x146`\x04a#\xBFV[a\x05(V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE8V[a\x02Da\x02?6`\x04a#\xF4V[a\x05OV[\0[a\x02Da\x02T6`\x04a$\x1EV[a\x06iV[a\x02aa\x06\x9AV[`@Q\x90\x81R` \x01a\x01\xE8V[a\x02aa\x02}6`\x04a#\xF4V[a\x06\xA9V[a\x02Da\x02\x906`\x04a$\x1EV[a\x07:V[a\x01\xF9a\x07UV[a\x02\x19a\x02\xAB6`\x04a$ZV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\t` R`@\x90 T\x16\x90V[a\x02Da\x02\xD76`\x04a$ZV[a\x07\xEDV[a\x02\x19a\x02\xEA6`\x04a#\xBFV[a\x07\xFCV[a\x02aa\x02\xFD6`\x04a$ZV[a\x08\\V[a\x02Da\x08\xE2V[a\x02Da\x03\x186`\x04a$uV[a\x08\xF6V[a\x02aa\x03+6`\x04a$ZV[a\tCV[a\x038a\taV[`@Qa\x01\xE8\x97\x96\x95\x94\x93\x92\x91\x90a$\xF8V[`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x19V[a\x02aa\x03j6`\x04a#\xBFV[a\t\xEAV[a\x03wa\ndV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE8V[a\x01\xF9a\noV[a\x02aa\x03\xA46`\x04a$ZV[a\n~V[a\x02Da\x03\xB76`\x04a%\x8EV[a\n\x9FV[a\x02Da\x03\xCA6`\x04a%\xE0V[a\n\xAAV[a\x02Da\x03\xDD6`\x04a&\xBCV[a\n\xDCV[a\x01\xF9a\x03\xF06`\x04a#\xBFV[a\x0C\tV[a\x01\xDCa\x04\x036`\x04a'\x1CV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x02Da\x04?6`\x04a$ZV[a\x0C}V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x04uWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x04\x90WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\0\x80Ta\x04\xA5\x90a'OV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xD1\x90a'OV[\x80\x15a\x05\x1EW\x80`\x1F\x10a\x04\xF3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x1EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x01W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x053\x82a\x0C\xF6V[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x05Z\x82a\x07\xFCV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC721: approval to current owne`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x05\xE8WPa\x05\xE8\x813a\x04\x03V[a\x06ZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FERC721: approve caller is not to`D\x82\x01R\x7Fken owner or approved for all\0\0\0`d\x82\x01R`\x84\x01a\x05\xC3V[a\x06d\x83\x83a\rUV[PPPV[a\x06s3\x82a\r\xC3V[a\x06\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a'\x83V[a\x06d\x83\x83\x83a\x0EBV[`\0a\x06\xA4a\x0F\xAEV[\x90P\x90V[`\0a\x06\xB3a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x07\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`d\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\x07*a\x07\x0C\x83a\x10\xD9V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\n` R`@\x90 \x90a\x11BV[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[a\x06d\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\n\xAAV[``Ca\x07`a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FVotes: broken clock mode\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[P`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7Fmode=blocknumber&from=default\0\0\0` \x82\x01R\x90V[3a\x07\xF8\x81\x83a\x11\xF7V[PPV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x04\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x05\xC3V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC721: address zero is not a va`D\x82\x01Rh64\xB2\x107\xBB\xB72\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x08\xEAa\x12iV[a\x08\xF4`\0a\x12\xC3V[V[a\x08\xFEa\x12iV[`\0a\t\t`\rT\x90V[\x90Pa\t\x19`\r\x80T`\x01\x01\x90UV[`\0\x81\x81R`\x0E` R`@\x90 a\t2\x83\x85\x83a(\x1EV[Pa\t=\x84\x82a\x13\x15V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x81 Ta\x04\x90V[`\0``\x80\x82\x80\x80\x83a\t\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x07a\x13/V[a\t\xC0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08a\x13/V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0a\t\xF4a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`d\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\nUa\nM\x83a\x10\xD9V[`\x0B\x90a\x11BV[`\x01`\x01`\xE0\x1B\x03\x16\x92\x91PPV[`\0a\x06\xA4Ca\x13\xDAV[```\x01\x80Ta\x04\xA5\x90a'OV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x81 a\nU\x90a\x14AV[a\x07\xF83\x83\x83a\x14{V[a\n\xB43\x83a\r\xC3V[a\n\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a'\x83V[a\t=\x84\x84\x84\x84a\x15IV[\x83B\x11\x15a\x0B,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FVotes: signature expired\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x0B\xA6\x90a\x0B\x9E\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x15|V[\x85\x85\x85a\x15\xA9V[\x90Pa\x0B\xB1\x81a\x15\xD1V[\x86\x14a\x0B\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsVotes: invalid nonce``\x1B`D\x82\x01R`d\x01a\x05\xC3V[a\x0C\0\x81\x88a\x11\xF7V[PPPPPPPV[``a\x0C\x14\x82a\x0C\xF6V[`\0a\x0C+`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\x0CKW`@Q\x80` \x01`@R\x80`\0\x81RPa\x0CvV[\x80a\x0CU\x84a\x15\xF9V[`@Q` \x01a\x0Cf\x92\x91\x90a(\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[a\x0C\x85a\x12iV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[a\x0C\xF3\x81a\x12\xC3V[PV[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01Rw\x11T\x90\xCD\xCC\x8CN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`B\x1B`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\r\x8A\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0\x80a\r\xCF\x83a\x07\xFCV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0E\x16WP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R T`\xFF\x16[\x80a\x0E:WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x0E/\x84a\x05(V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0EU\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x0EV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC721: transfer to the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xF0\x82a\x07\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x0EV[`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x80\x86R`\x03\x85R\x83\x86 \x80T`\0\x19\x01\x90U\x90\x87\x16\x80\x86R\x83\x86 \x80T`\x01\x01\x90U\x86\x86R`\x02\x90\x94R\x82\x85 \x80T\x90\x92\x16\x84\x17\x90\x91U\x90Q\x84\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4a\x06d\x83\x83\x83`\x01a\x16\x8CV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x10\x07WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x101WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x06\xA4`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[P\x90V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x11\x9FW`\0a\x11]\x84a\x16\x98V[a\x11g\x90\x85a)iV[`\0\x88\x81R` \x90 \x90\x91P\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x11\x8FW\x80\x91Pa\x11\x9DV[a\x11\x9A\x81`\x01a)|V[\x92P[P[`\0a\x11\xAD\x87\x87\x85\x85a\x17\x80V[\x90P\x80\x15a\x11\xE9Wa\x11\xD2\x87a\x11\xC4`\x01\x84a)iV[`\0\x91\x82R` \x90\x91 \x01\x90V[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x11\xECV[`\0[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\t` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x06d\x81\x83a\x12d\x86a\x17\xDEV[a\x17\xE9V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xC3V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[a\x07\xF8\x82\x82`@Q\x80` \x01`@R\x80`\0\x81RPa\x19UV[```\xFF\x83\x14a\x13IWa\x13B\x83a\x19\x88V[\x90Pa\x04\x90V[\x81\x80Ta\x13U\x90a'OV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x81\x90a'OV[\x80\x15a\x13\xCEW\x80`\x1F\x10a\x13\xA3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xCEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x04\x90V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[\x80T`\0\x90\x80\x15a\x14rWa\x14[\x83a\x11\xC4`\x01\x84a)iV[Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0CvV[`\0\x93\x92PPPV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x14\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x15T\x84\x84\x84a\x0EBV[a\x15`\x84\x84\x84\x84a\x19\xC7V[a\t=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[`\0a\x04\x90a\x15\x89a\x0F\xAEV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x15\xBA\x87\x87\x87\x87a\x1A\xC5V[\x91P\x91Pa\x15\xC7\x81a\x1B\x89V[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[```\0a\x16\x06\x83a\x1C\xD3V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16&Wa\x16&a%\xCAV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x16PW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x16ZWP\x93\x92PPPV[a\t=\x84\x84\x84\x84a\x1D\xABV[`\0\x81`\0\x03a\x16\xAAWP`\0\x91\x90PV[`\0`\x01a\x16\xB7\x84a\x1D\xBBV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x16\xD0Wa\x16\xD0a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x16\xE8Wa\x16\xE8a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17\0Wa\x17\0a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17\x18Wa\x17\x18a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x170Wa\x170a)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17HWa\x17Ha)\xE1V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x17`Wa\x17`a)\xE1V[\x04\x82\x01\x90\x1C\x90Pa\x0Cv\x81\x82\x85\x81a\x17zWa\x17za)\xE1V[\x04a\x1EOV[`\0[\x81\x83\x10\x15a\x17\xD6W`\0a\x17\x97\x84\x84a\x1EeV[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xC2W\x80\x92Pa\x17\xD0V[a\x17\xCD\x81`\x01a)|V[\x93P[Pa\x17\x83V[P\x93\x92PPPV[`\0a\x04\x90\x82a\x08\\V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x18\x0BWP`\0\x81\x11[\x15a\x06dW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x18\xB3W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\n` R`@\x81 \x81\x90a\x18N\x90a\x1E\x80a\x18I\x86a\x1E\x8CV[a\x1E\xF5V[`\x01`\x01`\xE0\x1B\x03\x16\x91P`\x01`\x01`\xE0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x18\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x06dW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\n` R`@\x81 \x81\x90a\x18\xEC\x90a\x1F>a\x18I\x86a\x1E\x8CV[`\x01`\x01`\xE0\x1B\x03\x16\x91P`\x01`\x01`\xE0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\x19F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x19_\x83\x83a\x1FJV[a\x19l`\0\x84\x84\x84a\x19\xC7V[a\x06dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[```\0a\x19\x95\x83a \xDFV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x1A\xBDW`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x1A\x0B\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a)\xF7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x1AFWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1AC\x91\x81\x01\x90a*4V[`\x01[a\x1A\xA3W=\x80\x80\x15a\x1AtW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1AyV[``\x91P[P\x80Q`\0\x03a\x1A\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC3\x90a)\x8FV[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x0E:V[P`\x01a\x0E:V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1A\xFCWP`\0\x90P`\x03a\x1B\x80V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1BPW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1ByW`\0`\x01\x92P\x92PPa\x1B\x80V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a\x1B\x9DWa\x1B\x9Da*QV[\x03a\x1B\xA5WPV[`\x01\x81`\x04\x81\x11\x15a\x1B\xB9Wa\x1B\xB9a*QV[\x03a\x1C\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x02\x81`\x04\x81\x11\x15a\x1C\x1AWa\x1C\x1Aa*QV[\x03a\x1CgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x05\xC3V[`\x03\x81`\x04\x81\x11\x15a\x1C{Wa\x1C{a*QV[\x03a\x0C\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x1D\x12Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x1D>Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x1D\\Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x1DtWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x1D\x88Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x1D\x9AW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x04\x90W`\x01\x01\x92\x91PPV[a\x1D\xB6\x84\x84\x83a!\x07V[a\t=V[`\0\x80`\x80\x83\x90\x1C\x15a\x1D\xD0W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a\x1D\xE2W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a\x1D\xF4W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a\x1E\x06W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a\x1E\x18W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a\x1E*W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a\x1E<W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x04\x90W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a\x1E^W\x81a\x0CvV[P\x90\x91\x90PV[`\0a\x1Et`\x02\x84\x84\x18a*gV[a\x0Cv\x90\x84\x84\x16a)|V[`\0a\x0Cv\x82\x84a*\x89V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x11>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x05\xC3V[`\0\x80a\x1F1a\x1F\x13a\x1F\x06a\ndV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x10\xD9V[a\x1F)a\x1F\x1F\x88a\x14AV[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a!}V[\x91P\x91P[\x93P\x93\x91PPV[`\0a\x0Cv\x82\x84a*\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1F\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a \x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T`\x01\x01\x90U\x84\x83R`\x02\x90\x91R\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90UQ\x83\x92\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4a\x07\xF8`\0\x83\x83`\x01a\x16\x8CV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x04\x90W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16a!)Wa!&`\x0Ba\x1F>a\x18I\x84a\x1E\x8CV[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a!KWa!H`\x0Ba\x1E\x80a\x18I\x84a\x1E\x8CV[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\t` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x06d\x92\x91\x82\x16\x91\x16\x83a\x17\xE9V[`\0\x80a\x1F1\x85\x85\x85\x82T`\0\x90\x81\x90\x80\x15a\"\xCFW`\0a!\xA4\x87a\x11\xC4`\x01\x85a)iV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84Rd\x01\0\0\0\0\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a\"&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FCheckpoint: decreasing keys\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC3V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a\"oW\x84a\"G\x88a\x11\xC4`\x01\x86a)iV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\"\xBFV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16d\x01\0\0\0\0\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa\x1F6\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xE0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16d\x01\0\0\0\0\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a\x1F6V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C\xF3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#QW`\0\x80\xFD[\x815a\x0Cv\x81a#)V[`\0[\x83\x81\x10\x15a#wW\x81\x81\x01Q\x83\x82\x01R` \x01a#_V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra#\x98\x81` \x86\x01` \x86\x01a#\\V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0Cv` \x83\x01\x84a#\x80V[`\0` \x82\x84\x03\x12\x15a#\xD1W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a#\xEFW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\x07W`\0\x80\xFD[a$\x10\x83a#\xD8V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a$3W`\0\x80\xFD[a$<\x84a#\xD8V[\x92Pa$J` \x85\x01a#\xD8V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a$lW`\0\x80\xFD[a\x0Cv\x82a#\xD8V[`\0\x80`\0`@\x84\x86\x03\x12\x15a$\x8AW`\0\x80\xFD[a$\x93\x84a#\xD8V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\xB0W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a$\xC4W`\0\x80\xFD[\x815\x81\x81\x11\x15a$\xD3W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a$\xE5W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\xFF`\xF8\x1B\x88\x16\x81R`\0` `\xE0\x81\x84\x01Ra%\x18`\xE0\x84\x01\x8Aa#\x80V[\x83\x81\x03`@\x85\x01Ra%*\x81\x8Aa#\x80V[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R\x83\x87\x01\x92P\x90\x83\x01\x90`\0[\x81\x81\x10\x15a%|W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a%`V[P\x90\x9C\x9BPPPPPPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a%\xA1W`\0\x80\xFD[a%\xAA\x83a#\xD8V[\x91P` \x83\x015\x80\x15\x15\x81\x14a%\xBFW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a%\xF6W`\0\x80\xFD[a%\xFF\x85a#\xD8V[\x93Pa&\r` \x86\x01a#\xD8V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&1W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a&EW`\0\x80\xFD[\x815\x81\x81\x11\x15a&WWa&Wa%\xCAV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a&\x7FWa&\x7Fa%\xCAV[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a&\x98W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a&\xD5W`\0\x80\xFD[a&\xDE\x87a#\xD8V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a'\x02W`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a'/W`\0\x80\xFD[a'8\x83a#\xD8V[\x91Pa'F` \x84\x01a#\xD8V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a'cW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\xF3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`-\x90\x82\x01R\x7FERC721: caller is not token owne`@\x82\x01Rl\x1C\x88\x1B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`\x9A\x1B``\x82\x01R`\x80\x01\x90V[`\x1F\x82\x11\x15a\x06dW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'\xF7WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a(\x16W\x82\x81U`\x01\x01a(\x03V[PPPPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a(6Wa(6a%\xCAV[a(J\x83a(D\x83Ta'OV[\x83a'\xD0V[`\0`\x1F\x84\x11`\x01\x81\x14a(~W`\0\x85\x15a(fWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua(\xD8V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a(\xAFW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a(\x8FV[P\x86\x82\x10\x15a(\xCCW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[`\0\x83Qa(\xF1\x81\x84` \x88\x01a#\\V[\x83Q\x90\x83\x01\x90a)\x05\x81\x83` \x88\x01a#\\V[\x01\x94\x93PPPPV[` \x80\x82R`%\x90\x82\x01R\x7FERC721: transfer from incorrect `@\x82\x01Rd7\xBB\xB72\xB9`\xD9\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\x90Wa\x04\x90a)SV[\x80\x82\x01\x80\x82\x11\x15a\x04\x90Wa\x04\x90a)SV[` \x80\x82R`2\x90\x82\x01R\x7FERC721: transfer to non ERC721Re`@\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a**\x90\x83\x01\x84a#\x80V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a*FW`\0\x80\xFD[\x81Qa\x0Cv\x81a#)V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82a*\x84WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x01`\x01`\xE0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a*\xA9Wa*\xA9a)SV[P\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a*\xA9Wa*\xA9a)SV\xFE\xA2dipfsX\"\x12 v\xB5\x89\x12\x0C\xB0(\xC78\xCA\xA4\xD5\x8B\xEE\xA20@\xD7\x1B\xEEL\x85'%#`|\xED\xF2\x1A\\VdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static CONTRIBUTION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Contribution<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Contribution<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Contribution<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Contribution<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Contribution<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Contribution))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Contribution<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONTRIBUTION_ABI.clone(),
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
                CONTRIBUTION_ABI.clone(),
                CONTRIBUTION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CLOCK_MODE` (0x4bf5d7e9) function
        pub fn clock_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([75, 245, 215, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clock` (0x91ddadf4) function
        pub fn clock(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([145, 221, 173, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegate` (0x5c19a95c) function
        pub fn delegate(
            &self,
            delegatee: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 25, 169, 92], delegatee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateBySig` (0xc3cda520) function
        pub fn delegate_by_sig(
            &self,
            delegatee: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 205, 165, 32], (delegatee, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegates` (0x587cde1e) function
        pub fn delegates(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([88, 124, 222, 30], account)
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
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPastTotalSupply` (0x8e539e8c) function
        pub fn get_past_total_supply(
            &self,
            timepoint: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 83, 158, 140], timepoint)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPastVotes` (0x3a46b1a8) function
        pub fn get_past_votes(
            &self,
            account: ::ethers::core::types::Address,
            timepoint: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 70, 177, 168], (account, timepoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotes` (0x9ab24eb0) function
        pub fn get_votes(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 178, 78, 176], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
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
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeMint` (0x785bb03e) function
        pub fn safe_mint(
            &self,
            to: ::ethers::core::types::Address,
            ipfs_hash: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 91, 176, 62], (to, ipfs_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
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
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DelegateChanged` event
        pub fn delegate_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelegateChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DelegateVotesChanged` event
        pub fn delegate_votes_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelegateVotesChangedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContributionEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Contribution<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    pub enum ContributionErrors {
        InvalidShortString(InvalidShortString),
        StringTooLong(StringTooLong),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ContributionErrors {
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
    impl ::ethers::core::abi::AbiEncode for ContributionErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
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
    impl ::ethers::contract::ContractRevert for ContributionErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
    impl ::core::fmt::Display for ContributionErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidShortString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ContributionErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for ContributionErrors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for ContributionErrors {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
        name = "DelegateChanged",
        abi = "DelegateChanged(address,address,address)"
    )]
    pub struct DelegateChangedFilter {
        #[ethevent(indexed)]
        pub delegator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from_delegate: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_delegate: ::ethers::core::types::Address,
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
        name = "DelegateVotesChanged",
        abi = "DelegateVotesChanged(address,uint256,uint256)"
    )]
    pub struct DelegateVotesChangedFilter {
        #[ethevent(indexed)]
        pub delegate: ::ethers::core::types::Address,
        pub previous_balance: ::ethers::core::types::U256,
        pub new_balance: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ContributionEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        DelegateChangedFilter(DelegateChangedFilter),
        DelegateVotesChangedFilter(DelegateVotesChangedFilter),
        Eip712DomainChangedFilter(Eip712DomainChangedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for ContributionEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ContributionEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ContributionEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = DelegateChangedFilter::decode_log(log) {
                return Ok(ContributionEvents::DelegateChangedFilter(decoded));
            }
            if let Ok(decoded) = DelegateVotesChangedFilter::decode_log(log) {
                return Ok(ContributionEvents::DelegateVotesChangedFilter(decoded));
            }
            if let Ok(decoded) = Eip712DomainChangedFilter::decode_log(log) {
                return Ok(ContributionEvents::Eip712DomainChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ContributionEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ContributionEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ContributionEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegateChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegateVotesChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Eip712DomainChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ContributionEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ContributionEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<DelegateChangedFilter> for ContributionEvents {
        fn from(value: DelegateChangedFilter) -> Self {
            Self::DelegateChangedFilter(value)
        }
    }
    impl ::core::convert::From<DelegateVotesChangedFilter> for ContributionEvents {
        fn from(value: DelegateVotesChangedFilter) -> Self {
            Self::DelegateVotesChangedFilter(value)
        }
    }
    impl ::core::convert::From<Eip712DomainChangedFilter> for ContributionEvents {
        fn from(value: Eip712DomainChangedFilter) -> Self {
            Self::Eip712DomainChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ContributionEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ContributionEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `delegate` function with signature `delegate(address)` and selector `0x5c19a95c`
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
    #[ethcall(name = "delegate", abi = "delegate(address)")]
    pub struct DelegateCall {
        pub delegatee: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `delegateBySig` function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc3cda520`
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
        name = "delegateBySig",
        abi = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegateBySigCall {
        pub delegatee: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `delegates` function with signature `delegates(address)` and selector `0x587cde1e`
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
    #[ethcall(name = "delegates", abi = "delegates(address)")]
    pub struct DelegatesCall {
        pub account: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPastTotalSupply` function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`
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
    #[ethcall(name = "getPastTotalSupply", abi = "getPastTotalSupply(uint256)")]
    pub struct GetPastTotalSupplyCall {
        pub timepoint: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPastVotes` function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`
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
    #[ethcall(name = "getPastVotes", abi = "getPastVotes(address,uint256)")]
    pub struct GetPastVotesCall {
        pub account: ::ethers::core::types::Address,
        pub timepoint: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getVotes` function with signature `getVotes(address)` and selector `0x9ab24eb0`
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
    #[ethcall(name = "getVotes", abi = "getVotes(address)")]
    pub struct GetVotesCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `safeMint` function with signature `safeMint(address,bytes)` and selector `0x785bb03e`
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
    #[ethcall(name = "safeMint", abi = "safeMint(address,bytes)")]
    pub struct SafeMintCall {
        pub to: ::ethers::core::types::Address,
        pub ipfs_hash: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
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
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ContributionCalls {
        ClockMode(ClockModeCall),
        DomainSeparator(DomainSeparatorCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Clock(ClockCall),
        Delegate(DelegateCall),
        DelegateBySig(DelegateBySigCall),
        Delegates(DelegatesCall),
        Eip712Domain(Eip712DomainCall),
        GetApproved(GetApprovedCall),
        GetPastTotalSupply(GetPastTotalSupplyCall),
        GetPastVotes(GetPastVotesCall),
        GetVotes(GetVotesCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        RenounceOwnership(RenounceOwnershipCall),
        SafeMint(SafeMintCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for ContributionCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ClockModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClockMode(decoded));
            }
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <ClockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Clock(decoded));
            }
            if let Ok(decoded)
                = <DelegateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegate(decoded));
            }
            if let Ok(decoded)
                = <DelegateBySigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelegateBySig(decoded));
            }
            if let Ok(decoded)
                = <DelegatesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegates(decoded));
            }
            if let Ok(decoded)
                = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded)
                = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded)
                = <GetPastTotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPastTotalSupply(decoded));
            }
            if let Ok(decoded)
                = <GetPastVotesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPastVotes(decoded));
            }
            if let Ok(decoded)
                = <GetVotesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVotes(decoded));
            }
            if let Ok(decoded)
                = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <SafeMintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SafeMint(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded)
                = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ContributionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClockMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Clock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Delegate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateBySig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712Domain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPastTotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPastVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVotes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ContributionCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClockMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateBySig(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegates(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPastTotalSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPastVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClockModeCall> for ContributionCalls {
        fn from(value: ClockModeCall) -> Self {
            Self::ClockMode(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for ContributionCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for ContributionCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ContributionCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ClockCall> for ContributionCalls {
        fn from(value: ClockCall) -> Self {
            Self::Clock(value)
        }
    }
    impl ::core::convert::From<DelegateCall> for ContributionCalls {
        fn from(value: DelegateCall) -> Self {
            Self::Delegate(value)
        }
    }
    impl ::core::convert::From<DelegateBySigCall> for ContributionCalls {
        fn from(value: DelegateBySigCall) -> Self {
            Self::DelegateBySig(value)
        }
    }
    impl ::core::convert::From<DelegatesCall> for ContributionCalls {
        fn from(value: DelegatesCall) -> Self {
            Self::Delegates(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for ContributionCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for ContributionCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<GetPastTotalSupplyCall> for ContributionCalls {
        fn from(value: GetPastTotalSupplyCall) -> Self {
            Self::GetPastTotalSupply(value)
        }
    }
    impl ::core::convert::From<GetPastVotesCall> for ContributionCalls {
        fn from(value: GetPastVotesCall) -> Self {
            Self::GetPastVotes(value)
        }
    }
    impl ::core::convert::From<GetVotesCall> for ContributionCalls {
        fn from(value: GetVotesCall) -> Self {
            Self::GetVotes(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for ContributionCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<NameCall> for ContributionCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for ContributionCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ContributionCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for ContributionCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ContributionCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SafeMintCall> for ContributionCalls {
        fn from(value: SafeMintCall) -> Self {
            Self::SafeMint(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for ContributionCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
    for ContributionCalls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for ContributionCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ContributionCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ContributionCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for ContributionCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ContributionCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ContributionCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
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
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `delegates` function with signature `delegates(address)` and selector `0x587cde1e`
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
    pub struct DelegatesReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPastTotalSupply` function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`
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
    pub struct GetPastTotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPastVotes` function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`
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
    pub struct GetPastVotesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVotes` function with signature `getVotes(address)` and selector `0x9ab24eb0`
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
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
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
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
}
