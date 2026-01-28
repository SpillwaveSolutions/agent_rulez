import { Header } from "./Header";
import { Sidebar } from "./Sidebar";
import { MainContent } from "./MainContent";
import { RightPanel } from "./RightPanel";
import { StatusBar } from "./StatusBar";
import { useUIStore } from "@/stores/uiStore";

export function AppShell() {
  const { sidebarOpen } = useUIStore();

  return (
    <div className="flex flex-col h-screen w-screen overflow-hidden bg-white dark:bg-[#1A1A1A]">
      {/* Header */}
      <Header />

      {/* Main content area */}
      <div className="flex flex-1 overflow-hidden">
        {/* Left sidebar */}
        {sidebarOpen && <Sidebar />}

        {/* Editor area */}
        <MainContent />

        {/* Right panel (Simulator/Tree) */}
        <RightPanel />
      </div>

      {/* Status bar */}
      <StatusBar />
    </div>
  );
}
