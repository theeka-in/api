import "dotenv/config";
import { defineConfig } from "prisma/config";

const dbUrl = () => {
    const host =
        process.env.ENV === "prod" ? process.env.DB_HOST! : "localhost";
    const port = process.env.DB_PORT ?? "5432";
    const user =
        process.env.DB_USER ??
        (() => {
            throw new Error("DB_USER must be set");
        })();
    const password =
        process.env.DB_PASSWORD ??
        (() => {
            throw new Error("DB_PASSWORD must be set");
        })();
    const name =
        process.env.DB_NAME ??
        (() => {
            throw new Error("DB_NAME must be set");
        })();

    return `postgres://${user}:${password}@${host}:${port}/${name}`;
};

export default defineConfig({
    schema: "prisma/schema.prisma",
    migrations: {
        path: "prisma/migrations",
    },
    datasource: {
        url: dbUrl(),
    },
});
