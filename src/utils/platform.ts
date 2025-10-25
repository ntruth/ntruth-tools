const userAgent =
  typeof navigator !== "undefined" ? navigator.userAgent.toLowerCase() : "";
const nodePlatform = (() => {
  const maybeGlobal = globalThis as { process?: { platform?: string } };
  return maybeGlobal.process?.platform ?? "";
})();

export const isMac =
  /macintosh|mac os x|mac os/i.test(userAgent) || nodePlatform === "darwin";
export const isWindows = /windows/i.test(userAgent) || nodePlatform === "win32";
export const platformLabel = isMac ? "macOS" : isWindows ? "Windows" : "Desktop";

export const hotkeys = {
  launcher: isMac ? "Option + Space" : "Alt + Space",
  clipboard: isMac ? "Command + Shift + V" : "Ctrl + Shift + V",
};
