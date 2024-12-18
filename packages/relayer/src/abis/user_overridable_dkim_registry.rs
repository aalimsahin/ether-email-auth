pub use user_overridable_dkim_registry::*;
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
pub mod user_overridable_dkim_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("REACTIVATE_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("REACTIVATE_PREFIX"),
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
                    ::std::borrow::ToOwned::to_owned("REVOKE_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("REVOKE_PREFIX"),
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
                    ::std::borrow::ToOwned::to_owned("SET_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SET_PREFIX"),
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
                    ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UPGRADE_INTERFACE_VERSION",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("changeMainAuthorizer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeMainAuthorizer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMainAuthorizer"),
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
                    ::std::borrow::ToOwned::to_owned("computeSignedMsg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeSignedMsg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prefix"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
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
                    ::std::borrow::ToOwned::to_owned("dkimPublicKeyHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "dkimPublicKeyHashes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("enabledTimeOfDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "enabledTimeOfDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_initialOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_mainAuthorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_setTimestampDelay",
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
                    ::std::borrow::ToOwned::to_owned("isDKIMPublicKeyHashValid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isDKIMPublicKeyHashValid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isDKIMPublicKeyHashValid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
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
                    ::std::borrow::ToOwned::to_owned("mainAuthorizer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mainAuthorizer"),
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
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
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
                    ::std::borrow::ToOwned::to_owned("reactivateDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reactivateDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                    ::std::borrow::ToOwned::to_owned("reactivatedDKIMPublicKeyHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reactivatedDKIMPublicKeyHashes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("revokeDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "revokeDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                    ::std::borrow::ToOwned::to_owned("revokedDKIMPublicKeyHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "revokedDKIMPublicKeyHashes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("setDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                    ::std::borrow::ToOwned::to_owned("setDKIMPublicKeyHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDKIMPublicKeyHashes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainNames"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHashes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizers"),
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
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
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
                    ::std::borrow::ToOwned::to_owned("setTimestampDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTimestampDelay"),
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
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DKIMPublicKeyHashReactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DKIMPublicKeyHashReactivated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DKIMPublicKeyHashRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DKIMPublicKeyHashRegistered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DKIMPublicKeyHashRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DKIMPublicKeyHashRevoked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MainAuthorizerChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MainAuthorizerChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMainAuthorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignature",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureS",
                            ),
                            inputs: ::std::vec![
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC1967InvalidImplementation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidInitialization",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitializing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotInitializing"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StringsInsufficientHexLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StringsInsufficientHexLength",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UUPSUnauthorizedCallContext",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UUPSUnsupportedProxiableUUID",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
    pub static USEROVERRIDABLEDKIMREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15`\x13W`\0\x80\xFD[P`\x80Qa8ra\0=`\09`\0\x81\x81a\"\"\x01R\x81\x81a\"K\x01Ra$l\x01Ra8r`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x8BW`\x005`\xE0\x1C\x80c}F6H\x11a\0\xD6W\x80c\xAD<\xB1\xCC\x11a\0\x7FW\x80c\xE7\xA7\x97z\x11a\0YW\x80c\xE7\xA7\x97z\x14a\x05\x92W\x80c\xF0\xBF\xB1\x97\x14a\x05\xB2W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xEDW`\0\x80\xFD[\x80c\xAD<\xB1\xCC\x14a\x04\xAFW\x80c\xD5\x07\xC3 \x14a\x04\xF8W\x80c\xE3\x08\xDE\x0C\x14a\x05AW`\0\x80\xFD[\x80c\x81.\x12\xCE\x11a\0\xB0W\x80c\x81.\x12\xCE\x14a\x04/W\x80c\x82\xBF\xF8\xCD\x14a\x04EW\x80c\x8D\xA5\xCB[\x14a\x04eW`\0\x80\xFD[\x80c}F6H\x14a\x03\x90W\x80c\x7F\x8E)\xBA\x14a\x03\xE2W\x80c\x7F\xF1\x03\xDA\x14a\x04\x0FW`\0\x80\xFD[\x80cL\x93\x06\x07\x11a\x018W\x80cWI\0\xDD\x11a\x01\x12W\x80cWI\0\xDD\x14a\x03 W\x80caJD\x85\x14a\x03[W\x80cqP\x18\xA6\x14a\x03{W`\0\x80\xFD[\x80cL\x93\x06\x07\x14a\x02\xCAW\x80cO\x1E\xF2\x86\x14a\x02\xEAW\x80cR\xD1\x90-\x14a\x02\xFDW`\0\x80\xFD[\x80c\"Z\x08\xD4\x11a\x01iW\x80c\"Z\x08\xD4\x14a\x02AW\x80c2\xE1\xE1\x94\x14a\x02\x8AW\x80cK\xCB\xBE\x96\x14a\x02\xAAW`\0\x80\xFD[\x80c\x07\xF1\xEA\xF5\x14a\x01\x90W\x80c\x0BU\xB3|\x14a\x01\xEFW\x80c\x17\x94\xBB<\x14a\x02\x1FW[`\0\x80\xFD[4\x80\x15a\x01\x9CW`\0\x80\xFD[Pa\x01\xD9`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\xE6\x91\x90a.\xF4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFBW`\0\x80\xFD[Pa\x02\x0Fa\x02\n6`\x04a0<V[a\x06\rV[`@Q\x90\x15\x15\x81R` \x01a\x01\xE6V[4\x80\x15a\x02+W`\0\x80\xFD[Pa\x02?a\x02:6`\x04a0\x97V[a\x08\x1DV[\0[4\x80\x15a\x02MW`\0\x80\xFD[Pa\x01\xD9`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01\x7FREACTIVATE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x02\x96W`\0\x80\xFD[Pa\x02?a\x02\xA56`\x04a0\xD8V[a\t\xE4V[4\x80\x15a\x02\xB6W`\0\x80\xFD[Pa\x01\xD9a\x02\xC56`\x04a1]V[a\x0F\xCBV[4\x80\x15a\x02\xD6W`\0\x80\xFD[Pa\x02?a\x02\xE56`\x04a1\xD0V[a\x10\x02V[a\x02?a\x02\xF86`\x04a1\xEDV[a\x11\xB7V[4\x80\x15a\x03\tW`\0\x80\xFD[Pa\x03\x12a\x11\xD6V[`@Q\x90\x81R` \x01a\x01\xE6V[4\x80\x15a\x03,W`\0\x80\xFD[Pa\x02\x0Fa\x03;6`\x04a2=V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x03gW`\0\x80\xFD[Pa\x02?a\x03v6`\x04a0\xD8V[a\x12\x05V[4\x80\x15a\x03\x87W`\0\x80\xFD[Pa\x02?a\x17\x94V[4\x80\x15a\x03\x9CW`\0\x80\xFD[P`\0Ta\x03\xBD\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE6V[4\x80\x15a\x03\xEEW`\0\x80\xFD[Pa\x03\x12a\x03\xFD6`\x04a2mV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04\x1BW`\0\x80\xFD[Pa\x02?a\x04*6`\x04a4\x01V[a\x17\xA8V[4\x80\x15a\x04;W`\0\x80\xFD[Pa\x03\x12`\x01T\x81V[4\x80\x15a\x04QW`\0\x80\xFD[Pa\x02?a\x04`6`\x04a0\xD8V[a\x19&V[4\x80\x15a\x04qW`\0\x80\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xBDV[4\x80\x15a\x04\xBBW`\0\x80\xFD[Pa\x01\xD9`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x05\x04W`\0\x80\xFD[Pa\x01\xD9`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x05MW`\0\x80\xFD[Pa\x02\x0Fa\x05\\6`\x04a0<V[\x82Q` \x81\x85\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x95\x82\x01\x95\x90\x95 \x91\x90\x94R\x83R`\0\x91\x82R`@\x80\x83 \x90\x93R\x81R T`\xFF\x16\x81V[4\x80\x15a\x05\x9EW`\0\x80\xFD[Pa\x02\x0Fa\x05\xAD6`\x04a5/V[a\x1D\xA2V[4\x80\x15a\x05\xBEW`\0\x80\xFD[Pa\x02\x0Fa\x05\xCD6`\x04a2=V[`\x03` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xF9W`\0\x80\xFD[Pa\x02?a\x06\x086`\x04a1\xD0V[a\x1E+V[`\0\x80\x84Q\x11a\x06dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x82a\x06\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R`d\x01a\x06[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x07:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x03a\x07\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fauthorizer cannot be mainAuthori`D\x82\x01R\x7Fzer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0a\x07\xD7\x84\x84a\x1E\x8FV[\x90P`\0a\x07\xE6\x86\x86\x86a\x1F\x82V[\x90P`\x01\x82\x10a\x07\xFBW`\0\x92PPPa\x08\x16V[`\x02\x81\x10\x15a\x08\x0FW`\0\x92PPPa\x08\x16V[`\x01\x92PPP[\x93\x92PPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x08hWP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x08\x85WP0;\x15[\x90P\x81\x15\x80\x15a\x08\x93WP\x80\x15[\x15a\x08\xCAW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x85U\x83\x15a\t+W\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[a\t4\x88a \x82V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x17\x90U`\x01\x86\x90U\x83\x15a\t\xDAW\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`\0\x84Q\x11a\n5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[\x82a\n\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R`d\x01a\x06[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x0B\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x03a\x0B\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FmainAuthorizer cannot reactivate`D\x82\x01R\x7F the public key hash\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x0CBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fpublic key hash is already react`D\x82\x01R\x7Fivated\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[a\x0CL\x83\x83a\x1E\x8FV[`\x01\x14a\x0C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Frevoke threshold must be one\0\0\0\0`D\x82\x01R`d\x01a\x06[V[`\x02a\x0C\xA8\x85\x85\x85a\x1F\x82V[\x10\x15a\r\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7Fset threshold must be larger tha`D\x82\x01R\x7Fn two\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x0FCW`\0a\r{`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01\x7FREACTIVATE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86a\x0F\xCBV[\x90P`\0a\r\x88\x82a \x93V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15a\x0E\xB5W`@Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x16&\xBA~\x90a\r\xFB\x90\x84\x90\x87\x90`\x04\x01a5tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E<\x91\x90a5\x8DV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16c\x16&\xBA~`\xE0\x1B\x14a\x0E\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid eip1271 signature\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[a\x0F@V[`\0a\x0E\xC1\x82\x85a \xCEV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid ecdsa signature\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[P[PP[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x90\x91\x85\x91\x7F2\x89\x9A\x1E\xA4\xD8\xE4\x91|k=l\x1Ci\xFDLp\x949C\xB0'\xFE\x9D\x83+ R\xE7\xEF\xF8\xD6\x91\x90\xA3PPPPV[``\x83\x83a\x0F\xD8\x84a \xF8V[`@Q` \x01a\x0F\xEA\x93\x92\x91\x90a5\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[a\x10\na!|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x10\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FnewMainAuthorizer address cannot`D\x82\x01R\x7F be zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x82\x16\x03a\x11JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FnewMainAuthorizer address cannot`D\x82\x01R\x7F be the same as the current main`d\x82\x01R\x7FAuthorizer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06[V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x91\x7F;\xB1\x96\x11\xD1\x15f1\xA8\\Y\xDD\xFEvhT\x1A/\0\xE6\xBA+~q\xCB\x0C`\xEC\xE0\xD5\xE5[\x91\xA2PV[a\x11\xBFa\"\nV[a\x11\xC8\x82a#\x0EV[a\x11\xD2\x82\x82a#\x16V[PPV[`\0a\x11\xE0a$TV[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[`\0\x84Q\x11a\x12VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[\x82a\x12\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R`d\x01a\x06[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x13,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\x02\x84`@Qa\x13<\x91\x90a6\x90V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x86\x81R\x90\x83R\x81\x81 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82R\x90\x92R\x90 T`\xFF\x16\x15a\x13\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash is already set\0\0`D\x82\x01R`d\x01a\x06[V[`\0\x83\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x14mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fpublic key hash is already revok`D\x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x16\x94W`\0a\x14\xCC`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86a\x0F\xCBV[\x90P`\0a\x14\xD9\x82a \x93V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15a\x16\x06W`@Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x16&\xBA~\x90a\x15L\x90\x84\x90\x87\x90`\x04\x01a5tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x8D\x91\x90a5\x8DV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16c\x16&\xBA~`\xE0\x1B\x14a\x16\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid eip1271 signature\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[a\x16\x91V[`\0a\x16\x12\x82\x85a \xCEV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x16\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid ecdsa signature\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[P[PP[`\x01`\x02\x85`@Qa\x16\xA6\x91\x90a6\x90V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x87\x81R\x90\x83R\x81\x81 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16\x80\x84R\x91\x90\x94R\x91\x81 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x94\x15\x15\x94\x90\x94\x17\x90\x93U\x91T\x16\x90\x03a\x178W`\x01Ta\x17(\x90Ba6\xDBV[`\0\x84\x81R`\x05` R`@\x90 U[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x85`@Qa\x17^\x91\x90a6\x90V[`@Q\x90\x81\x90\x03\x81 \x90\x7F}a~\xDC\x9D\n\xDE/\xB7hC\xEF_sr\xBD'0\xE9\0\xFA\x12\xE6t\xBE\xCA\xA8\xAD\x01\xEA\xB6\xCB\x90`\0\x90\xA4PPPPV[a\x17\x9Ca!|V[a\x17\xA6`\0a$\xC3V[V[\x82Q\x84Q\x14a\x17\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Finvalid publicKeyHashes length\0\0`D\x82\x01R`d\x01a\x06[V[\x81Q\x84Q\x14a\x18JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Finvalid authorizers length\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[\x80Q\x84Q\x14a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid signatures length\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[`\0[\x84Q\x81\x10\x15a\x19\x1FWa\x19\x17\x85\x82\x81Q\x81\x10a\x18\xBCWa\x18\xBCa6\xEEV[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a\x18\xD6Wa\x18\xD6a6\xEEV[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a\x18\xF0Wa\x18\xF0a6\xEEV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a\x19\nWa\x19\na6\xEEV[` \x02` \x01\x01Qa\x12\x05V[`\x01\x01a\x18\x9EV[PPPPPV[`\0\x84Q\x11a\x19wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[\x82a\x19\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R`d\x01a\x06[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x1AMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0\x83\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x1A\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fpublic key hash is already revok`D\x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x1D\x1AW`\0a\x1BR`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86a\x0F\xCBV[\x90P`\0a\x1B_\x82a \x93V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15a\x1C\x8CW`@Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x16&\xBA~\x90a\x1B\xD2\x90\x84\x90\x87\x90`\x04\x01a5tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x13\x91\x90a5\x8DV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16c\x16&\xBA~`\xE0\x1B\x14a\x1C\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid eip1271 signature\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[a\x1D\x17V[`\0a\x1C\x98\x82\x85a \xCEV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1D\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid ecdsa signature\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[P[PP[`\0\x83\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x90\x91\x85\x91\x7F5P6\xB8\xAD\x96>\x18^\t\xF0t\xE8VU\x96H:\0\x12\xCB\xE6 \xF5\x07\xC0\xF3IP\xA2\xF0\xB3\x91\x90\xA3PPPPV[`\0\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x14\x91\x90a7\x1DV[\x90Pa\x1E!\x84\x84\x83a\x06\rV[\x91PP[\x92\x91PPV[a\x1E3a!|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1E\x83W`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x04\x82\x01R`$\x01a\x06[V[a\x1E\x8C\x81a$\xC3V[PV[`\0\x82\x81R`\x03` \x90\x81R`@\x80\x83 \x83Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x90\x91R\x81 T\x81\x90`\xFF\x16\x15\x15`\x01\x03a\x1E\xDCWa\x1E\xD9`\x01\x82a6\xDBV[\x90P[`\0\x84\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15\x15`\x01\x03a\x1F&Wa\x1F#`\x02\x82a6\xDBV[\x90P[\x80`\x01\x14\x80\x15a\x1FjWP`\0\x84\x81R`\x04` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15\x15`\x01\x14[\x15a\x08\x16Wa\x1Fz`\x01\x82a7:V[\x94\x93PPPPV[`\0\x80`\0\x90P`\x02\x85`@Qa\x1F\x99\x91\x90a6\x90V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x87\x81R\x90\x83R\x81\x81 \x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R\x90\x92R\x90 T`\xFF\x16\x15\x15`\x01\x03a \x17W`\0\x84\x81R`\x05` R`@\x90 TB\x10\x15a \tWa \x02`\x01\x82a6\xDBV[\x90Pa \x17V[a \x14`\x02\x82a6\xDBV[\x90P[`\x02\x85`@Qa '\x91\x90a6\x90V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x87\x81R\x90\x83R\x81\x81 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x82R\x90\x92R\x90 T`\xFF\x16\x15\x15`\x01\x03a\x1FzWa y`\x02\x82a6\xDBV[\x95\x94PPPPPV[a \x8Aa%YV[a\x1E\x8C\x81a%\xC0V[`\0a \x9F\x82Qa%\xC8V[\x82`@Q` \x01a \xB1\x92\x91\x90a7MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0\x80a \xDE\x86\x86a&\x86V[\x92P\x92P\x92Pa \xEE\x82\x82a&\xD3V[P\x90\x94\x93PPPPV[``a\x1E%\x82a!t\x84`\xFF`\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x90\x81\x02\x92\x90\x92\x1C`@g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x90\x81\x02\x91\x90\x91\x1C` c\xFF\xFF\xFF\xFF\x82\x11\x90\x81\x02\x91\x90\x91\x1Ca\xFF\xFF\x81\x11`\x10\x81\x81\x02\x92\x90\x92\x1C\x94\x90\x94\x11`\x02\x90\x94\x02`\x04\x90\x92\x02`\x08\x90\x93\x02\x94\x02\x93\x90\x93\x01\x01\x91\x90\x91\x01\x01\x90V[`\x01\x01a'\xD7V[3a!\xBB\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17\xA6W`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x01a\x06[V[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\"\xD7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\"\xBE\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x17\xA6W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\x8Ca!|V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a#\x9BWP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra#\x98\x91\x81\x01\x90a7\xA8V[`\x01[a#\xE9W`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\x06[V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a$EW`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06[V[a$O\x83\x83a)\xF5V[PPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\xA6W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x17\xA6W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E3a%YV[```\0a%\xD5\x83a*XV[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xF5Wa%\xF5a/\x07V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a&\x1FW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a&)WP\x93\x92PPPV[`\0\x80`\0\x83Q`A\x03a&\xC0W` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa&\xB2\x88\x82\x85\x85a+:V[\x95P\x95P\x95PPPPa&\xCCV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a&\xE7Wa&\xE7a7\xC1V[\x03a&\xF0WPPV[`\x01\x82`\x03\x81\x11\x15a'\x04Wa'\x04a7\xC1V[\x03a';W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a'OWa'Oa7\xC1V[\x03a'\x89W`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06[V[`\x03\x82`\x03\x81\x11\x15a'\x9DWa'\x9Da7\xC1V[\x03a\x11\xD2W`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06[V[``\x82`\0a'\xE7\x84`\x02a7\xF0V[a'\xF2\x90`\x02a6\xDBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\nWa(\na/\x07V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a(4W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a(kWa(ka6\xEEV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a(\xCEWa(\xCEa6\xEEV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0a)\n\x85`\x02a7\xF0V[a)\x15\x90`\x01a6\xDBV[\x90P[`\x01\x81\x11\x15a)\xB2W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\x0F\x16`\x10\x81\x10a)VWa)Va6\xEEV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a)lWa)la6\xEEV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x92\x90\x92\x1C\x91a)\xAB\x81a8\x07V[\x90Pa)\x18V[P\x81\x15a\x1E!W`@Q\x7F\xE2.'\xEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x06[V[a)\xFE\x82a,4V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a*PWa$O\x82\x82a-\x03V[a\x11\xD2a-}V[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a*\xA1Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a*\xCDWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a*\xEBWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a+\x03Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a+\x17Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a+)W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x1E%W`\x01\x01\x92\x91PPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a+uWP`\0\x91P`\x03\x90P\x82a,*V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a+\xC9W=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a, WP`\0\x92P`\x01\x91P\x82\x90Pa,*V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;`\0\x03a,\x9DW`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x06[V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa--\x91\x90a6\x90V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a-hW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a-mV[``\x91P[P\x91P\x91Pa y\x85\x83\x83a-\xB5V[4\x15a\x17\xA6W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a-\xCAWa-\xC5\x82a.DV[a\x08\x16V[\x81Q\x15\x80\x15a-\xEEWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15[\x15a.=W`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`$\x01a\x06[V[P\x80a\x08\x16V[\x80Q\x15a.TW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a.\xA1W\x81\x81\x01Q\x83\x82\x01R` \x01a.\x89V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra.\xC2\x81` \x86\x01` \x86\x01a.\x86V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x08\x16` \x83\x01\x84a.\xAAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/}Wa/}a/\x07V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a/\x96W`\0\x80\xFD[\x815` \x83\x01`\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a/\xB7Wa/\xB7a/\x07V[P`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01a/\xEA\x81a/6V[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15a/\xFFW`\0\x80\xFD[\x82\x82` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1E\x8CW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a0QW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0hW`\0\x80\xFD[a0t\x86\x82\x87\x01a/\x85V[\x93PP` \x84\x015\x91P`@\x84\x015a0\x8C\x81a0\x1AV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a0\xACW`\0\x80\xFD[\x835a0\xB7\x81a0\x1AV[\x92P` \x84\x015a0\xC7\x81a0\x1AV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a0\xEEW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x05W`\0\x80\xFD[a1\x11\x87\x82\x88\x01a/\x85V[\x94PP` \x85\x015\x92P`@\x85\x015a1)\x81a0\x1AV[\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1EW`\0\x80\xFD[a1Q\x87\x82\x88\x01a/\x85V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1rW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x89W`\0\x80\xFD[a1\x95\x86\x82\x87\x01a/\x85V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xB2W`\0\x80\xFD[a1\xBE\x86\x82\x87\x01a/\x85V[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a1\xE2W`\0\x80\xFD[\x815a\x08\x16\x81a0\x1AV[`\0\x80`@\x83\x85\x03\x12\x15a2\0W`\0\x80\xFD[\x825a2\x0B\x81a0\x1AV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2'W`\0\x80\xFD[a23\x85\x82\x86\x01a/\x85V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a2PW`\0\x80\xFD[\x825\x91P` \x83\x015a2b\x81a0\x1AV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a2\x7FW`\0\x80\xFD[P5\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a2\xA0Wa2\xA0a/\x07V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a2\xBBW`\0\x80\xFD[\x815a2\xCEa2\xC9\x82a2\x86V[a/6V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a2\xF0W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a3\rW\x805\x83R` \x92\x83\x01\x92\x01a2\xF5V[P\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a3(W`\0\x80\xFD[\x815a36a2\xC9\x82a2\x86V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a3XW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a3\rW\x805a3p\x81a0\x1AV[\x83R` \x92\x83\x01\x92\x01a3]V[`\0\x82`\x1F\x83\x01\x12a3\x8FW`\0\x80\xFD[\x815a3\x9Da2\xC9\x82a2\x86V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a3\xBFW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a3\rW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\xE3W`\0\x80\xFD[a3\xF2\x88` \x83\x8A\x01\x01a/\x85V[\x84RP` \x92\x83\x01\x92\x01a3\xC4V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a4\x17W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4.W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a4?W`\0\x80\xFD[\x805a4Ma2\xC9\x82a2\x86V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x89\x83\x11\x15a4oW`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15a4\xB1W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\x93W`\0\x80\xFD[a4\xA2\x8C` \x83\x89\x01\x01a/\x85V[\x84RP` \x92\x83\x01\x92\x01a4tV[P\x96PPPP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xD1W`\0\x80\xFD[a4\xDD\x87\x82\x88\x01a2\xAAV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xFAW`\0\x80\xFD[a5\x06\x87\x82\x88\x01a3\x17V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5#W`\0\x80\xFD[a1Q\x87\x82\x88\x01a3~V[`\0\x80`@\x83\x85\x03\x12\x15a5BW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5YW`\0\x80\xFD[a5e\x85\x82\x86\x01a/\x85V[\x95` \x94\x90\x94\x015\x94PPPPV[\x82\x81R`@` \x82\x01R`\0a\x1Fz`@\x83\x01\x84a.\xAAV[`\0` \x82\x84\x03\x12\x15a5\x9FW`\0\x80\xFD[\x81Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x08\x16W`\0\x80\xFD[`\0\x84Qa5\xE1\x81\x84` \x89\x01a.\x86V[\x7Fdomain=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01\x90\x81R\x84Qa6\x1B\x81`\x07\x84\x01` \x89\x01a.\x86V[\x7F;public_key_hash=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x07\x92\x90\x91\x01\x91\x82\x01R\x83Qa6Y\x81`\x18\x84\x01` \x88\x01a.\x86V[\x7F;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x18\x92\x90\x91\x01\x91\x82\x01R`\x19\x01\x95\x94PPPPPV[`\0\x82Qa6\xA2\x81\x84` \x87\x01a.\x86V[\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1E%Wa\x1E%a6\xACV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a7/W`\0\x80\xFD[\x81Qa\x08\x16\x81a0\x1AV[\x81\x81\x03\x81\x81\x11\x15a\x1E%Wa\x1E%a6\xACV[\x7F\x19Ethereum Signed Message:\n\0\0\0\0\0\0\x81R`\0\x83Qa7\x85\x81`\x1A\x85\x01` \x88\x01a.\x86V[\x83Q\x90\x83\x01\x90a7\x9C\x81`\x1A\x84\x01` \x88\x01a.\x86V[\x01`\x1A\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a7\xBAW`\0\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1E%Wa\x1E%a6\xACV[`\0\x81a8\x16Wa8\x16a6\xACV[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x90V\xFE\xA2dipfsX\"\x12 q>\xE1\xC5W\xFE\x9D\x8C[\xE2@\x07\x8D\xB2Z\xAAQ\xB4PX\x8F\x10\\I\xA8\x0Bv \x05\x81\x99\x9CdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static USEROVERRIDABLEDKIMREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x8BW`\x005`\xE0\x1C\x80c}F6H\x11a\0\xD6W\x80c\xAD<\xB1\xCC\x11a\0\x7FW\x80c\xE7\xA7\x97z\x11a\0YW\x80c\xE7\xA7\x97z\x14a\x05\x92W\x80c\xF0\xBF\xB1\x97\x14a\x05\xB2W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xEDW`\0\x80\xFD[\x80c\xAD<\xB1\xCC\x14a\x04\xAFW\x80c\xD5\x07\xC3 \x14a\x04\xF8W\x80c\xE3\x08\xDE\x0C\x14a\x05AW`\0\x80\xFD[\x80c\x81.\x12\xCE\x11a\0\xB0W\x80c\x81.\x12\xCE\x14a\x04/W\x80c\x82\xBF\xF8\xCD\x14a\x04EW\x80c\x8D\xA5\xCB[\x14a\x04eW`\0\x80\xFD[\x80c}F6H\x14a\x03\x90W\x80c\x7F\x8E)\xBA\x14a\x03\xE2W\x80c\x7F\xF1\x03\xDA\x14a\x04\x0FW`\0\x80\xFD[\x80cL\x93\x06\x07\x11a\x018W\x80cWI\0\xDD\x11a\x01\x12W\x80cWI\0\xDD\x14a\x03 W\x80caJD\x85\x14a\x03[W\x80cqP\x18\xA6\x14a\x03{W`\0\x80\xFD[\x80cL\x93\x06\x07\x14a\x02\xCAW\x80cO\x1E\xF2\x86\x14a\x02\xEAW\x80cR\xD1\x90-\x14a\x02\xFDW`\0\x80\xFD[\x80c\"Z\x08\xD4\x11a\x01iW\x80c\"Z\x08\xD4\x14a\x02AW\x80c2\xE1\xE1\x94\x14a\x02\x8AW\x80cK\xCB\xBE\x96\x14a\x02\xAAW`\0\x80\xFD[\x80c\x07\xF1\xEA\xF5\x14a\x01\x90W\x80c\x0BU\xB3|\x14a\x01\xEFW\x80c\x17\x94\xBB<\x14a\x02\x1FW[`\0\x80\xFD[4\x80\x15a\x01\x9CW`\0\x80\xFD[Pa\x01\xD9`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\x01\xE6\x91\x90a.\xF4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xFBW`\0\x80\xFD[Pa\x02\x0Fa\x02\n6`\x04a0<V[a\x06\rV[`@Q\x90\x15\x15\x81R` \x01a\x01\xE6V[4\x80\x15a\x02+W`\0\x80\xFD[Pa\x02?a\x02:6`\x04a0\x97V[a\x08\x1DV[\0[4\x80\x15a\x02MW`\0\x80\xFD[Pa\x01\xD9`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01\x7FREACTIVATE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x02\x96W`\0\x80\xFD[Pa\x02?a\x02\xA56`\x04a0\xD8V[a\t\xE4V[4\x80\x15a\x02\xB6W`\0\x80\xFD[Pa\x01\xD9a\x02\xC56`\x04a1]V[a\x0F\xCBV[4\x80\x15a\x02\xD6W`\0\x80\xFD[Pa\x02?a\x02\xE56`\x04a1\xD0V[a\x10\x02V[a\x02?a\x02\xF86`\x04a1\xEDV[a\x11\xB7V[4\x80\x15a\x03\tW`\0\x80\xFD[Pa\x03\x12a\x11\xD6V[`@Q\x90\x81R` \x01a\x01\xE6V[4\x80\x15a\x03,W`\0\x80\xFD[Pa\x02\x0Fa\x03;6`\x04a2=V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x03gW`\0\x80\xFD[Pa\x02?a\x03v6`\x04a0\xD8V[a\x12\x05V[4\x80\x15a\x03\x87W`\0\x80\xFD[Pa\x02?a\x17\x94V[4\x80\x15a\x03\x9CW`\0\x80\xFD[P`\0Ta\x03\xBD\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xE6V[4\x80\x15a\x03\xEEW`\0\x80\xFD[Pa\x03\x12a\x03\xFD6`\x04a2mV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04\x1BW`\0\x80\xFD[Pa\x02?a\x04*6`\x04a4\x01V[a\x17\xA8V[4\x80\x15a\x04;W`\0\x80\xFD[Pa\x03\x12`\x01T\x81V[4\x80\x15a\x04QW`\0\x80\xFD[Pa\x02?a\x04`6`\x04a0\xD8V[a\x19&V[4\x80\x15a\x04qW`\0\x80\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xBDV[4\x80\x15a\x04\xBBW`\0\x80\xFD[Pa\x01\xD9`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x05\x04W`\0\x80\xFD[Pa\x01\xD9`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x05MW`\0\x80\xFD[Pa\x02\x0Fa\x05\\6`\x04a0<V[\x82Q` \x81\x85\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x95\x82\x01\x95\x90\x95 \x91\x90\x94R\x83R`\0\x91\x82R`@\x80\x83 \x90\x93R\x81R T`\xFF\x16\x81V[4\x80\x15a\x05\x9EW`\0\x80\xFD[Pa\x02\x0Fa\x05\xAD6`\x04a5/V[a\x1D\xA2V[4\x80\x15a\x05\xBEW`\0\x80\xFD[Pa\x02\x0Fa\x05\xCD6`\x04a2=V[`\x03` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xF9W`\0\x80\xFD[Pa\x02?a\x06\x086`\x04a1\xD0V[a\x1E+V[`\0\x80\x84Q\x11a\x06dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x82a\x06\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R`d\x01a\x06[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x07:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x03a\x07\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fauthorizer cannot be mainAuthori`D\x82\x01R\x7Fzer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0a\x07\xD7\x84\x84a\x1E\x8FV[\x90P`\0a\x07\xE6\x86\x86\x86a\x1F\x82V[\x90P`\x01\x82\x10a\x07\xFBW`\0\x92PPPa\x08\x16V[`\x02\x81\x10\x15a\x08\x0FW`\0\x92PPPa\x08\x16V[`\x01\x92PPP[\x93\x92PPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x81\x15\x80\x15a\x08hWP\x82[\x90P`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x08\x85WP0;\x15[\x90P\x81\x15\x80\x15a\x08\x93WP\x80\x15[\x15a\x08\xCAW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16`\x01\x17\x85U\x83\x15a\t+W\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x17\x85U[a\t4\x88a \x82V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x17\x90U`\x01\x86\x90U\x83\x15a\t\xDAW\x84T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[`\0\x84Q\x11a\n5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[\x82a\n\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R`d\x01a\x06[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x0B\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x03a\x0B\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FmainAuthorizer cannot reactivate`D\x82\x01R\x7F the public key hash\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x0CBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fpublic key hash is already react`D\x82\x01R\x7Fivated\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[a\x0CL\x83\x83a\x1E\x8FV[`\x01\x14a\x0C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Frevoke threshold must be one\0\0\0\0`D\x82\x01R`d\x01a\x06[V[`\x02a\x0C\xA8\x85\x85\x85a\x1F\x82V[\x10\x15a\r\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7Fset threshold must be larger tha`D\x82\x01R\x7Fn two\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x0FCW`\0a\r{`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01\x7FREACTIVATE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86a\x0F\xCBV[\x90P`\0a\r\x88\x82a \x93V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15a\x0E\xB5W`@Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x16&\xBA~\x90a\r\xFB\x90\x84\x90\x87\x90`\x04\x01a5tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E<\x91\x90a5\x8DV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16c\x16&\xBA~`\xE0\x1B\x14a\x0E\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid eip1271 signature\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[a\x0F@V[`\0a\x0E\xC1\x82\x85a \xCEV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid ecdsa signature\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[P[PP[`\0\x83\x81R`\x04` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x90\x91\x85\x91\x7F2\x89\x9A\x1E\xA4\xD8\xE4\x91|k=l\x1Ci\xFDLp\x949C\xB0'\xFE\x9D\x83+ R\xE7\xEF\xF8\xD6\x91\x90\xA3PPPPV[``\x83\x83a\x0F\xD8\x84a \xF8V[`@Q` \x01a\x0F\xEA\x93\x92\x91\x90a5\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[a\x10\na!|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x10\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FnewMainAuthorizer address cannot`D\x82\x01R\x7F be zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x82\x16\x03a\x11JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FnewMainAuthorizer address cannot`D\x82\x01R\x7F be the same as the current main`d\x82\x01R\x7FAuthorizer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06[V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x91\x7F;\xB1\x96\x11\xD1\x15f1\xA8\\Y\xDD\xFEvhT\x1A/\0\xE6\xBA+~q\xCB\x0C`\xEC\xE0\xD5\xE5[\x91\xA2PV[a\x11\xBFa\"\nV[a\x11\xC8\x82a#\x0EV[a\x11\xD2\x82\x82a#\x16V[PPV[`\0a\x11\xE0a$TV[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[`\0\x84Q\x11a\x12VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[\x82a\x12\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R`d\x01a\x06[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x13,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\x02\x84`@Qa\x13<\x91\x90a6\x90V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x86\x81R\x90\x83R\x81\x81 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82R\x90\x92R\x90 T`\xFF\x16\x15a\x13\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash is already set\0\0`D\x82\x01R`d\x01a\x06[V[`\0\x83\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x14mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fpublic key hash is already revok`D\x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x16\x94W`\0a\x14\xCC`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86a\x0F\xCBV[\x90P`\0a\x14\xD9\x82a \x93V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15a\x16\x06W`@Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x16&\xBA~\x90a\x15L\x90\x84\x90\x87\x90`\x04\x01a5tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x8D\x91\x90a5\x8DV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16c\x16&\xBA~`\xE0\x1B\x14a\x16\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid eip1271 signature\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[a\x16\x91V[`\0a\x16\x12\x82\x85a \xCEV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x16\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid ecdsa signature\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[P[PP[`\x01`\x02\x85`@Qa\x16\xA6\x91\x90a6\x90V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x87\x81R\x90\x83R\x81\x81 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16\x80\x84R\x91\x90\x94R\x91\x81 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x94\x15\x15\x94\x90\x94\x17\x90\x93U\x91T\x16\x90\x03a\x178W`\x01Ta\x17(\x90Ba6\xDBV[`\0\x84\x81R`\x05` R`@\x90 U[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x85`@Qa\x17^\x91\x90a6\x90V[`@Q\x90\x81\x90\x03\x81 \x90\x7F}a~\xDC\x9D\n\xDE/\xB7hC\xEF_sr\xBD'0\xE9\0\xFA\x12\xE6t\xBE\xCA\xA8\xAD\x01\xEA\xB6\xCB\x90`\0\x90\xA4PPPPV[a\x17\x9Ca!|V[a\x17\xA6`\0a$\xC3V[V[\x82Q\x84Q\x14a\x17\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Finvalid publicKeyHashes length\0\0`D\x82\x01R`d\x01a\x06[V[\x81Q\x84Q\x14a\x18JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Finvalid authorizers length\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[\x80Q\x84Q\x14a\x18\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid signatures length\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[`\0[\x84Q\x81\x10\x15a\x19\x1FWa\x19\x17\x85\x82\x81Q\x81\x10a\x18\xBCWa\x18\xBCa6\xEEV[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a\x18\xD6Wa\x18\xD6a6\xEEV[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a\x18\xF0Wa\x18\xF0a6\xEEV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a\x19\nWa\x19\na6\xEEV[` \x02` \x01\x01Qa\x12\x05V[`\x01\x01a\x18\x9EV[PPPPPV[`\0\x84Q\x11a\x19wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[\x82a\x19\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R`d\x01a\x06[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x1AMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[`\0\x83\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x1A\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fpublic key hash is already revok`D\x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06[V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x1D\x1AW`\0a\x1BR`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x86\x86a\x0F\xCBV[\x90P`\0a\x1B_\x82a \x93V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15a\x1C\x8CW`@Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x16&\xBA~\x90a\x1B\xD2\x90\x84\x90\x87\x90`\x04\x01a5tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x13\x91\x90a5\x8DV[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16c\x16&\xBA~`\xE0\x1B\x14a\x1C\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid eip1271 signature\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[a\x1D\x17V[`\0a\x1C\x98\x82\x85a \xCEV[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1D\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid ecdsa signature\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06[V[P[PP[`\0\x83\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x80\x85R\x92R\x80\x83 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x90\x91\x85\x91\x7F5P6\xB8\xAD\x96>\x18^\t\xF0t\xE8VU\x96H:\0\x12\xCB\xE6 \xF5\x07\xC0\xF3IP\xA2\xF0\xB3\x91\x90\xA3PPPPV[`\0\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x14\x91\x90a7\x1DV[\x90Pa\x1E!\x84\x84\x83a\x06\rV[\x91PP[\x92\x91PPV[a\x1E3a!|V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1E\x83W`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x04\x82\x01R`$\x01a\x06[V[a\x1E\x8C\x81a$\xC3V[PV[`\0\x82\x81R`\x03` \x90\x81R`@\x80\x83 \x83Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x90\x91R\x81 T\x81\x90`\xFF\x16\x15\x15`\x01\x03a\x1E\xDCWa\x1E\xD9`\x01\x82a6\xDBV[\x90P[`\0\x84\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15\x15`\x01\x03a\x1F&Wa\x1F#`\x02\x82a6\xDBV[\x90P[\x80`\x01\x14\x80\x15a\x1FjWP`\0\x84\x81R`\x04` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15\x15`\x01\x14[\x15a\x08\x16Wa\x1Fz`\x01\x82a7:V[\x94\x93PPPPV[`\0\x80`\0\x90P`\x02\x85`@Qa\x1F\x99\x91\x90a6\x90V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x87\x81R\x90\x83R\x81\x81 \x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R\x90\x92R\x90 T`\xFF\x16\x15\x15`\x01\x03a \x17W`\0\x84\x81R`\x05` R`@\x90 TB\x10\x15a \tWa \x02`\x01\x82a6\xDBV[\x90Pa \x17V[a \x14`\x02\x82a6\xDBV[\x90P[`\x02\x85`@Qa '\x91\x90a6\x90V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 `\0\x87\x81R\x90\x83R\x81\x81 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x82R\x90\x92R\x90 T`\xFF\x16\x15\x15`\x01\x03a\x1FzWa y`\x02\x82a6\xDBV[\x95\x94PPPPPV[a \x8Aa%YV[a\x1E\x8C\x81a%\xC0V[`\0a \x9F\x82Qa%\xC8V[\x82`@Q` \x01a \xB1\x92\x91\x90a7MV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0\x80a \xDE\x86\x86a&\x86V[\x92P\x92P\x92Pa \xEE\x82\x82a&\xD3V[P\x90\x94\x93PPPPV[``a\x1E%\x82a!t\x84`\xFF`\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x90\x81\x02\x92\x90\x92\x1C`@g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x90\x81\x02\x91\x90\x91\x1C` c\xFF\xFF\xFF\xFF\x82\x11\x90\x81\x02\x91\x90\x91\x1Ca\xFF\xFF\x81\x11`\x10\x81\x81\x02\x92\x90\x92\x1C\x94\x90\x94\x11`\x02\x90\x94\x02`\x04\x90\x92\x02`\x08\x90\x93\x02\x94\x02\x93\x90\x93\x01\x01\x91\x90\x91\x01\x01\x90V[`\x01\x01a'\xD7V[3a!\xBB\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17\xA6W`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x01a\x06[V[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\"\xD7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\"\xBE\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCTs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x17\xA6W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\x8Ca!|V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a#\x9BWP`@\x80Q`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01\x90\x92Ra#\x98\x91\x81\x01\x90a7\xA8V[`\x01[a#\xE9W`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\x06[V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a$EW`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06[V[a$O\x83\x83a)\xF5V[PPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\xA6W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x17\xA6W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E3a%YV[```\0a%\xD5\x83a*XV[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xF5Wa%\xF5a/\x07V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a&\x1FW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a&)WP\x93\x92PPPV[`\0\x80`\0\x83Q`A\x03a&\xC0W` \x84\x01Q`@\x85\x01Q``\x86\x01Q`\0\x1Aa&\xB2\x88\x82\x85\x85a+:V[\x95P\x95P\x95PPPPa&\xCCV[PP\x81Q`\0\x91P`\x02\x90[\x92P\x92P\x92V[`\0\x82`\x03\x81\x11\x15a&\xE7Wa&\xE7a7\xC1V[\x03a&\xF0WPPV[`\x01\x82`\x03\x81\x11\x15a'\x04Wa'\x04a7\xC1V[\x03a';W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a'OWa'Oa7\xC1V[\x03a'\x89W`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06[V[`\x03\x82`\x03\x81\x11\x15a'\x9DWa'\x9Da7\xC1V[\x03a\x11\xD2W`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06[V[``\x82`\0a'\xE7\x84`\x02a7\xF0V[a'\xF2\x90`\x02a6\xDBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\nWa(\na/\x07V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a(4W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a(kWa(ka6\xEEV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP\x7Fx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a(\xCEWa(\xCEa6\xEEV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\0a)\n\x85`\x02a7\xF0V[a)\x15\x90`\x01a6\xDBV[\x90P[`\x01\x81\x11\x15a)\xB2W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\x0F\x16`\x10\x81\x10a)VWa)Va6\xEEV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a)lWa)la6\xEEV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x92\x90\x92\x1C\x91a)\xAB\x81a8\x07V[\x90Pa)\x18V[P\x81\x15a\x1E!W`@Q\x7F\xE2.'\xEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x06[V[a)\xFE\x82a,4V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2\x80Q\x15a*PWa$O\x82\x82a-\x03V[a\x11\xD2a-}V[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a*\xA1Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a*\xCDWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a*\xEBWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a+\x03Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a+\x17Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a+)W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x1E%W`\x01\x01\x92\x91PPV[`\0\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a+uWP`\0\x91P`\x03\x90P\x82a,*V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a+\xC9W=`\0\x80>=`\0\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a, WP`\0\x92P`\x01\x91P\x82\x90Pa,*V[\x92P`\0\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;`\0\x03a,\x9DW`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x06[V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa--\x91\x90a6\x90V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a-hW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a-mV[``\x91P[P\x91P\x91Pa y\x85\x83\x83a-\xB5V[4\x15a\x17\xA6W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a-\xCAWa-\xC5\x82a.DV[a\x08\x16V[\x81Q\x15\x80\x15a-\xEEWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;\x15[\x15a.=W`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`$\x01a\x06[V[P\x80a\x08\x16V[\x80Q\x15a.TW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a.\xA1W\x81\x81\x01Q\x83\x82\x01R` \x01a.\x89V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra.\xC2\x81` \x86\x01` \x86\x01a.\x86V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x08\x16` \x83\x01\x84a.\xAAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/}Wa/}a/\x07V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a/\x96W`\0\x80\xFD[\x815` \x83\x01`\0\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a/\xB7Wa/\xB7a/\x07V[P`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01a/\xEA\x81a/6V[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15a/\xFFW`\0\x80\xFD[\x82\x82` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1E\x8CW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a0QW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0hW`\0\x80\xFD[a0t\x86\x82\x87\x01a/\x85V[\x93PP` \x84\x015\x91P`@\x84\x015a0\x8C\x81a0\x1AV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a0\xACW`\0\x80\xFD[\x835a0\xB7\x81a0\x1AV[\x92P` \x84\x015a0\xC7\x81a0\x1AV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a0\xEEW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x05W`\0\x80\xFD[a1\x11\x87\x82\x88\x01a/\x85V[\x94PP` \x85\x015\x92P`@\x85\x015a1)\x81a0\x1AV[\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1EW`\0\x80\xFD[a1Q\x87\x82\x88\x01a/\x85V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1rW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x89W`\0\x80\xFD[a1\x95\x86\x82\x87\x01a/\x85V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xB2W`\0\x80\xFD[a1\xBE\x86\x82\x87\x01a/\x85V[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a1\xE2W`\0\x80\xFD[\x815a\x08\x16\x81a0\x1AV[`\0\x80`@\x83\x85\x03\x12\x15a2\0W`\0\x80\xFD[\x825a2\x0B\x81a0\x1AV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2'W`\0\x80\xFD[a23\x85\x82\x86\x01a/\x85V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a2PW`\0\x80\xFD[\x825\x91P` \x83\x015a2b\x81a0\x1AV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a2\x7FW`\0\x80\xFD[P5\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a2\xA0Wa2\xA0a/\x07V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a2\xBBW`\0\x80\xFD[\x815a2\xCEa2\xC9\x82a2\x86V[a/6V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a2\xF0W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a3\rW\x805\x83R` \x92\x83\x01\x92\x01a2\xF5V[P\x95\x94PPPPPV[`\0\x82`\x1F\x83\x01\x12a3(W`\0\x80\xFD[\x815a36a2\xC9\x82a2\x86V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a3XW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a3\rW\x805a3p\x81a0\x1AV[\x83R` \x92\x83\x01\x92\x01a3]V[`\0\x82`\x1F\x83\x01\x12a3\x8FW`\0\x80\xFD[\x815a3\x9Da2\xC9\x82a2\x86V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a3\xBFW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a3\rW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\xE3W`\0\x80\xFD[a3\xF2\x88` \x83\x8A\x01\x01a/\x85V[\x84RP` \x92\x83\x01\x92\x01a3\xC4V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a4\x17W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4.W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a4?W`\0\x80\xFD[\x805a4Ma2\xC9\x82a2\x86V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x89\x83\x11\x15a4oW`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15a4\xB1W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\x93W`\0\x80\xFD[a4\xA2\x8C` \x83\x89\x01\x01a/\x85V[\x84RP` \x92\x83\x01\x92\x01a4tV[P\x96PPPP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xD1W`\0\x80\xFD[a4\xDD\x87\x82\x88\x01a2\xAAV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xFAW`\0\x80\xFD[a5\x06\x87\x82\x88\x01a3\x17V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5#W`\0\x80\xFD[a1Q\x87\x82\x88\x01a3~V[`\0\x80`@\x83\x85\x03\x12\x15a5BW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5YW`\0\x80\xFD[a5e\x85\x82\x86\x01a/\x85V[\x95` \x94\x90\x94\x015\x94PPPPV[\x82\x81R`@` \x82\x01R`\0a\x1Fz`@\x83\x01\x84a.\xAAV[`\0` \x82\x84\x03\x12\x15a5\x9FW`\0\x80\xFD[\x81Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x08\x16W`\0\x80\xFD[`\0\x84Qa5\xE1\x81\x84` \x89\x01a.\x86V[\x7Fdomain=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01\x90\x81R\x84Qa6\x1B\x81`\x07\x84\x01` \x89\x01a.\x86V[\x7F;public_key_hash=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x07\x92\x90\x91\x01\x91\x82\x01R\x83Qa6Y\x81`\x18\x84\x01` \x88\x01a.\x86V[\x7F;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x18\x92\x90\x91\x01\x91\x82\x01R`\x19\x01\x95\x94PPPPPV[`\0\x82Qa6\xA2\x81\x84` \x87\x01a.\x86V[\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1E%Wa\x1E%a6\xACV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a7/W`\0\x80\xFD[\x81Qa\x08\x16\x81a0\x1AV[\x81\x81\x03\x81\x81\x11\x15a\x1E%Wa\x1E%a6\xACV[\x7F\x19Ethereum Signed Message:\n\0\0\0\0\0\0\x81R`\0\x83Qa7\x85\x81`\x1A\x85\x01` \x88\x01a.\x86V[\x83Q\x90\x83\x01\x90a7\x9C\x81`\x1A\x84\x01` \x88\x01a.\x86V[\x01`\x1A\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a7\xBAW`\0\x80\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1E%Wa\x1E%a6\xACV[`\0\x81a8\x16Wa8\x16a6\xACV[P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x90V\xFE\xA2dipfsX\"\x12 q>\xE1\xC5W\xFE\x9D\x8C[\xE2@\x07\x8D\xB2Z\xAAQ\xB4PX\x8F\x10\\I\xA8\x0Bv \x05\x81\x99\x9CdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static USEROVERRIDABLEDKIMREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UserOverridableDKIMRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UserOverridableDKIMRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UserOverridableDKIMRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UserOverridableDKIMRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UserOverridableDKIMRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UserOverridableDKIMRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UserOverridableDKIMRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    USEROVERRIDABLEDKIMREGISTRY_ABI.clone(),
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
                USEROVERRIDABLEDKIMREGISTRY_ABI.clone(),
                USEROVERRIDABLEDKIMREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `REACTIVATE_PREFIX` (0x225a08d4) function
        pub fn reactivate_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([34, 90, 8, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REVOKE_PREFIX` (0xd507c320) function
        pub fn revoke_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([213, 7, 195, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SET_PREFIX` (0x07f1eaf5) function
        pub fn set_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([7, 241, 234, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UPGRADE_INTERFACE_VERSION` (0xad3cb1cc) function
        pub fn upgrade_interface_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([173, 60, 177, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeMainAuthorizer` (0x4c930607) function
        pub fn change_main_authorizer(
            &self,
            new_main_authorizer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 147, 6, 7], new_main_authorizer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeSignedMsg` (0x4bcbbe96) function
        pub fn compute_signed_msg(
            &self,
            prefix: ::std::string::String,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([75, 203, 190, 150], (prefix, domain_name, public_key_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dkimPublicKeyHashes` (0xe308de0c) function
        pub fn dkim_public_key_hashes(
            &self,
            p0: ::std::string::String,
            p1: [u8; 32],
            p2: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([227, 8, 222, 12], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enabledTimeOfDKIMPublicKeyHash` (0x7f8e29ba) function
        pub fn enabled_time_of_dkim_public_key_hash(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([127, 142, 41, 186], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1794bb3c) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            main_authorizer: ::ethers::core::types::Address,
            set_timestamp_delay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [23, 148, 187, 60],
                    (initial_owner, main_authorizer, set_timestamp_delay),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDKIMPublicKeyHashValid` (0x0b55b37c) function
        pub fn is_dkim_public_key_hash_valid_with_domain_name_and_public_key_hash(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            authorizer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [11, 85, 179, 124],
                    (domain_name, public_key_hash, authorizer),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDKIMPublicKeyHashValid` (0xe7a7977a) function
        pub fn is_dkim_public_key_hash_valid(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([231, 167, 151, 122], (domain_name, public_key_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mainAuthorizer` (0x7d463648) function
        pub fn main_authorizer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 70, 54, 72], ())
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
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactivateDKIMPublicKeyHash` (0x32e1e194) function
        pub fn reactivate_dkim_public_key_hash(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            authorizer: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [50, 225, 225, 148],
                    (domain_name, public_key_hash, authorizer, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactivatedDKIMPublicKeyHashes` (0x574900dd) function
        pub fn reactivated_dkim_public_key_hashes(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([87, 73, 0, 221], (p0, p1))
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
        ///Calls the contract's `revokeDKIMPublicKeyHash` (0x82bff8cd) function
        pub fn revoke_dkim_public_key_hash(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            authorizer: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [130, 191, 248, 205],
                    (domain_name, public_key_hash, authorizer, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokedDKIMPublicKeyHashes` (0xf0bfb197) function
        pub fn revoked_dkim_public_key_hashes(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([240, 191, 177, 151], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDKIMPublicKeyHash` (0x614a4485) function
        pub fn set_dkim_public_key_hash(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            authorizer: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [97, 74, 68, 133],
                    (domain_name, public_key_hash, authorizer, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDKIMPublicKeyHashes` (0x7ff103da) function
        pub fn set_dkim_public_key_hashes(
            &self,
            domain_names: ::std::vec::Vec<::std::string::String>,
            public_key_hashes: ::std::vec::Vec<[u8; 32]>,
            authorizers: ::std::vec::Vec<::ethers::core::types::Address>,
            signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [127, 241, 3, 218],
                    (domain_names, public_key_hashes, authorizers, signatures),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTimestampDelay` (0x812e12ce) function
        pub fn set_timestamp_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([129, 46, 18, 206], ())
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
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DKIMPublicKeyHashReactivated` event
        pub fn dkim_public_key_hash_reactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DkimpublicKeyHashReactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DKIMPublicKeyHashRegistered` event
        pub fn dkim_public_key_hash_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DkimpublicKeyHashRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DKIMPublicKeyHashRevoked` event
        pub fn dkim_public_key_hash_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DkimpublicKeyHashRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MainAuthorizerChanged` event
        pub fn main_authorizer_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MainAuthorizerChangedFilter,
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
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UserOverridableDKIMRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UserOverridableDKIMRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `ECDSAInvalidSignature` with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`
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
    #[etherror(name = "ECDSAInvalidSignature", abi = "ECDSAInvalidSignature()")]
    pub struct ECDSAInvalidSignature;
    ///Custom Error type `ECDSAInvalidSignatureLength` with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`
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
    #[etherror(
        name = "ECDSAInvalidSignatureLength",
        abi = "ECDSAInvalidSignatureLength(uint256)"
    )]
    pub struct ECDSAInvalidSignatureLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ECDSAInvalidSignatureS` with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`
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
    #[etherror(name = "ECDSAInvalidSignatureS", abi = "ECDSAInvalidSignatureS(bytes32)")]
    pub struct ECDSAInvalidSignatureS {
        pub s: [u8; 32],
    }
    ///Custom Error type `ERC1967InvalidImplementation` with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`
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
    #[etherror(
        name = "ERC1967InvalidImplementation",
        abi = "ERC1967InvalidImplementation(address)"
    )]
    pub struct ERC1967InvalidImplementation {
        pub implementation: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967NonPayable` with signature `ERC1967NonPayable()` and selector `0xb398979f`
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
    #[etherror(name = "ERC1967NonPayable", abi = "ERC1967NonPayable()")]
    pub struct ERC1967NonPayable;
    ///Custom Error type `FailedCall` with signature `FailedCall()` and selector `0xd6bda275`
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
    #[etherror(name = "FailedCall", abi = "FailedCall()")]
    pub struct FailedCall;
    ///Custom Error type `InvalidInitialization` with signature `InvalidInitialization()` and selector `0xf92ee8a9`
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
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization()")]
    pub struct InvalidInitialization;
    ///Custom Error type `NotInitializing` with signature `NotInitializing()` and selector `0xd7e6bcf8`
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
    #[etherror(name = "NotInitializing", abi = "NotInitializing()")]
    pub struct NotInitializing;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
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
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
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
    #[etherror(
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `StringsInsufficientHexLength` with signature `StringsInsufficientHexLength(uint256,uint256)` and selector `0xe22e27eb`
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
    #[etherror(
        name = "StringsInsufficientHexLength",
        abi = "StringsInsufficientHexLength(uint256,uint256)"
    )]
    pub struct StringsInsufficientHexLength {
        pub value: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `UUPSUnauthorizedCallContext` with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`
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
    #[etherror(
        name = "UUPSUnauthorizedCallContext",
        abi = "UUPSUnauthorizedCallContext()"
    )]
    pub struct UUPSUnauthorizedCallContext;
    ///Custom Error type `UUPSUnsupportedProxiableUUID` with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`
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
    #[etherror(
        name = "UUPSUnsupportedProxiableUUID",
        abi = "UUPSUnsupportedProxiableUUID(bytes32)"
    )]
    pub struct UUPSUnsupportedProxiableUUID {
        pub slot: [u8; 32],
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UserOverridableDKIMRegistryErrors {
        AddressEmptyCode(AddressEmptyCode),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedCall(FailedCall),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        StringsInsufficientHexLength(StringsInsufficientHexLength),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for UserOverridableDKIMRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignature(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignatureLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignatureLength(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignatureS as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignatureS(decoded));
            }
            if let Ok(decoded) = <ERC1967InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <ERC1967NonPayable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967NonPayable(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedCall(decoded));
            }
            if let Ok(decoded) = <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotInitializing(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <StringsInsufficientHexLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StringsInsufficientHexLength(decoded));
            }
            if let Ok(decoded) = <UUPSUnauthorizedCallContext as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UUPSUnauthorizedCallContext(decoded));
            }
            if let Ok(decoded) = <UUPSUnsupportedProxiableUUID as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UUPSUnsupportedProxiableUUID(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UserOverridableDKIMRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967NonPayable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringsInsufficientHexLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for UserOverridableDKIMRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967NonPayable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedCall as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StringsInsufficientHexLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnauthorizedCallContext as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnsupportedProxiableUUID as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for UserOverridableDKIMRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StringsInsufficientHexLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for UserOverridableDKIMRegistryErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for UserOverridableDKIMRegistryErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for UserOverridableDKIMRegistryErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for UserOverridableDKIMRegistryErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<StringsInsufficientHexLength>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: StringsInsufficientHexLength) -> Self {
            Self::StringsInsufficientHexLength(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
            Self::UUPSUnsupportedProxiableUUID(value)
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
    #[ethevent(
        name = "DKIMPublicKeyHashReactivated",
        abi = "DKIMPublicKeyHashReactivated(bytes32,address)"
    )]
    pub struct DkimpublicKeyHashReactivatedFilter {
        #[ethevent(indexed)]
        pub public_key_hash: [u8; 32],
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
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
        name = "DKIMPublicKeyHashRegistered",
        abi = "DKIMPublicKeyHashRegistered(string,bytes32,address)"
    )]
    pub struct DkimpublicKeyHashRegisteredFilter {
        #[ethevent(indexed)]
        pub domain_name: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub public_key_hash: [u8; 32],
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
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
        name = "DKIMPublicKeyHashRevoked",
        abi = "DKIMPublicKeyHashRevoked(bytes32,address)"
    )]
    pub struct DkimpublicKeyHashRevokedFilter {
        #[ethevent(indexed)]
        pub public_key_hash: [u8; 32],
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint64)")]
    pub struct InitializedFilter {
        pub version: u64,
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
    #[ethevent(name = "MainAuthorizerChanged", abi = "MainAuthorizerChanged(address)")]
    pub struct MainAuthorizerChangedFilter {
        #[ethevent(indexed)]
        pub new_main_authorizer: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UserOverridableDKIMRegistryEvents {
        DkimpublicKeyHashReactivatedFilter(DkimpublicKeyHashReactivatedFilter),
        DkimpublicKeyHashRegisteredFilter(DkimpublicKeyHashRegisteredFilter),
        DkimpublicKeyHashRevokedFilter(DkimpublicKeyHashRevokedFilter),
        InitializedFilter(InitializedFilter),
        MainAuthorizerChangedFilter(MainAuthorizerChangedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for UserOverridableDKIMRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DkimpublicKeyHashReactivatedFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::DkimpublicKeyHashReactivatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DkimpublicKeyHashRegisteredFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::DkimpublicKeyHashRegisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DkimpublicKeyHashRevokedFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::DkimpublicKeyHashRevokedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(UserOverridableDKIMRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MainAuthorizerChangedFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::MainAuthorizerChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::OwnershipTransferredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(UserOverridableDKIMRegistryEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UserOverridableDKIMRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DkimpublicKeyHashReactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DkimpublicKeyHashRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DkimpublicKeyHashRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MainAuthorizerChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DkimpublicKeyHashReactivatedFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: DkimpublicKeyHashReactivatedFilter) -> Self {
            Self::DkimpublicKeyHashReactivatedFilter(value)
        }
    }
    impl ::core::convert::From<DkimpublicKeyHashRegisteredFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: DkimpublicKeyHashRegisteredFilter) -> Self {
            Self::DkimpublicKeyHashRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<DkimpublicKeyHashRevokedFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: DkimpublicKeyHashRevokedFilter) -> Self {
            Self::DkimpublicKeyHashRevokedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for UserOverridableDKIMRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MainAuthorizerChangedFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: MainAuthorizerChangedFilter) -> Self {
            Self::MainAuthorizerChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for UserOverridableDKIMRegistryEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `REACTIVATE_PREFIX` function with signature `REACTIVATE_PREFIX()` and selector `0x225a08d4`
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
    #[ethcall(name = "REACTIVATE_PREFIX", abi = "REACTIVATE_PREFIX()")]
    pub struct ReactivatePrefixCall;
    ///Container type for all input parameters for the `REVOKE_PREFIX` function with signature `REVOKE_PREFIX()` and selector `0xd507c320`
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
    #[ethcall(name = "REVOKE_PREFIX", abi = "REVOKE_PREFIX()")]
    pub struct RevokePrefixCall;
    ///Container type for all input parameters for the `SET_PREFIX` function with signature `SET_PREFIX()` and selector `0x07f1eaf5`
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
    #[ethcall(name = "SET_PREFIX", abi = "SET_PREFIX()")]
    pub struct SetPrefixCall;
    ///Container type for all input parameters for the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
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
    #[ethcall(name = "UPGRADE_INTERFACE_VERSION", abi = "UPGRADE_INTERFACE_VERSION()")]
    pub struct UpgradeInterfaceVersionCall;
    ///Container type for all input parameters for the `changeMainAuthorizer` function with signature `changeMainAuthorizer(address)` and selector `0x4c930607`
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
    #[ethcall(name = "changeMainAuthorizer", abi = "changeMainAuthorizer(address)")]
    pub struct ChangeMainAuthorizerCall {
        pub new_main_authorizer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `computeSignedMsg` function with signature `computeSignedMsg(string,string,bytes32)` and selector `0x4bcbbe96`
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
        name = "computeSignedMsg",
        abi = "computeSignedMsg(string,string,bytes32)"
    )]
    pub struct ComputeSignedMsgCall {
        pub prefix: ::std::string::String,
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `dkimPublicKeyHashes` function with signature `dkimPublicKeyHashes(string,bytes32,address)` and selector `0xe308de0c`
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
        name = "dkimPublicKeyHashes",
        abi = "dkimPublicKeyHashes(string,bytes32,address)"
    )]
    pub struct DkimPublicKeyHashesCall(
        pub ::std::string::String,
        pub [u8; 32],
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `enabledTimeOfDKIMPublicKeyHash` function with signature `enabledTimeOfDKIMPublicKeyHash(bytes32)` and selector `0x7f8e29ba`
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
        name = "enabledTimeOfDKIMPublicKeyHash",
        abi = "enabledTimeOfDKIMPublicKeyHash(bytes32)"
    )]
    pub struct EnabledTimeOfDKIMPublicKeyHashCall(pub [u8; 32]);
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256)` and selector `0x1794bb3c`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,uint256)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub main_authorizer: ::ethers::core::types::Address,
        pub set_timestamp_delay: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32,address)` and selector `0x0b55b37c`
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
        name = "isDKIMPublicKeyHashValid",
        abi = "isDKIMPublicKeyHashValid(string,bytes32,address)"
    )]
    pub struct IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub authorizer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32)` and selector `0xe7a7977a`
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
        name = "isDKIMPublicKeyHashValid",
        abi = "isDKIMPublicKeyHashValid(string,bytes32)"
    )]
    pub struct IsDKIMPublicKeyHashValidCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `mainAuthorizer` function with signature `mainAuthorizer()` and selector `0x7d463648`
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
    #[ethcall(name = "mainAuthorizer", abi = "mainAuthorizer()")]
    pub struct MainAuthorizerCall;
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
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `reactivateDKIMPublicKeyHash` function with signature `reactivateDKIMPublicKeyHash(string,bytes32,address,bytes)` and selector `0x32e1e194`
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
        name = "reactivateDKIMPublicKeyHash",
        abi = "reactivateDKIMPublicKeyHash(string,bytes32,address,bytes)"
    )]
    pub struct ReactivateDKIMPublicKeyHashCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub authorizer: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `reactivatedDKIMPublicKeyHashes` function with signature `reactivatedDKIMPublicKeyHashes(bytes32,address)` and selector `0x574900dd`
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
        name = "reactivatedDKIMPublicKeyHashes",
        abi = "reactivatedDKIMPublicKeyHashes(bytes32,address)"
    )]
    pub struct ReactivatedDKIMPublicKeyHashesCall(
        pub [u8; 32],
        pub ::ethers::core::types::Address,
    );
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
    ///Container type for all input parameters for the `revokeDKIMPublicKeyHash` function with signature `revokeDKIMPublicKeyHash(string,bytes32,address,bytes)` and selector `0x82bff8cd`
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
        name = "revokeDKIMPublicKeyHash",
        abi = "revokeDKIMPublicKeyHash(string,bytes32,address,bytes)"
    )]
    pub struct RevokeDKIMPublicKeyHashCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub authorizer: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `revokedDKIMPublicKeyHashes` function with signature `revokedDKIMPublicKeyHashes(bytes32,address)` and selector `0xf0bfb197`
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
        name = "revokedDKIMPublicKeyHashes",
        abi = "revokedDKIMPublicKeyHashes(bytes32,address)"
    )]
    pub struct RevokedDKIMPublicKeyHashesCall(
        pub [u8; 32],
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `setDKIMPublicKeyHash` function with signature `setDKIMPublicKeyHash(string,bytes32,address,bytes)` and selector `0x614a4485`
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
        name = "setDKIMPublicKeyHash",
        abi = "setDKIMPublicKeyHash(string,bytes32,address,bytes)"
    )]
    pub struct SetDKIMPublicKeyHashCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub authorizer: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setDKIMPublicKeyHashes` function with signature `setDKIMPublicKeyHashes(string[],bytes32[],address[],bytes[])` and selector `0x7ff103da`
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
        name = "setDKIMPublicKeyHashes",
        abi = "setDKIMPublicKeyHashes(string[],bytes32[],address[],bytes[])"
    )]
    pub struct SetDKIMPublicKeyHashesCall {
        pub domain_names: ::std::vec::Vec<::std::string::String>,
        pub public_key_hashes: ::std::vec::Vec<[u8; 32]>,
        pub authorizers: ::std::vec::Vec<::ethers::core::types::Address>,
        pub signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `setTimestampDelay` function with signature `setTimestampDelay()` and selector `0x812e12ce`
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
    #[ethcall(name = "setTimestampDelay", abi = "setTimestampDelay()")]
    pub struct SetTimestampDelayCall;
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
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UserOverridableDKIMRegistryCalls {
        ReactivatePrefix(ReactivatePrefixCall),
        RevokePrefix(RevokePrefixCall),
        SetPrefix(SetPrefixCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        ChangeMainAuthorizer(ChangeMainAuthorizerCall),
        ComputeSignedMsg(ComputeSignedMsgCall),
        DkimPublicKeyHashes(DkimPublicKeyHashesCall),
        EnabledTimeOfDKIMPublicKeyHash(EnabledTimeOfDKIMPublicKeyHashCall),
        Initialize(InitializeCall),
        IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(
            IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall,
        ),
        IsDKIMPublicKeyHashValid(IsDKIMPublicKeyHashValidCall),
        MainAuthorizer(MainAuthorizerCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        ReactivateDKIMPublicKeyHash(ReactivateDKIMPublicKeyHashCall),
        ReactivatedDKIMPublicKeyHashes(ReactivatedDKIMPublicKeyHashesCall),
        RenounceOwnership(RenounceOwnershipCall),
        RevokeDKIMPublicKeyHash(RevokeDKIMPublicKeyHashCall),
        RevokedDKIMPublicKeyHashes(RevokedDKIMPublicKeyHashesCall),
        SetDKIMPublicKeyHash(SetDKIMPublicKeyHashCall),
        SetDKIMPublicKeyHashes(SetDKIMPublicKeyHashesCall),
        SetTimestampDelay(SetTimestampDelayCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for UserOverridableDKIMRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ReactivatePrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivatePrefix(decoded));
            }
            if let Ok(decoded) = <RevokePrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePrefix(decoded));
            }
            if let Ok(decoded) = <SetPrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPrefix(decoded));
            }
            if let Ok(decoded) = <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) = <ChangeMainAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeMainAuthorizer(decoded));
            }
            if let Ok(decoded) = <ComputeSignedMsgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeSignedMsg(decoded));
            }
            if let Ok(decoded) = <DkimPublicKeyHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DkimPublicKeyHashes(decoded));
            }
            if let Ok(decoded) = <EnabledTimeOfDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnabledTimeOfDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(decoded),
                );
            }
            if let Ok(decoded) = <IsDKIMPublicKeyHashValidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDKIMPublicKeyHashValid(decoded));
            }
            if let Ok(decoded) = <MainAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MainAuthorizer(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <ReactivateDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivateDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <ReactivatedDKIMPublicKeyHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivatedDKIMPublicKeyHashes(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RevokeDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <RevokedDKIMPublicKeyHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokedDKIMPublicKeyHashes(decoded));
            }
            if let Ok(decoded) = <SetDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <SetDKIMPublicKeyHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDKIMPublicKeyHashes(decoded));
            }
            if let Ok(decoded) = <SetTimestampDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTimestampDelay(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UserOverridableDKIMRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ReactivatePrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokePrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeMainAuthorizer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeSignedMsg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DkimPublicKeyHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnabledTimeOfDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MainAuthorizer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactivateDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactivatedDKIMPublicKeyHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokedDKIMPublicKeyHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDKIMPublicKeyHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTimestampDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UserOverridableDKIMRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ReactivatePrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeMainAuthorizer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeSignedMsg(element) => ::core::fmt::Display::fmt(element, f),
                Self::DkimPublicKeyHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnabledTimeOfDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MainAuthorizer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactivateDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReactivatedDKIMPublicKeyHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokedDKIMPublicKeyHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDKIMPublicKeyHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetTimestampDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ReactivatePrefixCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ReactivatePrefixCall) -> Self {
            Self::ReactivatePrefix(value)
        }
    }
    impl ::core::convert::From<RevokePrefixCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: RevokePrefixCall) -> Self {
            Self::RevokePrefix(value)
        }
    }
    impl ::core::convert::From<SetPrefixCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: SetPrefixCall) -> Self {
            Self::SetPrefix(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<ChangeMainAuthorizerCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ChangeMainAuthorizerCall) -> Self {
            Self::ChangeMainAuthorizer(value)
        }
    }
    impl ::core::convert::From<ComputeSignedMsgCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ComputeSignedMsgCall) -> Self {
            Self::ComputeSignedMsg(value)
        }
    }
    impl ::core::convert::From<DkimPublicKeyHashesCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: DkimPublicKeyHashesCall) -> Self {
            Self::DkimPublicKeyHashes(value)
        }
    }
    impl ::core::convert::From<EnabledTimeOfDKIMPublicKeyHashCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: EnabledTimeOfDKIMPublicKeyHashCall) -> Self {
            Self::EnabledTimeOfDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<
        IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall,
    > for UserOverridableDKIMRegistryCalls {
        fn from(
            value: IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall,
        ) -> Self {
            Self::IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<IsDKIMPublicKeyHashValidCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: IsDKIMPublicKeyHashValidCall) -> Self {
            Self::IsDKIMPublicKeyHashValid(value)
        }
    }
    impl ::core::convert::From<MainAuthorizerCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: MainAuthorizerCall) -> Self {
            Self::MainAuthorizer(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<ReactivateDKIMPublicKeyHashCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ReactivateDKIMPublicKeyHashCall) -> Self {
            Self::ReactivateDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<ReactivatedDKIMPublicKeyHashesCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ReactivatedDKIMPublicKeyHashesCall) -> Self {
            Self::ReactivatedDKIMPublicKeyHashes(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RevokeDKIMPublicKeyHashCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: RevokeDKIMPublicKeyHashCall) -> Self {
            Self::RevokeDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<RevokedDKIMPublicKeyHashesCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: RevokedDKIMPublicKeyHashesCall) -> Self {
            Self::RevokedDKIMPublicKeyHashes(value)
        }
    }
    impl ::core::convert::From<SetDKIMPublicKeyHashCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: SetDKIMPublicKeyHashCall) -> Self {
            Self::SetDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<SetDKIMPublicKeyHashesCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: SetDKIMPublicKeyHashesCall) -> Self {
            Self::SetDKIMPublicKeyHashes(value)
        }
    }
    impl ::core::convert::From<SetTimestampDelayCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: SetTimestampDelayCall) -> Self {
            Self::SetTimestampDelay(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    ///Container type for all return fields from the `REACTIVATE_PREFIX` function with signature `REACTIVATE_PREFIX()` and selector `0x225a08d4`
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
    pub struct ReactivatePrefixReturn(pub ::std::string::String);
    ///Container type for all return fields from the `REVOKE_PREFIX` function with signature `REVOKE_PREFIX()` and selector `0xd507c320`
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
    pub struct RevokePrefixReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SET_PREFIX` function with signature `SET_PREFIX()` and selector `0x07f1eaf5`
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
    pub struct SetPrefixReturn(pub ::std::string::String);
    ///Container type for all return fields from the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
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
    pub struct UpgradeInterfaceVersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `computeSignedMsg` function with signature `computeSignedMsg(string,string,bytes32)` and selector `0x4bcbbe96`
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
    pub struct ComputeSignedMsgReturn(pub ::std::string::String);
    ///Container type for all return fields from the `dkimPublicKeyHashes` function with signature `dkimPublicKeyHashes(string,bytes32,address)` and selector `0xe308de0c`
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
    pub struct DkimPublicKeyHashesReturn(pub bool);
    ///Container type for all return fields from the `enabledTimeOfDKIMPublicKeyHash` function with signature `enabledTimeOfDKIMPublicKeyHash(bytes32)` and selector `0x7f8e29ba`
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
    pub struct EnabledTimeOfDKIMPublicKeyHashReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32,address)` and selector `0x0b55b37c`
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
    pub struct IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashReturn(pub bool);
    ///Container type for all return fields from the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32)` and selector `0xe7a7977a`
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
    pub struct IsDKIMPublicKeyHashValidReturn(pub bool);
    ///Container type for all return fields from the `mainAuthorizer` function with signature `mainAuthorizer()` and selector `0x7d463648`
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
    pub struct MainAuthorizerReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `reactivatedDKIMPublicKeyHashes` function with signature `reactivatedDKIMPublicKeyHashes(bytes32,address)` and selector `0x574900dd`
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
    pub struct ReactivatedDKIMPublicKeyHashesReturn(pub bool);
    ///Container type for all return fields from the `revokedDKIMPublicKeyHashes` function with signature `revokedDKIMPublicKeyHashes(bytes32,address)` and selector `0xf0bfb197`
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
    pub struct RevokedDKIMPublicKeyHashesReturn(pub bool);
    ///Container type for all return fields from the `setTimestampDelay` function with signature `setTimestampDelay()` and selector `0x812e12ce`
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
    pub struct SetTimestampDelayReturn(pub ::ethers::core::types::U256);
}
