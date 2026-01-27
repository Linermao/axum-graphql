CREATE TABLE IF NOT EXISTS rf_nodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    canva_id UUID NOT NULL REFERENCES rf_canvas (id) ON DELETE CASCADE,
    node_type TEXT NOT NULL,
    position_x DOUBLE PRECISION NOT NULL,
    position_y DOUBLE PRECISION NOT NULL,
    width DOUBLE PRECISION NOT NULL,
    height DOUBLE PRECISION NOT NULL,
    rotation DOUBLE PRECISION NOT NULL DEFAULT 0,
    z_index INTEGER NOT NULL DEFAULT 0,
    parent_id UUID REFERENCES rf_nodes (id) ON DELETE SET NULL,
    draggable BOOLEAN NOT NULL DEFAULT true,
    selectable BOOLEAN NOT NULL DEFAULT true,
    hidden BOOLEAN NOT NULL DEFAULT false,
    data JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_rf_nodes_canva_id ON rf_nodes (canva_id);

CREATE INDEX idx_rf_nodes_parent_id ON rf_nodes (parent_id);