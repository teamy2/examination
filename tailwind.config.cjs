/** @type {import('tailwindcss').Config}*/

const colors = require('tailwindcss/colors')
const config = {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {},
	},
	plugins: [
		require('@tailwindcss/line-clamp'),
		require('daisyui')
	],
	daisyui: {
		themes: [
			{
				'light': {
					...require('daisyui/src/theming/themes')['[data-theme=light]'],
					accent: '#661ae6',
					primary: '#198e84',
					secondary: '#1a59bd',
				},
			},
			{
				'dark': {
					...require('daisyui/src/theming/themes')['[data-theme=dark]'],
					accent: '#661ae6',
					primary: '#198e84',
					secondary: '#1a59bd',
				},
			},
		]
	}
};

module.exports = config;
