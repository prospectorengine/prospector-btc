📘 CONCEPTO: IDENTITY REFRESH PIPELINE
Ubicación: .documents/architecture/identity_refresh_pipeline.md
El ciclo de vida cambia de lineal a circular:
Dashboard (Inject) -> DB -> Worker (Login) -> Worker (Harvest & Encrypt) -> DB (Update) -> Dashboard (Notify)
Esto garantiza que mientras haya workers activos, las identidades se "curen" a sí mismas, extendiendo su vida útil indefinidamente hasta que Google fuerce un cierre de sesión manual.
1. ACTUALIZACIÓN DE REPOSITORIO (L3 - RUST)
Implementamos la capacidad de Actualización Quirúrgica sin alterar el estado de uso.
Archivo: libs/infra/db-turso/src/repositories/identity/queries.rs
Acción: Agregar la consulta SQL.
code
Rust
// [libs/infra/db-turso/src/repositories/identity/queries.rs]

/// Actualiza las credenciales de una identidad sin resetear sus contadores de uso.
/// Se usa cuando un worker refresca automáticamente las cookies.
pub const REFRESH_IDENTITY_CREDENTIALS: &str = r#"
    UPDATE identities
    SET
        credentials_json = ?2,
        updated_at = CURRENT_TIMESTAMP,
        last_used_at = CURRENT_TIMESTAMP,
        -- Si estaba en cooldown o warning, la revivimos a active porque funcionó
        status = 'active'
    WHERE email = ?1
    RETURNING email
"#;
