const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "anchor",
  programName: "gumdrop",
  programId: "gdrpGjVffourzkdDRrQmySw4aTHr8a3xmQzzxSwFD1a",
  idlDir,
  idlName: "gumdrop",
  binaryInstallDir,
  programDir: path.join(programDir, "mpl-gumdrop"),
  rustbin: {
    locked: true,
    versionRangeFallback: "0.27.0",
  },
});
