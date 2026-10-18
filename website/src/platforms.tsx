import { Command, Terminal } from "lucide-react";
import type { ReactNode } from "react";

export const REPO = "https://github.com/Mxqius/ForceConnect";
export const RELEASES = `${REPO}/releases/latest`;

export type PlatformId = "windows" | "macos" | "linux";

export interface Platform {
  id: PlatformId;
  name: string;
  icon: ReactNode;
  /** What the visitor needs, shown under the name. */
  detail: string;
  /** Flip to true once a release ships a build for this platform. */
  available: boolean;
}

export const PLATFORMS: readonly Platform[] = [
  {
    id: "windows",
    name: "Windows",
    icon: <WindowsIcon />,
    detail: "Windows 10 (2004)+ · x64",
    available: true,
  },
  {
    id: "macos",
    name: "macOS",
    icon: <Command />,
    detail: "Apple Silicon & Intel",
    available: false,
  },
  {
    id: "linux",
    name: "Linux",
    icon: <Terminal />,
    detail: ".deb · .rpm · AppImage",
    available: false,
  },
];

/** Best guess at the visitor's OS, so the hero can offer the right build. */
export function detectPlatform(): PlatformId | null {
  if (typeof navigator === "undefined") return null;
  const nav = navigator as Navigator & {
    userAgentData?: { platform?: string };
  };
  const ua = (nav.userAgentData?.platform ?? nav.userAgent).toLowerCase();
  // Phones and tablets get no desktop build to download.
  if (/android|iphone|ipad|ipod/.test(nav.userAgent.toLowerCase())) return null;
  if (ua.includes("win")) return "windows";
  if (ua.includes("mac")) return "macos";
  if (ua.includes("linux") || ua.includes("x11")) return "linux";
  return null;
}

function WindowsIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden>
      <path d="M3 5.1 10.4 4v7.1H3V5.1Zm8.4-1.2L21 2.5v8.6h-9.6V3.9ZM3 12.9h7.4V20L3 18.9v-6Zm8.4 0H21v8.6l-9.6-1.4v-7.2Z" />
    </svg>
  );
}
