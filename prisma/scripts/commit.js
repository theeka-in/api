//@ts-check

import { existsSync, readFileSync, writeFileSync } from "fs";
import path from "path";

const latestMigrateName = process.argv[2]

if (!latestMigrateName) {
    console.log("please enter a valid migration name");
    process.exit(0)
}

const dir = process.cwd()
const latestMigrateFilePath = path.join(dir, "migrations", "latest-migrate.json")
const prismaFilePath = path.join(dir, "schema", "schema.prisma")
const prismaFile = readFileSync(prismaFilePath, "utf-8").replaceAll("  ", "").replace(/datasource\s+\w+\s*\{[^}]*\}/g, "")

if (existsSync(latestMigrateFilePath)) {
    /**
     * @type {{ name: string, prismaFile: string }}
     */
    const latestMigrateFile = JSON.parse(readFileSync(latestMigrateFilePath, "utf-8"))

    if (latestMigrateFile.prismaFile === prismaFile) {
        console.log("there are no changes")
        process.exit(0);
    }
}

writeFileSync(latestMigrateFilePath, JSON.stringify({
    name: latestMigrateName,
    prismaFile
}, undefined, 2))

console.log("commit message successfully pushed")