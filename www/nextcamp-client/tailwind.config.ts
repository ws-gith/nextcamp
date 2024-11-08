import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';
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

	plugins: [typography, forms, containerQueries]
} as Config;
