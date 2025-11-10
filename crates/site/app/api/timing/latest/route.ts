import { NextResponse } from 'next/server'
import { getServerSupabaseClient } from '@/lib/server-supabase'

export const dynamic = 'force-dynamic'
export const revalidate = 0

export async function GET() {
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
      .order('symbol', { ascending: true })

    if (error) {
      console.error('Supabase error fetching latest timing:', error)
      return NextResponse.json(
        { error: 'Failed to fetch latest timing data' },
        { status: 500 }
      )
    }

    return NextResponse.json(data ?? [])
  } catch (error) {
    console.error('API error fetching latest timing:', error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
}

