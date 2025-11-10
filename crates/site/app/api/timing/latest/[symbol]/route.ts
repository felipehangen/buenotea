import { NextRequest, NextResponse } from 'next/server'
import { getServerSupabaseClient } from '@/lib/server-supabase'

export const dynamic = 'force-dynamic'
export const revalidate = 0

export async function GET(
  request: NextRequest,
  { params }: { params: { symbol: string } }
) {
  const symbol =
    request.nextUrl.searchParams.get('symbol') ??
    params.symbol ??
    request.nextUrl.pathname.split('/').pop() ??
    ''
  const normalizedSymbol = symbol.toUpperCase()

  if (!normalizedSymbol) {
    return NextResponse.json({ error: 'Symbol is required' }, { status: 400 })
  }

  try {
    const supabase = getServerSupabaseClient()
    const columns = [
      'id',
      'symbol',
      'tts_score',
      'trading_signal',
      'confidence_score',
      'rsi_score',
      'macd_score',
      'bollinger_score',
      'ma_score',
      'stochastic_score',
      'williams_score',
      'atr_score',
      'volume_score',
      'short_term_trend',
      'medium_term_trend',
      'long_term_trend',
      'trend_strength',
      'trend_consistency',
      'support_level',
      'resistance_level',
      'support_distance',
      'resistance_distance',
      'support_strength',
      'resistance_strength',
      'current_volume',
      'avg_volume',
      'volume_ratio',
      'volume_trend',
      'vp_relationship',
      'volatility_score',
      'risk_level',
      'max_drawdown_risk',
      'stop_loss',
      'risk_reward_ratio',
      'primary_api_source',
      'fallback_api_source',
      'price_data_points',
      'analysis_period_days',
      'current_price',
      'chatgpt_explanation',
      'trading_suggestion',
      'flags',
      'created_at',
      'updated_at',
    ].join(',')

    const { data, error } = await supabase
      .from('timing')
      .select(columns)
      .eq('symbol', normalizedSymbol)
      .single()

    if (error) {
      if (error.code === 'PGRST116') {
        return NextResponse.json(null)
      }
      console.error(`Supabase error fetching latest timing for ${symbol}:`, error)
      return NextResponse.json(
        { error: 'Failed to fetch timing data' },
        { status: 500 }
      )
    }

    return NextResponse.json(data)
  } catch (error) {
    console.error(`API error fetching timing for ${symbol}:`, error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
}

