#!/usr/bin/env node

const { spawnSync } = require("node:child_process");
const { existsSync } = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const exe = path.join(
  root,
  "target",
  "release",
  process.platform === "win32" ? "practicode.exe" : "practicode",
);

function rustInstallCommand() {
  if (process.platform === "win32") {
    return "winget install -e --id Rustlang.Rustup";
  }
  if (process.platform === "darwin") {
    return "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh";
  }
  return "sudo apt update && sudo apt install -y curl build-essential && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh";
}

function printRustInstallHelp() {
  console.error("practicode: Rust/Cargo is required to build the binary on first run.");
  console.error(`Install Rust: ${rustInstallCommand()}`);
  console.error("Then restart your terminal and run practicode again.");
  console.error("More options: https://www.rust-lang.org/tools/install");
}

if (!existsSync(exe)) {
  const build = spawnSync(
    "cargo",
    ["build", "--release", "--locked", "--manifest-path", path.join(root, "Cargo.toml")],
    { stdio: "inherit" },
  );
  if (build.error) {
    console.error(`practicode: failed to run cargo: ${build.error.message}`);
    printRustInstallHelp();
    process.exit(1);
  }
  if (build.status !== 0) {
    printRustInstallHelp();
    process.exit(build.status ?? 1);
  }
}

const run = spawnSync(exe, process.argv.slice(2), {
  cwd: process.cwd(),
  stdio: "inherit",
});

if (run.error) {
  console.error(`practicode: failed to run binary: ${run.error.message}`);
  process.exit(1);
}
process.exit(run.status ?? 1);
