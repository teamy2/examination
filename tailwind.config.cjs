/** @type {import('tailwindcss').Config}*/

const colors = require('tailwindcss/colors')
const config = {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		colors: {
			transparent: 'transparent',
			current: 'currentColor',
			blue1: '#1a59bd',
			blue2: '#0c264f',
			turquoise: '#189e97'

	},
		extend: {},
	},
	plugins: [
		require('daisyui')
	],
};

module.exports = config;
