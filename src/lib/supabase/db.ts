import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import * as dotenv from "dotenv";
import * as schema from "../../../migrations/schema";
import { migrate } from "drizzle-orm/postgres-js/migrator";

dotenv.config({ path: ".env" });

if (!process.env.DB_USER || !process.env.DB_PASSWORD) {
  console.error("🔴 DB_USER or DB_PASSWORD is not set!");
  process.exit(1);
}

const client = postgres({
  database: "postgres",
  port: 5432,
  host: "aws-0-us-west-1.pooler.supabase.com",
  user: process.env.DB_USER,
  password: process.env.DB_PASSWORD,
  max: 5, // Adjust connection pool size
});

const db = drizzle(client, { schema });

const migrateDb = async () => {
  if (process.env.SKIP_MIGRATIONS === "true") {
    console.log("⚠️ Skipping migrations as SKIP_MIGRATIONS is set to true.");
    return;
  }
  try {
    console.log("🟠 Migrating client");
    await migrate(db, { migrationsFolder: "./drizzle" });
    console.log("🟢 Successfully Migrated");
  } catch (error) {
    console.error("🔴 Error while migrating client", error);
  } finally {
    await client.end(); // Close the connection pool
  }
};

migrateDb();

export default db;
