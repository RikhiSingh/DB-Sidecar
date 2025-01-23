CREATE TABLE "workspaces" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"created_at" timestamp with time zone,
	"workspace_owner" uuid NOT NULL,
	"title" text NOT NULL,
	"icon_id" text NOT NULL,
	"data" text NOT NULL,
	"in_trash" text NOT NULL,
	"logo" text NOT NULL,
	"banner_url" text NOT NULL
);
