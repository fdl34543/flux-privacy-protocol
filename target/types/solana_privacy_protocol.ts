/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/solana_privacy_protocol.json`.
 */
export type SolanaPrivacyProtocol = {
  "address": "A1QwxxHo4FXq8UumCmqM8P2iexxSjUY3SKGJFB5zwcMY",
  "metadata": {
    "name": "solanaPrivacyProtocol",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "idlExposePrivacyPool",
      "discriminator": [
        47,
        212,
        32,
        37,
        46,
        102,
        233,
        157
      ],
      "accounts": [
        {
          "name": "privacyPool"
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "docs": [
        "Initialize protocol state and vault accounts"
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "privacyPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  105,
                  118,
                  97,
                  99,
                  121,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "protocolState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  116,
                  111,
                  99,
                  111,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "mint"
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "privateTransfer",
      "discriminator": [
        107,
        20,
        177,
        94,
        33,
        119,
        16,
        110
      ],
      "accounts": [
        {
          "name": "protocolState",
          "writable": true
        },
        {
          "name": "privacyPool",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "oldNullifier",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "newCommitment",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "proofData",
          "type": "bytes"
        },
        {
          "name": "publicInputs",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "shieldTokens",
      "docs": [
        "Shield tokens (public → private)"
      ],
      "discriminator": [
        43,
        77,
        69,
        75,
        38,
        130,
        29,
        188
      ],
      "accounts": [
        {
          "name": "protocolState",
          "writable": true
        },
        {
          "name": "privacyPool",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "vault",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "commitment",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "unshieldTokens",
      "docs": [
        "Unshield tokens (private → public)"
      ],
      "discriminator": [
        65,
        41,
        210,
        244,
        205,
        191,
        91,
        181
      ],
      "accounts": [
        {
          "name": "protocolState",
          "writable": true
        },
        {
          "name": "privacyPool",
          "writable": true
        },
        {
          "name": "vault",
          "writable": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "nullifier",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "proofData",
          "type": "bytes"
        },
        {
          "name": "publicInputs",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "protocolState",
      "discriminator": [
        33,
        51,
        173,
        134,
        35,
        140,
        195,
        248
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "doubleSpend",
      "msg": "Double spend detected"
    }
  ],
  "types": [
    {
      "name": "protocolState",
      "docs": [
        "Global protocol state"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "stateBump",
            "type": "u8"
          },
          {
            "name": "vaultBump",
            "type": "u8"
          },
          {
            "name": "totalShielded",
            "type": "u64"
          },
          {
            "name": "totalPublic",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
