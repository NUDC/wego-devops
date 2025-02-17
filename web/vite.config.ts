import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueJsx from '@vitejs/plugin-vue-jsx';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { AntDesignVueResolver } from 'unplugin-vue-components/resolvers';
import path from 'path';
import tailwindcss from '@tailwindcss/vite';

// https://vitejs.dev/config/
export default defineConfig(() => {
  return {
    server: {
      port: parseInt(process.env.VITE_CLI_PORT || '3000'),
      strictPort: true, // 强制使用指定端口
    },
    resolve: {
      alias: {
        '@': path.resolve(__dirname, 'src'),
      },
    },
    plugins: [
      tailwindcss(),
      vue(),
      vueJsx(),
      AutoImport({
        dts: 'types/auto-imports.d.ts',
        imports: ['vue', 'vue-router', 'pinia'],
      }),
      Components({
        dts: 'types/components.d.ts',
        deep: true,
        dirs: ['src/components'],
        extensions: ['vue', 'tsx'],
        exclude: ['Example.vue'],
        resolvers: [
          AntDesignVueResolver({
            importStyle: false,
          }),
        ],
      }),
    ],
    optimizeDeps: {
      include: ['@ant-design/icons-vue', 'ant-design-vue'],
    },
  };
});
