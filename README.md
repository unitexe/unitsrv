# unitsrv

## Usage
Get device's wg pubkey:
```bash
grpcurl -plaintext -import-path ./proto -proto wireguard.proto -d '{}' '[::1]:50051' unit.network.v0.Wireguard/GetPublicKey
```
