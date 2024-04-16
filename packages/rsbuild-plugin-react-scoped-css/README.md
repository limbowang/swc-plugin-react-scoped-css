# rsbuild-plugin-react-scoped-css

## Usage

```javascript
import { defineConfig } from '@rsbuild/core';
import { pluginReactScopedCss } from 'rsbuild-plugin-react-scoped-css';

export default defineConfig({
  plugins: [
    pluginReactScopedCss({
      // include: /\.scoped\.(sc|sa|le|c)ss$/,
      // hashSeed: ''
    })
  ]
});
```
