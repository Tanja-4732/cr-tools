import init, { run_app } from "../package/cr_tools.js";
async function main() {
  await init("/cr_tools_bg.wasm");
  run_app();
}
main();
