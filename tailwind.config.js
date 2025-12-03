/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#007AFF',
          dark: '#0051D5',
        },
        background: {
          light: '#FFFFFF',
          dark: '#1C1C1E',
        },
        surface: {
          light: '#F2F2F7',
          dark: '#2C2C2E',
        },
        text: {
          primary: '#000000',
          secondary: '#3C3C43',
          tertiary: '#8E8E93',
          'primary-dark': '#FFFFFF',
          'secondary-dark': '#EBEBF5',
          'tertiary-dark': '#8E8E93',
        },
      },
      borderRadius: {
        DEFAULT: '8px',
      },
      boxShadow: {
        float: '0 8px 24px rgba(0, 0, 0, 0.12)',
      },
    },
  },
  plugins: [],
}
