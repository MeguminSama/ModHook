const mod_entrypoint = require("$ENTRYPOINT");

// Hard-code support for Moonlight, since it doesn't auto-inject.
if (
  mod_entrypoint &&
  mod_entrypoint["inject"] &&
  typeof mod_entrypoint["inject"] === "function"
) {
  let asar = require("path").resolve(__dirname, "../_app.asar");
  mod_entrypoint.inject(asar);
}
