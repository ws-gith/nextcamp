import type { Config } from 'tailwindcss';
import plugin from 'tailwindcss/plugin';
import defaultTheme from "tailwindcss/defaultTheme"

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {
      fontFamily: {
        "sans": ["'Poppins', sans-serif", ...defaultTheme.fontFamily.sans]
      }
    }
  },

  plugins: [plugin(function ({ addUtilities }) {
    addUtilities({
      '.center': {
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
      },
      '.between': {
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
      }
    })
  })]
} as Config;