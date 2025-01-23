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
    url: process.env.DATABASE_URL!,
  },
});
