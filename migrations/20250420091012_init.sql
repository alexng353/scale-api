-- Add migration script here
CREATE TABLE IF NOT EXISTS "public"."users" (
  "id" UUID NOT NULL DEFAULT uuid_generate_v4(),
  "email" VARCHAR(255) NOT NULL,
  "password" VARCHAR(255) NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("id")
);
