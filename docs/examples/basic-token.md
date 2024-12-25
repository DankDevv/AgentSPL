# Basic Token Example

## Create and Deploy Token

```typescript
import { createToken } from '@agentspl/token';

async function deployToken() {
  // Initialize token
  const token = await createToken({
    name: "Example Token",
    symbol: "EXT",
    decimals: 9
  });

  // Mint initial supply
  await token.mint({
    to: ownerAddress,
    amount: 1_000_000
  });

  return token;
}
```

## Token Operations

```typescript
// Transfer tokens
await token.transfer({
  from: ownerAddress,
  to: recipientAddress,
  amount: 1000
});

// Check balance
const balance = await token.balanceOf(ownerAddress);
```