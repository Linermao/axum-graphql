CREATE TABLE IF NOT EXISTS projects (
    project_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    default_rf_canva_id UUID REFERENCES rf_canvas (id) ON DELETE SET NULL,
);