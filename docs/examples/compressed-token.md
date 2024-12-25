# Compressed Token Example

## Create Compressed Token

```typescript
import { createCompressedToken } from '@agentspl/token';

async function deployCompressedToken() {
  const token = await createCompressedToken({
    name: "Compressed Token",
    symbol: "CMPT",
    decimals: 9,
    compression: {
      maxDepth: 14,
      maxBufferSize: 64
    }
  });

  return token;
}
```

## Efficient Operations

```typescript
// Batch mint with compression
await token.batchMint([
  { to: addr1, amount: 100 },
  { to: addr2, amount: 200 }
]);

// Verify compressed state
const proof = await token.getProof(addr1);
const isValid = await token.verifyProof(proof);