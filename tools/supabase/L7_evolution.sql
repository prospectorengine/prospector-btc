-- =================================================================
-- APARATO: ESTRATIGRAFÍA L7 (USER SERVICES & MONETIZATION)
-- CLASIFICACIÓN: ESTRATO ESTRATÉGICO (MOTOR B)
-- RESPONSABILIDAD: GOBERNANZA DE SUSCRIPCIONES, SEÑALES Y PRESTIGIO
--
-- VISION HIPER-HOLÍSTICA 2026:
-- 1. IDEMPOTENCIA: Uso de guardias relacionales para permitir ejecuciones
--    recursivas sin pérdida de datos ni colisiones de tipos.
-- 2. MULTI-TENANCY RLS: Aislamiento absoluto por 'user_id' para el
--    cumplimiento de la soberanía del dato.
-- 3. PHOENIX PROVISIONING: Automatización de la ignición de cuenta,
--    asegurando que cada operador nazca con sus estratos inicializados.
-- 4. PERFORMANCE: Índices de búsqueda para el rastro forense y notificaciones.
-- =================================================================

-- -----------------------------------------------------------------
-- ESTRATO 0: DEFINICIÓN DE TIPOS SOBERANOS
-- -----------------------------------------------------------------
DO $$
BEGIN
    -- Clasificación de niveles de acceso (Sincronizado con L2 Rust)
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'subscription_tier') THEN
        CREATE TYPE public.subscription_tier AS ENUM ('observer', 'operator', 'architect');
    END IF;

    -- Severidad de señales Herald
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'notification_severity') THEN
        CREATE TYPE public.notification_severity AS ENUM ('info', 'warning', 'critical', 'collision', 'community');
    END IF;
END $$;

-- -----------------------------------------------------------------
-- ESTRATO 1: BILLING (SOBERANÍA FINANCIERA)
-- -----------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.subscriptions (
    user_id uuid REFERENCES public.profiles(id) ON DELETE CASCADE PRIMARY KEY,
    tier public.subscription_tier DEFAULT 'observer',
    status text DEFAULT 'active', -- 'active', 'past_due', 'canceled', 'trialing'
    stripe_customer_id text UNIQUE,
    stripe_subscription_id text UNIQUE,
    current_period_end timestamp with time zone,
    updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS public.billing_credits (
    user_id uuid REFERENCES public.profiles(id) ON DELETE CASCADE PRIMARY KEY,
    compute_energy_balance double precision DEFAULT 100.0, -- Créditos Génesis
    total_consumed_lifetime double precision DEFAULT 0.0,
    last_refill_at timestamp with time zone DEFAULT now()
);

-- -----------------------------------------------------------------
-- ESTRATO 2: HERALD (NERVIO DE COMUNICACIÓN)
-- -----------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.notifications (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid REFERENCES public.profiles(id) ON DELETE CASCADE,
    severity public.notification_severity DEFAULT 'info',
    title text NOT NULL,
    content_json jsonb NOT NULL, -- Almacena metadatos de colisión o mensajes
    is_read_confirmation boolean DEFAULT false,
    dispatch_channel text DEFAULT 'all', -- 'websocket', 'email', 'all'
    created_at timestamp with time zone DEFAULT now()
);

-- -----------------------------------------------------------------
-- ESTRATO 3: NEXUS (REPUTACIÓN Y COMUNIDAD)
-- -----------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.reputation_strata (
    user_id uuid REFERENCES public.profiles(id) ON DELETE CASCADE PRIMARY KEY,
    experience_points bigint DEFAULT 0,
    current_mastery_level int DEFAULT 1,
    rank_title_label text DEFAULT 'Novice_Archaeologist',
    achievements_unlocked jsonb DEFAULT '[]',
    updated_at timestamp with time zone DEFAULT now()
);

-- -----------------------------------------------------------------
-- ESTRATO 4: CHRONICLES (ARQUEOLOGÍA DE INFORMACIÓN)
-- -----------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.forensic_articles (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    author_id uuid REFERENCES public.profiles(id),
    title_key text NOT NULL, -- Mapeado a i18n
    body_markdown text NOT NULL,
    category text DEFAULT 'entropy_research',
    is_published boolean DEFAULT false,
    published_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now()
);

-- -----------------------------------------------------------------
-- ESTRATO 5: AUTOMATIZACIÓN DE IGNICIÓN (TRIGGERS)
-- -----------------------------------------------------------------
-- Esta función garantiza que al crearse un perfil, todos los servicios
-- L7 se activen atómicamente.
CREATE OR REPLACE FUNCTION public.handle_l7_account_provisioning()
RETURNS trigger AS $$
BEGIN
    -- Inicializar Subscripción (Nivel Observador)
    INSERT INTO public.subscriptions (user_id, tier) VALUES (new.id, 'observer');

    -- Inyectar Créditos de Energía Iniciales
    INSERT INTO public.billing_credits (user_id, compute_energy_balance) VALUES (new.id, 100.0);

    -- Forjar Perfil de Reputación
    INSERT INTO public.reputation_strata (user_id) VALUES (new.id);

    -- Notificación de Handshake Exitoso
    INSERT INTO public.notifications (user_id, title, content_json, severity)
    VALUES (new.id, 'Uplink Established', '{"msg": "Welcome to the Swarm. Neural link confirmed."}', 'info');

    RETURN new;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Disparador de Provisión
DROP TRIGGER IF EXISTS on_profile_created_initialize_l7 ON public.profiles;
CREATE TRIGGER on_profile_created_initialize_l7
    AFTER INSERT ON public.profiles
    FOR EACH ROW EXECUTE PROCEDURE public.handle_l7_account_provisioning();

-- -----------------------------------------------------------------
-- ESTRATO 6: SEGURIDAD PERIMETRAL (RLS)
-- -----------------------------------------------------------------
ALTER TABLE public.subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.billing_credits ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.notifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.reputation_strata ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.forensic_articles ENABLE ROW LEVEL SECURITY;

-- Políticas de Privacidad: Solo el dueño ve sus finanzas y mensajes
CREATE POLICY "Users can view own subscription" ON public.subscriptions FOR SELECT USING (auth.uid() = user_id);
CREATE POLICY "Users can view own energy balance" ON public.billing_credits FOR SELECT USING (auth.uid() = user_id);
CREATE POLICY "Users can manage own notifications" ON public.notifications FOR ALL USING (auth.uid() = user_id);

-- Políticas Públicas: Los rangos y artículos son visibles para la comunidad
CREATE POLICY "Public can view reputation strata" ON public.reputation_strata FOR SELECT USING (true);
CREATE POLICY "Public can read published articles" ON public.forensic_articles FOR SELECT USING (is_published = true);

-- -----------------------------------------------------------------
-- ESTRATO 7: OPTIMIZACIÓN DE ACCESO (ÍNDICES)
-- -----------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_notif_unread ON public.notifications (user_id) WHERE (is_read_confirmation = false);
CREATE INDEX IF NOT EXISTS idx_reputation_global ON public.reputation_strata (experience_points DESC);
CREATE INDEX IF NOT EXISTS idx_articles_published ON public.forensic_articles (published_at DESC) WHERE (is_published = true);
