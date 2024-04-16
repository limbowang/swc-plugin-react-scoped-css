import { expect, describe, it } from 'vitest';
import { pluginReactScopedCss } from '../src';
import { createRsbuild } from '@rsbuild/core';

describe('plugins/react-scoped-css', () => {
  it('should apply react aliases by default', async () => {
    const rsbuild = await createRsbuild({
      rsbuildConfig: {
        plugins: [pluginReactScopedCss()]
      }
    });

    const configs = await rsbuild.initConfigs();
    expect(configs[0]).toMatchSnapshot();
  });
});
