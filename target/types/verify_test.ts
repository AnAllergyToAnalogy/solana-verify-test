/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/verify_test.json`.
 */
export type VerifyTest = {
  "address": "DSieW9K3zXmTtmv6y5QBUquRiquE8HysUDxbcqgk2c44",
  "metadata": {
    "name": "verifyTest",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "testIx",
      "discriminator": [
        171,
        118,
        51,
        73,
        181,
        5,
        163,
        241
      ],
      "accounts": [],
      "args": []
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "customError",
      "msg": "Custom error message"
    }
  ],
  "constants": [
    {
      "name": "seed",
      "type": "string",
      "value": "\"anchor\""
    }
  ]
};
