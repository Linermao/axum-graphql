CREATE TABLE IF NOT EXISTS rf_edges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    canva_id UUID NOT NULL REFERENCES rf_canvas (id) ON DELETE CASCADE,
    source_node_id UUID NOT NULL REFERENCES rf_nodes (id) ON DELETE CASCADE,
    target_node_id UUID NOT NULL REFERENCES rf_nodes (id) ON DELETE CASCADE,
    edge_type TEXT NOT NULL,
    source_handle TEXT,
    target_handle TEXT,
    animated BOOLEAN NOT NULL DEFAULT false,
    label TEXT,
    data JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (
        source_node_id <> target_node_id
    )
);

CREATE INDEX idx_rf_edges_canva_id ON rf_edges (canva_id);

CREATE INDEX idx_rf_edges_source ON rf_edges (source_node_id);

CREATE INDEX idx_rf_edges_target ON rf_edges (target_node_id);