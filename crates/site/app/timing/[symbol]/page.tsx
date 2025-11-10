'use client'

import { useEffect, useState } from 'react'
import { useParams } from 'next/navigation'
import Link from 'next/link'
import { type TimingRecord, type TimingHistoryRecord } from '@/lib/supabase'
import SignalBadge from '@/components/timing/SignalBadge'
import RiskBadge from '@/components/timing/RiskBadge'
import HistoryChart from '@/components/charts/HistoryChart'
import { formatNumber, formatLargeNumber, getTrendEmoji } from '@/lib/utils'

export default function TickerDetailPage() {
  const params = useParams()
  const symbol = (params.symbol as string)?.toUpperCase()
  const [timingData, setTimingData] = useState<TimingRecord | null>(null)
  const [historyData, setHistoryData] = useState<TimingHistoryRecord[]>([])
  const [loading, setLoading] = useState(true)
  const [historyDays, setHistoryDays] = useState(30)

  useEffect(() => {
    async function loadData() {
      if (!symbol) return

      setLoading(true)
      try {
        const latestResponse = await fetch(`/api/timing/latest/${symbol}`, {
          cache: 'no-store',
        })
        if (!latestResponse.ok) {
          throw new Error(await latestResponse.text())
        }
        const timing: TimingRecord | null = await latestResponse.json()
        setTimingData(timing)

        // Fetch history via API route
        const historyResponse = await fetch(
          `/api/timing/history/${symbol}?days=${historyDays}`
        )
        if (!historyResponse.ok) {
          throw new Error(await historyResponse.text())
        }
        const history: TimingHistoryRecord[] = await historyResponse.json()
        setHistoryData(history)
      } catch (error) {
        console.error('Error loading ticker data:', error)
      } finally {
        setLoading(false)
      }
    }
    loadData()
  }, [symbol, historyDays])

  if (loading) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 text-center">
        <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <p className="text-gray-600 mt-4">Loading {symbol} data...</p>
      </div>
    )
  }

  if (!timingData) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 text-center">
        <h1 className="text-2xl font-bold text-gray-900 mb-4">Stock Not Found</h1>
        <p className="text-gray-600 mb-8">
          Could not find timing data for {symbol}
        </p>
        <Link
          href="/timing"
          className="text-blue-600 hover:text-blue-800 font-medium"
        >
          ← Back to Timing List
        </Link>
      </div>
    )
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {/* Header */}
      <div className="mb-6">
        <Link
          href="/timing"
          className="text-blue-600 hover:text-blue-800 text-sm font-medium mb-4 inline-block"
        >
          ← Back to Timing List
        </Link>
        <h1 className="text-4xl font-bold text-gray-900">{symbol}</h1>
        <p className="text-gray-600 mt-2">Technical Trading Score Analysis</p>
      </div>

      {/* Current Signal Card */}
      <div className="bg-white rounded-lg shadow-lg p-8 mb-6">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div>
            <h3 className="text-sm font-medium text-gray-600 mb-2">Trading Signal</h3>
            <SignalBadge signal={timingData.trading_signal} className="text-lg px-4 py-2" />
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-600 mb-2">TTS Score</h3>
            <div className="text-3xl font-bold text-gray-900">
              {formatNumber(timingData.tts_score, 2)}
            </div>
            <div className="text-sm text-gray-500 mt-1">
              Confidence: {formatNumber(timingData.confidence_score * 100, 0)}%
            </div>
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-600 mb-2">Risk Level</h3>
            <RiskBadge riskLevel={timingData.risk_level} className="text-lg px-4 py-2" />
          </div>
        </div>
      </div>

      {/* Historical Chart */}
      <div className="bg-white rounded-lg shadow p-6 mb-6">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-bold text-gray-900">TTS Score History</h2>
          <div className="flex gap-2">
            {[7, 30, 90].map((days) => (
              <button
                key={days}
                onClick={() => setHistoryDays(days)}
                className={`px-3 py-1 rounded-lg text-sm font-medium transition-colors ${
                  historyDays === days
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                {days}d
              </button>
            ))}
          </div>
        </div>
        {historyData.length > 0 ? (
          <HistoryChart data={historyData} />
        ) : (
          <div className="text-center py-12 text-gray-500">
            No historical data available for this period
          </div>
        )}
      </div>

      {/* Trend Analysis */}
      <div className="bg-white rounded-lg shadow p-6 mb-6">
        <h2 className="text-xl font-bold text-gray-900 mb-4">Trend Analysis</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-sm font-medium text-gray-600 mb-2">Short Term (1-5 days)</h3>
            <div className="text-2xl">
              {getTrendEmoji(timingData.short_term_trend)}
            </div>
            <div className="text-lg font-semibold text-gray-900 mt-2">
              {timingData.short_term_trend}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-sm font-medium text-gray-600 mb-2">Medium Term (1-4 weeks)</h3>
            <div className="text-2xl">
              {getTrendEmoji(timingData.medium_term_trend)}
            </div>
            <div className="text-lg font-semibold text-gray-900 mt-2">
              {timingData.medium_term_trend}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-sm font-medium text-gray-600 mb-2">Long Term (1-3 months)</h3>
            <div className="text-2xl">
              {getTrendEmoji(timingData.long_term_trend)}
            </div>
            <div className="text-lg font-semibold text-gray-900 mt-2">
              {timingData.long_term_trend}
            </div>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-sm font-medium text-gray-600 mb-2">Trend Strength</h3>
            <div className="text-2xl font-bold text-gray-900">
              {formatNumber(timingData.trend_strength, 2)}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-sm font-medium text-gray-600 mb-2">Trend Consistency</h3>
            <div className="text-2xl font-bold text-gray-900">
              {formatNumber(timingData.trend_consistency, 2)}
            </div>
          </div>
        </div>
      </div>

      {/* Technical Indicators */}
      <div className="bg-white rounded-lg shadow p-6 mb-6">
        <h2 className="text-xl font-bold text-gray-900 mb-4">Technical Indicators</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-xs font-medium text-gray-600 mb-1">RSI</h3>
            <div className="text-lg font-semibold text-gray-900">
              {formatNumber(timingData.rsi_score, 2)}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-xs font-medium text-gray-600 mb-1">MACD</h3>
            <div className="text-lg font-semibold text-gray-900">
              {formatNumber(timingData.macd_score, 2)}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-xs font-medium text-gray-600 mb-1">Bollinger</h3>
            <div className="text-lg font-semibold text-gray-900">
              {formatNumber(timingData.bollinger_score, 2)}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-xs font-medium text-gray-600 mb-1">Moving Avg</h3>
            <div className="text-lg font-semibold text-gray-900">
              {formatNumber(timingData.ma_score, 2)}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-xs font-medium text-gray-600 mb-1">Stochastic</h3>
            <div className="text-lg font-semibold text-gray-900">
              {formatNumber(timingData.stochastic_score, 2)}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-xs font-medium text-gray-600 mb-1">Williams %R</h3>
            <div className="text-lg font-semibold text-gray-900">
              {formatNumber(timingData.williams_score, 2)}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-xs font-medium text-gray-600 mb-1">ATR</h3>
            <div className="text-lg font-semibold text-gray-900">
              {formatNumber(timingData.atr_score, 2)}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-xs font-medium text-gray-600 mb-1">Volume</h3>
            <div className="text-lg font-semibold text-gray-900">
              {formatNumber(timingData.volume_score, 2)}
            </div>
          </div>
        </div>
      </div>

      {/* Support & Resistance */}
      <div className="bg-white rounded-lg shadow p-6 mb-6">
        <h2 className="text-xl font-bold text-gray-900 mb-4">Support & Resistance</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h3 className="text-sm font-medium text-gray-600 mb-3">Support</h3>
            <div className="text-2xl font-bold text-green-600 mb-2">
              ${formatNumber(timingData.support_level, 2)}
            </div>
            <div className="text-sm text-gray-600">
              Distance: {formatNumber(timingData.support_distance, 2)}%
            </div>
            <div className="text-sm text-gray-600">
              Strength: {formatNumber(timingData.support_strength, 2)}
            </div>
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-600 mb-3">Resistance</h3>
            <div className="text-2xl font-bold text-red-600 mb-2">
              ${formatNumber(timingData.resistance_level, 2)}
            </div>
            <div className="text-sm text-gray-600">
              Distance: {formatNumber(timingData.resistance_distance, 2)}%
            </div>
            <div className="text-sm text-gray-600">
              Strength: {formatNumber(timingData.resistance_strength, 2)}
            </div>
          </div>
        </div>
      </div>

      {/* Volume Analysis */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-bold text-gray-900 mb-4">Volume Analysis</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div>
            <h3 className="text-sm font-medium text-gray-600 mb-2">Current Volume</h3>
            <div className="text-2xl font-bold text-gray-900">
              {formatLargeNumber(timingData.current_volume)}
            </div>
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-600 mb-2">Avg Volume (20d)</h3>
            <div className="text-2xl font-bold text-gray-900">
              {formatLargeNumber(timingData.avg_volume)}
            </div>
          </div>
          <div>
            <h3 className="text-sm font-medium text-gray-600 mb-2">Volume Ratio</h3>
            <div className="text-2xl font-bold text-gray-900">
              {formatNumber(timingData.volume_ratio, 2)}x
            </div>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-sm font-medium text-gray-600 mb-2">Volume Trend</h3>
            <div className="text-lg font-semibold text-gray-900">
              {timingData.volume_trend}
            </div>
          </div>
          <div className="border border-gray-200 rounded-lg p-4">
            <h3 className="text-sm font-medium text-gray-600 mb-2">Volume-Price Relationship</h3>
            <div className="text-lg font-semibold text-gray-900">
              {timingData.vp_relationship}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

