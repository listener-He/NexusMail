/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      backgroundImage: {
        'lumina-primary': 'linear-gradient(135deg, #00F2FE 0%, #4FACFE 100%)',
        'lumina-success': 'linear-gradient(135deg, #0BA360 0%, #3CBA92 100%)',
        'lumina-danger': 'linear-gradient(135deg, #FF0844 0%, #FFB199 100%)',
        'lumina-active': 'linear-gradient(135deg, #6EE7B7, #3B82F6)',
        'lumina-archive': 'linear-gradient(135deg, #9CA3AF, #4B5563)',
        'lumina-focus': 'linear-gradient(to bottom, #1F2937, #111827)',
      },
    },
  },
  plugins: [],
}
