import { useEffect } from "react";
import { AppShell } from "./components/layout/AppShell";
import { useUIStore } from "./stores/uiStore";

function App() {
  const { theme, setTheme } = useUIStore();

  // Initialize theme from system preference or localStorage
  useEffect(() => {
    const stored = localStorage.getItem("rulez-ui-theme");
    if (stored === "light" || stored === "dark" || stored === "system") {
      setTheme(stored);
    } else {
      // Default to system preference
      setTheme("system");
    }
  }, [setTheme]);

  // Apply theme class to document
  useEffect(() => {
    const root = document.documentElement;
    const isDark =
      theme === "dark" ||
      (theme === "system" && window.matchMedia("(prefers-color-scheme: dark)").matches);

    if (isDark) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }

    // Listen for system preference changes
    if (theme === "system") {
      const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
      const handler = (e: MediaQueryListEvent) => {
        if (e.matches) {
          root.classList.add("dark");
        } else {
          root.classList.remove("dark");
        }
      };
      mediaQuery.addEventListener("change", handler);
      return () => mediaQuery.removeEventListener("change", handler);
    }
  }, [theme]);

  return <AppShell />;
}

export default App;
