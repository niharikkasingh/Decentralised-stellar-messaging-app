// src/services/freighter.service.js
// Works with either:
//  - <script src="https://cdnjs.cloudflare.com/ajax/libs/stellar-freighter-api/5.0.0/index.min.js"></script>
//    (=> window.freighterApi)
//  - or npm: import * as freighterApi from "@stellar/freighter-api";

let api = null;

// Try window.freighterApi first (CDN), else dynamic import (npm)
async function getApi() {
  if (api) return api;

  if (typeof window !== "undefined" && window.freighterApi) {
    api = window.freighterApi;
    return api;
  }

  try {
    const mod = await import("@stellar/freighter-api");
    // When bundled as ESM, the functions are top-level exports:
    api = mod;
    return api;
  } catch (e) {
    console.warn("Freighter API (npm) not available:", e?.message || e);
  }

  return null;
}

export async function connectFreighter() {
  const _api = await getApi();
  if (!_api) {
    throw new Error("Freighter not detected. Install/unlock extension and reload.");
  }

  // v5 API returns objects like {isConnected}, {isAllowed}, {address}
  const connected = await _api.isConnected().catch(() => ({ isConnected: false }));
  if (!connected?.isConnected) {
    throw new Error("Freighter extension not connected.");
  }

  const allowed = await _api.isAllowed().catch(() => ({ isAllowed: false }));
  if (!allowed?.isAllowed) {
    const set = await _api.setAllowed();
    if (!set?.isAllowed) throw new Error("User did not allow the app.");
  }

  // Prefer getAddress() (lightweight). Falls back to requestAccess().
  const addrObj = await _api.getAddress().catch(() => ({}));
  let address = addrObj?.address;
  if (!address) {
    const req = await _api.requestAccess();
    if (req?.error) throw new Error(req.error);
    address = req?.address;
  }
  if (!address) throw new Error("Unable to obtain wallet address.");
  return { address };
}

export async function signTxXdr(xdr, opts) {
  const _api = await getApi();
  if (!_api) throw new Error("Freighter API missing.");
  const res = await _api.signTransaction(xdr, opts);
  if (res?.error) throw new Error(res.error?.message || "Freighter signing failed");
  return res.signedTxXdr || res; // CDN vs npm return shapes
}