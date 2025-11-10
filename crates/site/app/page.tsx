'use client'

import { useEffect, useState } from 'react'
import Link from 'next/link'
import { getSignalEmoji } from '@/lib/utils'

export default function Home() {
  const [stats, setStats] = useState({
    total: 0,
    buyCount: 0,
    sellCount: 0,
    neutralCount: 0,
    avgScore: 0,
  })
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    async function loadStats() {
      try {
        const response = await fetch('/api/stats', { cache: 'no-store' })
        if (!response.ok) {
          throw new Error(await response.text())
        }
        const data = await response.json()
        setStats(data)
      } catch (error) {
        console.error('Error loading stats:', error)
      } finally {
        setLoading(false)
      }
    }
    loadStats()
  }, [])

  return (
    <div className="bg-gradient-to-b from-blue-50 to-white">
      {/* Hero Section */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
        <div className="text-center">
          <h1 className="text-5xl font-bold text-gray-900 mb-6">
            Welcome to <span className="text-blue-600">BuenoTea</span>
          </h1>
          <p className="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
            Advanced Technical Trading Score (TTS) analysis for S&P 500 stocks.
            Real-time signals backed by comprehensive technical indicators and historical tracking.
          </p>
          <div className="flex justify-center gap-4">
            <Link
              href="/timing"
              className="bg-blue-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-blue-700 transition-colors"
            >
              View Trading Signals
            </Link>
          </div>
        </div>
      </div>

      {/* Stats Section */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="bg-white rounded-lg shadow-lg p-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-6 text-center">
            Current Market Analysis
          </h2>
          
          {loading ? (
            <div className="text-center py-12">
              <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
              <p className="text-gray-600 mt-4">Loading statistics...</p>
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-6">
              <div className="bg-gray-50 rounded-lg p-6 text-center border border-gray-200">
                <div className="text-3xl font-bold text-gray-900">{stats.total}</div>
                <div className="text-sm text-gray-600 mt-2">Total Stocks</div>
              </div>
              
              <div className="bg-blue-50 rounded-lg p-6 text-center border border-blue-200">
                <div className="text-3xl font-bold text-blue-600">
                  {getSignalEmoji('Buy')} {stats.buyCount}
                </div>
                <div className="text-sm text-gray-600 mt-2">Buy Signals</div>
                <div className="text-xs text-gray-500 mt-1">
                  {((stats.buyCount / stats.total) * 100).toFixed(1)}%
                </div>
              </div>
              
              <div className="bg-gray-50 rounded-lg p-6 text-center border border-gray-200">
                <div className="text-3xl font-bold text-gray-600">
                  {getSignalEmoji('Neutral')} {stats.neutralCount}
                </div>
                <div className="text-sm text-gray-600 mt-2">Neutral</div>
                <div className="text-xs text-gray-500 mt-1">
                  {((stats.neutralCount / stats.total) * 100).toFixed(1)}%
                </div>
              </div>
              
              <div className="bg-red-50 rounded-lg p-6 text-center border border-red-200">
                <div className="text-3xl font-bold text-red-600">
                  {getSignalEmoji('Sell')} {stats.sellCount}
                </div>
                <div className="text-sm text-gray-600 mt-2">Sell Signals</div>
                <div className="text-xs text-gray-500 mt-1">
                  {((stats.sellCount / stats.total) * 100).toFixed(1)}%
                </div>
              </div>
              
              <div className="bg-green-50 rounded-lg p-6 text-center border border-green-200">
                <div className="text-3xl font-bold text-green-600">{stats.avgScore.toFixed(2)}</div>
                <div className="text-sm text-gray-600 mt-2">Avg TTS Score</div>
                <div className="text-xs text-gray-500 mt-1">
                  Range: -1.0 to +1.0
                </div>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Features Section */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 pb-20">
        <h2 className="text-2xl font-bold text-gray-900 mb-8 text-center">
          Features
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div className="bg-white rounded-lg shadow p-6">
            <div className="text-4xl mb-4">📊</div>
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              Real-Time Signals
            </h3>
            <p className="text-gray-600">
              Technical Trading Score (TTS) analysis for 500+ stocks, updated daily with Buy, Sell, and Neutral signals.
            </p>
          </div>
          
          <div className="bg-white rounded-lg shadow p-6">
            <div className="text-4xl mb-4">📈</div>
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              Historical Tracking
            </h3>
            <p className="text-gray-600">
              Track signal changes over time with interactive charts showing TTS score history and trend analysis.
            </p>
          </div>
          
          <div className="bg-white rounded-lg shadow p-6">
            <div className="text-4xl mb-4">🎯</div>
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              Detailed Analysis
            </h3>
            <p className="text-gray-600">
              Comprehensive technical indicators including RSI, MACD, Bollinger Bands, Moving Averages, and more.
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
