# Compression API Reference

## Classes

### StateCompressor
Handles token state compression.

#### Methods

##### `compress(data: TokenData): Promise<CompressedData>`
Compresses token data for efficient storage.

```typescript
interface TokenData {
  supply: number;
  holders: Map<string, number>;
  metadata: TokenMetadata;
}
```

##### `decompress(data: CompressedData): Promise<TokenData>`
Decompresses token data.

##### `verifyProof(proof: MerkleProof): Promise<boolean>`
Verifies a merkle proof for compressed data.