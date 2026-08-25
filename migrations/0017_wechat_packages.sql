CREATE TABLE IF NOT EXISTS wechat_packages (
    story_id UUID PRIMARY KEY REFERENCES stories(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    summary_md TEXT NOT NULL,
    key_facts JSONB NOT NULL DEFAULT '[]'::jsonb,
    unknowns JSONB NOT NULL DEFAULT '[]'::jsonb,
    viewpoint TEXT NOT NULL DEFAULT '',
    source_note TEXT NOT NULL DEFAULT '',
    image_url TEXT,
    image_origin TEXT NOT NULL CHECK (image_origin IN ('source','victoriapark')),
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft','approved','published')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS wechat_packages_status_idx
    ON wechat_packages (status, updated_at DESC);
