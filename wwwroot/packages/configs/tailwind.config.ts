import type { Config } from 'tailwindcss';
import plugin from 'tailwindcss/plugin';

export default {
    content: [],

    plugins: [
        plugin(function ({ addUtilities }) {
            addUtilities({
                '.center': {
                    display: 'flex',
                    'justify-content': 'center',
                    'align-items': 'center',
                },
                '.between': {
                    display: 'flex',
                    'justify-content': 'space-between',
                    'align-items': 'center',
                },
            });
        }),
    ],
} satisfies Config;
