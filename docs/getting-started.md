# Getting Started

## Installation

```bash
npm install @agentspl/token
```

## Quick Setup

```typescript
import { createToken } from '@agentspl/token';

const token = await createToken({
  name: "MyToken",
  symbol: "MTK",
  decimals: 9
});
```

## Prerequisites

- Solana CLI tools (v1.17.0 or higher)
- Node.js (v18 or higher)
- Rust toolchain