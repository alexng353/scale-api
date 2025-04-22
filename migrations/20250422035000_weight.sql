-- Add migration script here
CREATE TABLE IF NOT EXISTS weights (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  user_id UUID NOT NULL REFERENCES users(id),
  weightLBs FLOAT NOT NULL,
  weightKGs FLOAT NOT NULL,
);
