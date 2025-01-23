import {
  pgTable,
  uuid,
  timestamp,
  text,
  foreignKey,
} from "drizzle-orm/pg-core";

export const workspaces = pgTable("workspaces", {
  id: uuid().defaultRandom().primaryKey().notNull(),
  createdAt: timestamp("created_at", { withTimezone: true, mode: "string" }),
  workspaceOwner: uuid("workspace_owner").notNull(),
  title: text().notNull(),
  iconId: text("icon_id").notNull(),
  data: text().notNull(),
  inTrash: text("in_trash").notNull(),
  logo: text().notNull(),
  bannerUrl: text("banner_url").notNull(),
});

export const files = pgTable(
  "files",
  {
    id: uuid().defaultRandom().primaryKey().notNull(),
    createdAt: timestamp("created_at", { withTimezone: true, mode: "string" }),
    title: text().notNull(),
    iconId: text("icon_id").notNull(),
    data: text().notNull(),
    inTrash: text("in_trash").notNull(),
    bannerUrl: text("banner_url").notNull(),
    workspaceId: uuid("workspace_id"),
    folderId: uuid("folder_id"),
  },
  (table) => [
    foreignKey({
      columns: [table.folderId],
      foreignColumns: [folders.id],
      name: "files_folder_id_folders_id_fk",
    }).onDelete("cascade"),
    foreignKey({
      columns: [table.workspaceId],
      foreignColumns: [workspaces.id],
      name: "files_workspace_id_workspaces_id_fk",
    }).onDelete("cascade"),
  ]
);

export const folders = pgTable(
  "folders",
  {
    id: uuid().defaultRandom().primaryKey().notNull(),
    createdAt: timestamp("created_at", { withTimezone: true, mode: "string" }),
    title: text().notNull(),
    iconId: text("icon_id").notNull(),
    data: text().notNull(),
    inTrash: text("in_trash").notNull(),
    bannerUrl: text("banner_url").notNull(),
    workspaceId: uuid("workspace_id"),
  },
  (table) => [
    foreignKey({
      columns: [table.workspaceId],
      foreignColumns: [workspaces.id],
      name: "folders_workspace_id_workspaces_id_fk",
    }).onDelete("cascade"),
  ]
);
