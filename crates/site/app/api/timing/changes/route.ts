import { NextRequest, NextResponse } from 'next/server'
import { getServerSupabaseClient } from '@/lib/server-supabase'

export const dynamic = 'force-dynamic'
export const revalidate = 0

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams
  const days = parseInt(searchParams.get('days') || '7')

  try {
    const supabase = getServerSupabaseClient()
    const { data, error } = await supabase
    .rpc('get_timing_signal_changes', {
      days_back: days
    })

    if (error) {
      console.error('Supabase error:', error)
      return NextResponse.json(
        { error: 'Failed to fetch signal changes' },
        { status: 500 }
      )
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

