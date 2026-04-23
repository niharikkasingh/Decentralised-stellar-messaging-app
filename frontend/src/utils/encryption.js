import CryptoJS from "crypto-js";

const SECRET = "tracevia-demo-key"; // change if needed

export function encrypt(text) {
  return CryptoJS.AES.encrypt(text, SECRET).toString();
}

export function decrypt(cipher) {
  try {
    const bytes = CryptoJS.AES.decrypt(cipher, SECRET);
    return bytes.toString(CryptoJS.enc.Utf8) || cipher;
  } catch {
    return cipher;
  }
}