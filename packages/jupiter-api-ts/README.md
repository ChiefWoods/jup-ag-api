# Jupiter API TypeScript Client

```bash
bun add jupiter-api-ts
```

```ts
import { JupiterApi, createJupiterApiClient } from "jupiter-api-ts";

const jupiter = new JupiterApi({
  client: createJupiterApiClient({
    auth: "your-jupiter-api-key"
  }),
});
```

See the repository README for complete TypeScript and Rust examples.
