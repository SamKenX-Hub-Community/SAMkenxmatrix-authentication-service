// Copyright 2022 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Generated enums from the IANA JOSE registry
//!
//! <https://www.iana.org/assignments/jose/jose.xhtml>

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum JsonWebSignatureAlgorithm {
    /// HMAC using SHA-256
    #[serde(rename = "HS256")]
    Hs256,

    /// HMAC using SHA-384
    #[serde(rename = "HS384")]
    Hs384,

    /// HMAC using SHA-512
    #[serde(rename = "HS512")]
    Hs512,

    /// RSASSA-PKCS1-v1_5 using SHA-256
    #[serde(rename = "RS256")]
    Rs256,

    /// RSASSA-PKCS1-v1_5 using SHA-384
    #[serde(rename = "RS384")]
    Rs384,

    /// RSASSA-PKCS1-v1_5 using SHA-512
    #[serde(rename = "RS512")]
    Rs512,

    /// ECDSA using P-256 and SHA-256
    #[serde(rename = "ES256")]
    Es256,

    /// ECDSA using P-384 and SHA-384
    #[serde(rename = "ES384")]
    Es384,

    /// ECDSA using P-521 and SHA-512
    #[serde(rename = "ES512")]
    Es512,

    /// RSASSA-PSS using SHA-256 and MGF1 with SHA-256
    #[serde(rename = "PS256")]
    Ps256,

    /// RSASSA-PSS using SHA-384 and MGF1 with SHA-384
    #[serde(rename = "PS384")]
    Ps384,

    /// RSASSA-PSS using SHA-512 and MGF1 with SHA-512
    #[serde(rename = "PS512")]
    Ps512,

    /// No digital signature or MAC performed
    #[serde(rename = "none")]
    None,

    /// RSAES-PKCS1-v1_5
    #[serde(rename = "RSA1_5")]
    Rsa15,

    /// RSAES OAEP using default parameters
    #[serde(rename = "RSA-OAEP")]
    RsaOaep,

    /// RSAES OAEP using SHA-256 and MGF1 with SHA-256
    #[serde(rename = "RSA-OAEP-256")]
    RsaOaep256,

    /// AES Key Wrap using 128-bit key
    #[serde(rename = "A128KW")]
    A128Kw,

    /// AES Key Wrap using 192-bit key
    #[serde(rename = "A192KW")]
    A192Kw,

    /// AES Key Wrap using 256-bit key
    #[serde(rename = "A256KW")]
    A256Kw,

    /// Direct use of a shared symmetric key
    #[serde(rename = "dir")]
    Dir,

    /// ECDH-ES using Concat KDF
    #[serde(rename = "ECDH-ES")]
    EcdhEs,

    /// ECDH-ES using Concat KDF and "A128KW" wrapping
    #[serde(rename = "ECDH-ES+A128KW")]
    EcdhEsA128Kw,

    /// ECDH-ES using Concat KDF and "A192KW" wrapping
    #[serde(rename = "ECDH-ES+A192KW")]
    EcdhEsA192Kw,

    /// ECDH-ES using Concat KDF and "A256KW" wrapping
    #[serde(rename = "ECDH-ES+A256KW")]
    EcdhEsA256Kw,

    /// Key wrapping with AES GCM using 128-bit key
    #[serde(rename = "A128GCMKW")]
    A128Gcmkw,

    /// Key wrapping with AES GCM using 192-bit key
    #[serde(rename = "A192GCMKW")]
    A192Gcmkw,

    /// Key wrapping with AES GCM using 256-bit key
    #[serde(rename = "A256GCMKW")]
    A256Gcmkw,

    /// PBES2 with HMAC SHA-256 and "A128KW" wrapping
    #[serde(rename = "PBES2-HS256+A128KW")]
    Pbes2Hs256A128Kw,

    /// PBES2 with HMAC SHA-384 and "A192KW" wrapping
    #[serde(rename = "PBES2-HS384+A192KW")]
    Pbes2Hs384A192Kw,

    /// PBES2 with HMAC SHA-512 and "A256KW" wrapping
    #[serde(rename = "PBES2-HS512+A256KW")]
    Pbes2Hs512A256Kw,

    /// EdDSA signature algorithms
    #[serde(rename = "EdDSA")]
    EdDsa,

    /// RSA-OAEP using SHA-384 and MGF1 with SHA-384
    #[serde(rename = "RSA-OAEP-384")]
    RsaOaep384,

    /// RSA-OAEP using SHA-512 and MGF1 with SHA-512
    #[serde(rename = "RSA-OAEP-512")]
    RsaOaep512,

    /// ECDSA using secp256k1 curve and SHA-256
    #[serde(rename = "ES256K")]
    Es256K,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JsonWebEncryptionAlgorithm {
    /// AES_128_CBC_HMAC_SHA_256 authenticated encryption algorithm
    #[serde(rename = "A128CBC-HS256")]
    A128CbcHs256,

    /// AES_192_CBC_HMAC_SHA_384 authenticated encryption algorithm
    #[serde(rename = "A192CBC-HS384")]
    A192CbcHs384,

    /// AES_256_CBC_HMAC_SHA_512 authenticated encryption algorithm
    #[serde(rename = "A256CBC-HS512")]
    A256CbcHs512,

    /// AES GCM using 128-bit key
    #[serde(rename = "A128GCM")]
    A128Gcm,

    /// AES GCM using 192-bit key
    #[serde(rename = "A192GCM")]
    A192Gcm,

    /// AES GCM using 256-bit key
    #[serde(rename = "A256GCM")]
    A256Gcm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JsonWebEncryptionCompressionAlgorithm {
    /// DEFLATE
    #[serde(rename = "DEF")]
    Def,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JsonWebKeyType {
    /// Elliptic Curve
    #[serde(rename = "EC")]
    Ec,

    /// RSA
    #[serde(rename = "RSA")]
    Rsa,

    /// Octet sequence
    #[serde(rename = "oct")]
    Oct,

    /// Octet string key pairs
    #[serde(rename = "OKP")]
    Okp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum JsonWebKeyEcEllipticCurve {
    /// P-256 Curve
    #[serde(rename = "P-256")]
    P256,

    /// P-384 Curve
    #[serde(rename = "P-384")]
    P384,

    /// P-521 Curve
    #[serde(rename = "P-521")]
    P521,

    /// SECG secp256k1 curve
    #[serde(rename = "secp256k1")]
    Secp256K1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum JsonWebKeyOkpEllipticCurve {
    /// Ed25519 signature algorithm key pairs
    #[serde(rename = "Ed25519")]
    Ed25519,

    /// Ed448 signature algorithm key pairs
    #[serde(rename = "Ed448")]
    Ed448,

    /// X25519 function key pairs
    #[serde(rename = "X25519")]
    X25519,

    /// X448 function key pairs
    #[serde(rename = "X448")]
    X448,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum JsonWebKeyUse {
    /// Digital Signature or MAC
    #[serde(rename = "sig")]
    Sig,

    /// Encryption
    #[serde(rename = "enc")]
    Enc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum JsonWebKeyOperation {
    /// Compute digital signature or MAC
    #[serde(rename = "sign")]
    Sign,

    /// Verify digital signature or MAC
    #[serde(rename = "verify")]
    Verify,

    /// Encrypt content
    #[serde(rename = "encrypt")]
    Encrypt,

    /// Decrypt content and validate decryption, if applicable
    #[serde(rename = "decrypt")]
    Decrypt,

    /// Encrypt key
    #[serde(rename = "wrapKey")]
    WrapKey,

    /// Decrypt key and validate decryption, if applicable
    #[serde(rename = "unwrapKey")]
    UnwrapKey,

    /// Derive key
    #[serde(rename = "deriveKey")]
    DeriveKey,

    /// Derive bits not to be used as a key
    #[serde(rename = "deriveBits")]
    DeriveBits,
}