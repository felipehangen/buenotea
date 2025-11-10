import { createClient } from '@supabase/supabase-js'

const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL!
const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!

export const supabase = createClient(supabaseUrl, supabaseAnonKey)

// Types
export interface TimingRecord {
  id: number
  symbol: string
  tts_score: number
  trading_signal: string
  confidence_score: number
  rsi_score: number
  macd_score: number
  bollinger_score: number
  ma_score: number
  stochastic_score: number
  williams_score: number
  atr_score: number
  volume_score: number
  short_term_trend: string
  medium_term_trend: string
  long_term_trend: string
  trend_strength: number
  trend_consistency: number
  support_level: number
  resistance_level: number
  support_distance: number
  resistance_distance: number
  support_strength: number
  resistance_strength: number
  current_volume: number
  avg_volume: number
  volume_ratio: number
  volume_trend: string
  vp_relationship: string
  volatility_score: number
  risk_level: string
  max_drawdown_risk: number
  stop_loss: number
  risk_reward_ratio: number
  primary_api_source: string
  current_price: number
  created_at: string
  updated_at: string
}

export interface TimingHistoryRecord {
  symbol: string
  tts_score: number
  trading_signal: string
  confidence_score: number
  short_term_trend: string
  medium_term_trend: string
  long_term_trend: string
  risk_level: string
  created_at: string
}

// Helper functions

/**
 * Get latest timing data for all stocks
 */
export async function getLatestTiming(): Promise<TimingRecord[]> {
  const { data, error } = await supabase
    .from('timing')
    .select('*')
    .order('symbol', { ascending: true })

  if (error) {
    console.error('Error fetching timing data:', error)
    throw error
  }

  return data || []
}

/**
 * Get timing data for a specific symbol
 */
export async function getTimingBySymbol(symbol: string): Promise<TimingRecord | null> {
  const { data, error } = await supabase
    .from('timing')
    .select('*')
    .eq('symbol', symbol)
    .single()

  if (error) {
    console.error(`Error fetching timing for ${symbol}:`, error)
    return null
  }

  return data
}

/**
 * Get timing history for a symbol
 * Note: This should be called server-side via API route to use the RPC function
 */
export async function getTimingHistory(symbol: string, days: number = 90): Promise<TimingHistoryRecord[]> {
  const { data, error } = await supabase
    .rpc('get_timing_history', {
      stock_symbol: symbol,
      days_back: days
    })

  if (error) {
    console.error(`Error fetching history for ${symbol}:`, error)
    return []
  }

  return data || []
}

/**
 * Get signal changes in the last N days
 */
export async function getSignalChanges(days: number = 7) {
  const { data, error } = await supabase
    .rpc('get_timing_signal_changes', {
      days_back: days
    })

  if (error) {
    console.error('Error fetching signal changes:', error)
    return []
  }

  return data || []
}

/**
 * Get stocks by signal type
 */
export async function getStocksBySignal(signal: string): Promise<TimingRecord[]> {
  const { data, error } = await supabase
    .from('timing')
    .select('*')
    .eq('trading_signal', signal)
    .order('tts_score', { ascending: false })

  if (error) {
    console.error(`Error fetching ${signal} stocks:`, error)
    return []
  }

  return data || []
}

/**
 * Get aggregate statistics
 */
export async function getStats() {
  const { data: allTiming, error } = await supabase
    .from('timing')
    .select('trading_signal, tts_score')

  if (error) {
    console.error('Error fetching stats:', error)
    return {
      total: 0,
      buyCount: 0,
      sellCount: 0,
      neutralCount: 0,
      avgScore: 0,
    }
  }

  const total = allTiming?.length || 0
  const buyCount = allTiming?.filter(t => t.trading_signal === 'Buy' || t.trading_signal === 'StrongBuy').length || 0
  const sellCount = allTiming?.filter(t => t.trading_signal === 'Sell' || t.trading_signal === 'StrongSell').length || 0
  const neutralCount = allTiming?.filter(t => t.trading_signal === 'Neutral').length || 0
  const avgScore = allTiming?.reduce((sum, t) => sum + t.tts_score, 0) / total || 0

  return {
    total,
    buyCount,
    sellCount,
    neutralCount,
    avgScore: parseFloat(avgScore.toFixed(2)),
  }
}

