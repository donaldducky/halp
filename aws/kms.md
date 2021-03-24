# AWS Key Management Service (KMS)

## v2

v2's API [changed](https://github.com/aws/aws-cli/issues/4994)

```bash
# encrypt a value
aws kms encrypt --key-id "<key-id>" --cli-binary-format raw-in-base64-out --plaintext "<plain-text>" | jq -r '.CiphertextBlob'

# using terraform and exported key id
aws kms encrypt --key-id "$(terraform output -raw <my_exported_key_id>)" --cli-binary-format raw-in-base64-out --plaintext "<plain-text>" | jq -r '.CiphertextBlob'

# decrypt a value
aws kms decrypt --key-id "<key-id>" --ciphertext-blob "<ciphertext-blob-from-encrypt>"
```

## TODO

- [ ] where do we look for breaking changes between versions?
- [ ] is there a way to skip jq and [use](https://github.com/aws/aws-cli/issues/4994#issuecomment-679133083) `--output text --query CiphertextBlob`? It seems broken for me using:
  ```bash
  aws --version
  aws-cli/2.1.31 Python/3.9.2 Darwin/20.2.0 source/x86_64 prompt/off

  aws kms encrypt --key-id "<key-id>" --cli-binary-format raw-in-base64-out --plaintext "blah" --text plain --query CiphertextBlob
  parse error: Invalid numeric literal at line 2, column 0
  ```
