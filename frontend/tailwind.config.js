/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                'brand-primary': 'var(--primary-color, #06b6d4)',
                'brand-secondary': 'var(--secondary-color, #4f46e5)',
            }
        },
    },
    plugins: [],
}
