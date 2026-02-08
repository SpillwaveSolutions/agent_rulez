import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // Light theme
        surface: {
          DEFAULT: "#F5F5F5",
          dark: "#252525",
        },
        accent: {
          DEFAULT: "#3B82F6",
          dark: "#60A5FA",
        },
        // Semantic colors
        success: {
          DEFAULT: "#10B981",
          dark: "#34D399",
        },
        error: {
          DEFAULT: "#EF4444",
          dark: "#F87171",
        },
        warning: {
          DEFAULT: "#F59E0B",
          dark: "#FBBF24",
        },
      },
      fontFamily: {
        mono: [
          "JetBrains Mono",
          "Menlo",
          "Monaco",
          "Consolas",
          "Liberation Mono",
          "Courier New",
          "monospace",
        ],
      },
    },
  },
  plugins: [],
} satisfies Config;
