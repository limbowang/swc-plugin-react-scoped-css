import { mergeRsbuildConfig, type RsbuildPlugin } from '@rsbuild/core';
import { PluginReactScopedCssOptions } from './types';

export const pluginReactScopedCss = (opts: PluginReactScopedCssOptions = {}): RsbuildPlugin => {
  const options = opts ? {...opts} : {} as PluginReactScopedCssOptions;

  if (options?.include) {
    const type = Object.prototype.toString.call(options.include);
    if (type === '[object RegExp]') {
      options.include = options.include.toString().slice(1, -1);
    } else if (type !== '[object String]') {
      throw new Error('The type of include must be RegExp or String');
    }
  }

  return {
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
                      options,
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
  }
};
