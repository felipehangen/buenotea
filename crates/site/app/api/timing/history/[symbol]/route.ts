import { NextRequest, NextResponse } from 'next/server'
import { getServerSupabaseClient } from '@/lib/server-supabase'

export const dynamic = 'force-dynamic'
export const revalidate = 0

export async function GET(
  request: NextRequest,
  { params }: { params: { symbol: string } }
) {
  const { symbol } = params
  const searchParams = request.nextUrl.searchParams
  const days = parseInt(searchParams.get('days') || '90')

  try {
    const supabase = getServerSupabaseClient()

    // First try the RPC for efficient filtering
    const { data, error } = await supabase
      .rpc('get_timing_history', {
        stock_symbol: symbol.toUpperCase(),
        days_back: days
      })

    if (error) {
      console.error('Supabase error (RPC get_timing_history):', error)
      // Fallback to direct query if RPC fails
      const { data: fallbackData, error: fallbackError } = await supabase
        .from('timing_history')
        .select(
          [
            'id',
            'symbol',
            'tts_score',
            'trading_signal',
            'confidence_score',
            'short_term_trend',
            'medium_term_trend',
            'long_term_trend',
            'risk_level',
            'created_at',
          ].join(',')
        )
        .eq('symbol', symbol.toUpperCase())
        .gte('created_at', new Date(Date.now() - days * 24 * 60 * 60 * 1000).toISOString())
        .order('created_at', { ascending: false })

      if (fallbackError) {
        console.error('Supabase error (fallback query):', fallbackError)
        return NextResponse.json(
          { error: 'Failed to fetch timing history' },
          { status: 500 }
        )
      }

      return NextResponse.json(fallbackData ?? [])
    }

    return NextResponse.json(data || [])
  } catch (error) {
    console.error('API error:', error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
}

