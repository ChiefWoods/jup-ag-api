# JavaScript API Client for Jupiter V6

## Table of Contents

- [Installation](#installation)
- [Developing](#developing)
- [Usage](#usage)

## Installation

Install the package:

```bash
bun add @jup-ag/api
```

## Usage

To start using the API client, you need to require it in your Node.js project:

```typescript
import { createJupiterApiClient } from '@jup-ag/api';

const jupiterQuoteApi = createJupiterApiClient(config); // config is optional such as api key
```

Now, you can call methods provided by the API client to interact with Jupiter's API. For example:

```typescript
jupiterQuoteApi.quoteGet({
    inputMint: "So11111111111111111111111111111111111111112",
    outputMint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    amount: "100000000",
    slippageBps: 100,
})
```
