-- Check current state of market_regime tables

-- List all tables with 'regime' in the name
SELECT 
    schemaname, 
    tablename,
    tableowner
FROM pg_tables 
WHERE tablename LIKE '%regime%'
ORDER BY tablename;

-- Check if market_regime table exists and its columns
SELECT 
    table_name,
    column_name,
    data_type,
    is_nullable
FROM information_schema.columns
WHERE table_schema = 'public' 
  AND table_name = 'market_regime'
ORDER BY ordinal_position;

-- Check if market_regime_history table exists and its columns
SELECT 
    table_name,
    column_name,
    data_type,
    is_nullable
FROM information_schema.columns
WHERE table_schema = 'public' 
  AND table_name = 'market_regime_history'
ORDER BY ordinal_position;

-- Check existing views
SELECT 
    schemaname,
    viewname,
    viewowner
FROM pg_views 
WHERE viewname LIKE '%regime%'
ORDER BY viewname;

-- Check existing functions
SELECT 
    n.nspname as schema,
    p.proname as function_name,
    pg_get_function_identity_arguments(p.oid) as arguments
FROM pg_proc p
LEFT JOIN pg_namespace n ON p.pronamespace = n.oid
WHERE n.nspname = 'public' 
  AND p.proname LIKE '%regime%'
ORDER BY function_name;

