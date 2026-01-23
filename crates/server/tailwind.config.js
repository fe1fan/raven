/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    "./src/**/*.{rs,html}",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: {
        apple: {
          blue: { DEFAULT: '#007AFF', dark: '#0A84FF' },
          green: { DEFAULT: '#34C759', dark: '#30D158' },
          red: { DEFAULT: '#FF3B30', dark: '#FF453A' },
          yellow: { DEFAULT: '#FFCC00', dark: '#FFD60A' },
          indigo: { DEFAULT: '#5856D6', dark: '#5E5CE6' },
          gray: {
            100: '#F5F5F7', // SystemBackground Light
            200: '#E5E5EA',
            300: '#C7C7CC',
            400: '#AEAEB2',
            500: '#8E8E93',
            600: '#636366',
            700: '#48484A',
            800: '#2C2C2E',
            900: '#1C1C1E', // SystemBackground Dark-ish
          },
          label: '#000000',
          secondaryLabel: 'rgba(60, 60, 67, 0.6)',
          darkLabel: '#FFFFFF',
          darkSecondaryLabel: 'rgba(235, 235, 245, 0.6)',
        }
      },
      borderRadius: {
        'apple-xl': '12px',
        'apple-2xl': '16px',
        'apple-3xl': '22px',
      },
      fontFamily: {
        sans: ['SF Pro', 'Inter', 'system-ui', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-in-out',
        'slide-in': 'slideIn 0.3s ease-in-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideIn: {
          '0%': { transform: 'translateY(-10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}
