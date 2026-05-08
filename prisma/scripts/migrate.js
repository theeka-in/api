import { spawn } from "node:child_process"
import { existsSync, readFileSync, rmSync } from "node:fs"
import path from "node:path"

const dir = process.cwd()
const filePath = path.join(dir, "migrations", "latest-migrate.json")

if (!existsSync(filePath)) {
    const proc = spawn("npx", ["prisma", "migrate", "deploy"], { shell: true });

    proc.stdout.on('data', (data) => {
        process.stdout.write(data);
    });

    proc.stderr.on('data', (data) => {
        process.stderr.write(data);
    });

    proc.on('close', (code) => {
        if (existsSync(filePath)) {
            rmSync(filePath, { force: true });
        }
        console.log(`exited with code ${code}`);
    });

    process.exit(0)
}

/**
 * @type {{name: string}}
 */
const latestMigrate = JSON.parse(readFileSync(filePath, "utf-8"))

const proc = spawn("npx", ["prisma", "migrate", "dev", "--name", latestMigrate.name], { shell: true });

proc.stdout.on('data', (data) => {
    process.stdout.write(data);
});

proc.stderr.on('data', (data) => {
    process.stderr.write(data);
});

proc.on('close', (code) => {
    if (existsSync(filePath)) {
        rmSync(filePath, { force: true });
    }
    console.log(`exited with code ${code}`);
});