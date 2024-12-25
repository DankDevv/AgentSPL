# Token Management

## Creating Tokens

```typescript
import { createToken } from '@agentspl/token';

const token = await createToken({
  name: "MyToken",
  symbol: "MTK",
  decimals: 9,
  initialSupply: 1_000_000
});
```

## Minting

```typescript
await token.mint({
  to: recipientAddress,
  amount: 1000
});
```

## Transfers

```typescript
await token.transfer({
  from: senderAddress,
  to: recipientAddress,
  amount: 500
});