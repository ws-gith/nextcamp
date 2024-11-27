import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';
import tailwind from "@nextcamp/configs/tailwind"

export default {
  presets: [tailwind],
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {}
  },

  plugins: [typography, forms, containerQueries]
} satisfies Config;
