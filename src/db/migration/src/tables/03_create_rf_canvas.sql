CREATE TABLE IF NOT EXISTS rf_canvas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    project_id UUID NOT NULL REFERENCES projects (project_id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (project_id, name)
);

CREATE INDEX idx_rf_canvas_project_id ON rf_canvas (project_id);