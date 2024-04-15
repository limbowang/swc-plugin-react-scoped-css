import { mergeRsbuildConfig, type RsbuildPlugin } from '@rsbuild/core';

export const pluginReactScopedCss = (): RsbuildPlugin => ({
  name: 'scope:react-scoped-css',
  setup(api) {
    api.modifyRspackConfig((config) => {
      if (config.module?.rules) {
        for (let rule of config.module.rules) {
          if (rule && (rule as any).oneOf) {
            const oneOfs = (rule as any).oneOf;
            if (Array.isArray(oneOfs)) {
              for (let oneOf of oneOfs) {
                if (oneOf.type === 'css' && Array.isArray(oneOf.use)) {
                  oneOf.use.unshift({
                    loader: require.resolve('scoped-css-loader'),
                  });
                }
              }
            }
          }
        }
      }

      return config;
    });

    api.modifyRsbuildConfig((config) => {
      const newConfig = mergeRsbuildConfig(config, {
        tools: {
          swc: {
            jsc: {
              experimental: {
                plugins: [
                  [
                    require.resolve('swc-plugin-react-scoped-css'),
                    {},
                  ],
                ],
              },
            },
          },
        },
      });
      return newConfig;
    });
  },
});
