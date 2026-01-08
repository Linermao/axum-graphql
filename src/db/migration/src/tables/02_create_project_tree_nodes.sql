CREATE TABLE IF NOT EXISTS project_tree_nodes (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL,
    parent_id UUID NULL,
    label TEXT NOT NULL,
    position INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_parent FOREIGN KEY (parent_id)
        REFERENCES project_tree_nodes (id) ON DELETE CASCADE,

    CONSTRAINT fk_project FOREIGN KEY (project_id)
        REFERENCES projects (project_id) ON DELETE CASCADE
);