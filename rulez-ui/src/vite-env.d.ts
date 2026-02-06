/// <reference types="vite/client" />

// Extend Window interface for Tauri
interface Window {
  __TAURI__?: {
    [key: string]: unknown;
  };
}
