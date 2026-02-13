import { useEffect } from "react";
import { AppShell } from "./components/layout/AppShell";
import { OnboardingWizard } from "./components/onboarding/OnboardingWizard";
import { useSettingsStore } from "./stores/settingsStore";

function App() {
  const settings = useSettingsStore((s) => s.settings);
  const isLoaded = useSettingsStore((s) => s.isLoaded);
  const loadSettings = useSettingsStore((s) => s.loadSettings);

  // Load persisted settings on startup
  useEffect(() => {
    void loadSettings();
  }, [loadSettings]);

  // Apply theme class to document
  useEffect(() => {
    const theme = settings.theme;
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
  }, [settings.theme]);

  return (
    <>
      <AppShell />
      {isLoaded && !settings.onboardingComplete && <OnboardingWizard />}
    </>
  );
}

export default App;
