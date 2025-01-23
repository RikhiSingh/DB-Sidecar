import { defineConfig } from "drizzle-kit";
import * as dotenv from "dotenv";
dotenv.config({ path: ".env" });

if (!process.env.DATABASE_URL) {
  console.log("🔴 DATABASE_URL is not set!");
}

export default defineConfig({
  out: "./drizzle",
  schema: "./src/lib/supabase/schema.ts",
  dialect: "postgresql",
  dbCredentials: {
    database: "postgres",
    port: 5432,
    host: "aws-0-us-west-1.pooler.supabase.com",
    user: process.env.DB_USER,
    password: process.env.DB_PASSWORD,
  },
});
