import aspectRatio from '@tailwindcss/aspect-ratio';
import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';
import defaultTheme from 'tailwindcss/defaultTheme';
import plugin from 'tailwindcss/plugin';

const fontFamily = ["'Plus Jakarta Sans Variable', sans-serif"];

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			fontFamily: {
				sans: [fontFamily, ...defaultTheme.fontFamily.sans]
			}
		}
	},

	plugins: [
		plugin(function ({ addUtilities }) {
			addUtilities({
				'.center': {
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'center'
				},

				'.between': {
					display: 'flex',
					alignItems: 'center',
					justifyContent: 'space-between'
				}
			});
		}),
		typography,
		forms,
		containerQueries,
		aspectRatio
	]
} as Config;
