-- tools/supabase/integrity_ledger.sql
-- =================================================================
-- APARATO: SYSTEM INTEGRITY LEDGER (V11.0 - SOBERANO)
-- RESPONSABILIDAD: MEMORIA CENTRAL DE DIAGNÓSTICOS DE INFRAESTRUCTURA
-- =================================================================

CREATE TABLE IF NOT EXISTS public.system_integrity_reports (
    identifier uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    apparatus_name text NOT NULL,        -- 'auditor', 'inspector', 'dumper'
    status text NOT NULL,                -- 'OPERATIONAL', 'DEGRADED', 'CRITICAL'
    metrics jsonb NOT NULL,              -- Telemetría técnica completa
    detected_at_timestamp timestamp with time zone DEFAULT timezone('utc'::text, now()) NOT NULL
);

-- PROTECCIÓN SOBERANA (RLS)
ALTER TABLE public.system_integrity_reports ENABLE ROW LEVEL SECURITY;

-- Solo los miembros autorizados del Cuartel General pueden visualizar la salud del sistema
CREATE POLICY "Operators can view integrity reports" ON public.system_integrity_reports
    FOR SELECT USING (
        EXISTS (
            SELECT 1 FROM public.workspace_members
            WHERE user_id = auth.uid()
        )
    );

-- INDEXACIÓN PARA CONSULTAS SSE DE ALTA VELOCIDAD
CREATE INDEX IF NOT EXISTS idx_integrity_last_seen ON public.system_integrity_reports (apparatus_name, detected_at_timestamp DESC);
