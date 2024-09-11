// tauri-v2/tooling/api/src/core.ts
async function invoke(cmd, args = {}, options) {
  return window.__TAURI_INTERNALS__.invoke(cmd, args, options);
}

// tauri-plugins/plugins/dialog/guest-js/index.ts
async function open(options = {}) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  return invoke("plugin:dialog|open", { options });
}
async function save(options = {}) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  return invoke("plugin:dialog|save", { options });
}
async function message(message2, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invoke("plugin:dialog|message", {
    message: message2.toString(),
    title: opts?.title?.toString(),
    type_: opts?.type,
    okButtonLabel: opts?.okLabel?.toString()
  });
}
async function ask(message2, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invoke("plugin:dialog|ask", {
    message: message2.toString(),
    title: opts?.title?.toString(),
    type_: opts?.type,
    okButtonLabel: opts?.okLabel?.toString() ?? "Yes",
    cancelButtonLabel: opts?.cancelLabel?.toString() ?? "No"
  });
}
async function confirm(message2, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invoke("plugin:dialog|confirm", {
    message: message2.toString(),
    title: opts?.title?.toString(),
    type_: opts?.type,
    okButtonLabel: opts?.okLabel?.toString() ?? "Ok",
    cancelButtonLabel: opts?.cancelLabel?.toString() ?? "Cancel"
  });
}
export {
  ask,
  confirm,
  message,
  open,
  save
};
