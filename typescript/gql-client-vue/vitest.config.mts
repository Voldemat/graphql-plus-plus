import { defineConfig } from 'vitest/config'
import viteVue from '@vitejs/plugin-vue'

export default defineConfig({
    plugins: [viteVue()],
    test: {
        environment: 'jsdom',
    },
})
