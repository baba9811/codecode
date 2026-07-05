const { spawnSync } = require("node:child_process");
const path = require("node:path");

if (process.env.CODECODE_SKIP_BUILD === "1") {
  process.exit(0);
}

const root = path.resolve(__dirname, "..");
const build = spawnSync("cargo", ["build", "--release", "--locked"], {
  cwd: root,
  stdio: "inherit",
});

if (build.error) {
  console.warn(`codecode: cargo build skipped: ${build.error.message}`);
  console.warn("codecode: install Rust/Cargo before first run, or set CODECODE_SKIP_BUILD=1.");
  process.exit(0);
}

process.exit(build.status ?? 1);
