/** @type {import('tailwindcss').Config} */
module.exports = {
	content: {
		files: ["./templates/**/*.html"],
	},
	theme: {
		extend: {
			height: {
				almostscreen: '90vh',
				halfscreen: '60vh',
			}
		},
	},
	plugins: [],
}
