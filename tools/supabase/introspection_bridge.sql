-- =================================================================
-- APARATO: INTROSPECCIÓN SOBERANA (SQL BRIDGE)
-- RESPONSABILIDAD: EXPOSICIÓN DE METADATOS DE SEGURIDAD PARA LA IA
-- =================================================================

CREATE OR REPLACE FUNCTION public.inspect_strategic_topology()
RETURNS JSONB
LANGUAGE plpgsql
SECURITY DEFINER -- Ejecuta con privilegios de sistema
AS $$
DECLARE
    topology_result JSONB;
BEGIN
    topology_result = jsonb_build_object(
        'tables', (
            SELECT jsonb_agg(jsonb_build_object('name', tablename, 'rls_enabled', rowsecurity))
            FROM pg_tables WHERE schemaname = 'public'
        ),
        'policies', (
            SELECT jsonb_agg(jsonb_build_object('table', tablename, 'name', policyname, 'action', cmd, 'roles', roles))
            FROM pg_policies WHERE schemaname = 'public'
        ),
        'triggers', (
            SELECT jsonb_agg(jsonb_build_object('name', tgname, 'table', relname))
            FROM pg_trigger t JOIN pg_class c ON t.tgrelid = c.oid JOIN pg_namespace n ON c.relnamespace = n.oid
            WHERE n.nspname = 'public' AND t.tgisinternal = false
        )
    );
    RETURN topology_result;
END;
$$;
