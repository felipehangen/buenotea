import { createClient } from '@supabase/supabase-js'

let cachedClient: ReturnType<typeof createClient> | null = null

export function getServerSupabaseClient() {
  if (cachedClient) {
    return cachedClient
  }

  const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL
  const serviceKey = process.env.SUPABASE_SERVICE_KEY

  if (!supabaseUrl) {
    throw new Error('NEXT_PUBLIC_SUPABASE_URL is not set')
  }

  if (!serviceKey || serviceKey.toLowerCase().includes('your_service_key')) {
    throw new Error('SUPABASE_SERVICE_KEY is not configured. Provide the Supabase service role key for server-side access.')
  }

  cachedClient = createClient(supabaseUrl, serviceKey, {
    auth: {
      persistSession: false,
    },
  })

  return cachedClient
}

