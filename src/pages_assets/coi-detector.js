/**
 * COI (Cross-Origin Isolation) detector utilities.
 *
 * This file is embedded into the static bundle at build time (see `src/pages/bundle.rs`).
 * It is intentionally side-effect free so it can be imported by any page/module that
 * wants to surface COI status UI.
 */

export function hasSharedArrayBuffer() {
  try {
    // Some browsers throw when COOP/COEP is not enabled.
    // eslint-disable-next-line no-new
    new SharedArrayBuffer(1);
    return true;
  } catch {
    return false;
  }
}

export function isCrossOriginIsolated() {
  return typeof crossOriginIsolated === "boolean" ? crossOriginIsolated : false;
}

