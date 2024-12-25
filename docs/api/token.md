# Token API Reference

## Classes

### Token
Main class for token operations.

#### Methods

##### `createToken(config: TokenConfig): Promise<Token>`
Creates a new token with the specified configuration.

```typescript
interface TokenConfig {
  name: string;
  symbol: string;
  decimals: number;
  initialSupply?: number;
  authority?: PublicKey;
}
```

##### `mint(params: MintParams): Promise<void>`
Mints new tokens.

```typescript
interface MintParams {
  to: PublicKey;
  amount: number;
}
```

##### `transfer(params: TransferParams): Promise<void>`
Transfers tokens between accounts.

```typescript
interface TransferParams {
  from: PublicKey;
  to: PublicKey;
  amount: number;
}
```