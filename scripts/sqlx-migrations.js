    // @ts-check
    import fs from "fs";
    import path from "path";

    const args = process.argv.slice(2);

    const dryRun = args.includes("--dry-run");
    const prismaDir = path.resolve(path.join("migrations", "_prisma"));
    const sqlxDir = path.resolve("migrations");

    if (!fs.existsSync(prismaDir)) {
        console.error(`prisma migrations folder not found: ${prismaDir}`);
        process.exit(1);
    }

    const entries = fs
        .readdirSync(prismaDir, { withFileTypes: true })
        .filter((e) => e.isDirectory() && !e.name.startsWith("_"))
        .sort((a, b) => a.name.localeCompare(b.name));

    if (!dryRun && !fs.existsSync(sqlxDir)) {
        fs.mkdirSync(sqlxDir, { recursive: true });
    }

    for (const entry of entries) {
        const sqlFile = path.join(prismaDir, entry.name, "migration.sql");
        if (!fs.existsSync(sqlFile)) continue;

        const outFile = path.join(sqlxDir, `${entry.name}.sql`);
        if (!dryRun && fs.existsSync(outFile)) continue;

        const sql = fs.readFileSync(sqlFile, "utf8");

        if (dryRun) {
            console.log(`would write ${entry.name}.sql`);
        } else {
            fs.writeFileSync(outFile, sql, "utf8");
            console.log(`wrote ${entry.name}.sql`);
        }
    }