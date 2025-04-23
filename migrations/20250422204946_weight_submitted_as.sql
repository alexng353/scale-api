-- Add migration script here

CREATE TYPE weight_unit AS ENUM ('LBs', 'KGs');

ALTER TABLE weights ADD COLUMN unit weight_unit NOT NULL DEFAULT 'LBs';
