import { componentize } from "@bytecodealliance/componentize-js"
import { readFile, writeFile } from "node:fs/promises"

const { component } = await componentize(
    await readFile("app.mjs", "utf-8"),
    {
        witPath: "../wit",
        worldName: "proxy",
        enableStdout: true,
    }
);

await writeFile("http.wasm", component)
