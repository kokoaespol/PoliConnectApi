// @ts-check
import { join } from 'path';

import { skeleton } from '@skeletonlabs/tw-plugin';

/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'media',
  content: [
    './src/**/*.{html,js,svelte,ts}',
    join(require.resolve(
            '@skeletonlabs/skeleton'),
        '../**/*.{html,js,svelte,ts}'
    )
  ],
  theme: {
    extend: {},
  },
  plugins: [
    skeleton
  ]
}

