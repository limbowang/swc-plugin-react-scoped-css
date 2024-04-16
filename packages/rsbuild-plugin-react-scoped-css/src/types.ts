export type PluginReactScopedCssOptions = {
  /**
   *
   * file include
   * @default /\.scoped\.(c|le|sc|sa)ss$/
   */
  include?: RegExp | string;
  /*
   * base hash seed
   */
  hashSeed?: string;
};
