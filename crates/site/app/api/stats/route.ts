import { NextResponse } from 'next/server'
import { getServerSupabaseClient } from '@/lib/server-supabase'

export const dynamic = 'force-dynamic'
export const revalidate = 0

// Cache stats for 5 minutes
let cachedStats: any = null
let cacheTime = 0
const CACHE_DURATION = 5 * 60 * 1000 // 5 minutes

export async function GET() {
  const now = Date.now()

  // Return cached data if still valid
  if (cachedStats && (now - cacheTime) < CACHE_DURATION) {
    return NextResponse.json(cachedStats)
  }

  try {
    const supabase = getServerSupabaseClient()
    const { data, error } = await supabase
      .from('timing')
      .select('trading_signal, tts_score')

    if (error) {
      console.error('Supabase error:', error)
      return NextResponse.json(
        { error: 'Failed to fetch stats' },
        { status: 500 }
      )
    }

    const total = data?.length || 0
    const buyCount = data?.filter(
      (t: any) => t.trading_signal === 'Buy' || t.trading_signal === 'StrongBuy'
    ).length || 0
    const sellCount = data?.filter(
      (t: any) => t.trading_signal === 'Sell' || t.trading_signal === 'StrongSell'
    ).length || 0
    const neutralCount = data?.filter(
      (t: any) => t.trading_signal === 'Neutral'
    ).length || 0
    const avgScore = data?.reduce((sum: number, t: any) => sum + t.tts_score, 0) / total || 0

    const stats = {
      total,
      buyCount,
      sellCount,
      neutralCount,
      avgScore: parseFloat(avgScore.toFixed(2)),
    }

    // Update cache
    cachedStats = stats
    cacheTime = now

    return NextResponse.json(stats)
  } catch (error) {
    console.error('API error:', error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
}

