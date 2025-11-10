'use client'

import { useEffect, useState } from 'react'
import Link from 'next/link'
import { type TimingRecord } from '@/lib/supabase'
import SignalBadge from '@/components/timing/SignalBadge'
import RiskBadge from '@/components/timing/RiskBadge'
import { formatDate, formatNumber } from '@/lib/utils'

type SortField = 'symbol' | 'tts_score' | 'trading_signal' | 'risk_level'
type SortDirection = 'asc' | 'desc'

export default function TimingPage() {
  const [timingData, setTimingData] = useState<TimingRecord[]>([])
  const [filteredData, setFilteredData] = useState<TimingRecord[]>([])
  const [loading, setLoading] = useState(true)
  const [searchTerm, setSearchTerm] = useState('')
  const [filterSignal, setFilterSignal] = useState('all')
  const [sortField, setSortField] = useState<SortField>('symbol')
  const [sortDirection, setSortDirection] = useState<SortDirection>('asc')

  useEffect(() => {
    async function loadData() {
      try {
        const response = await fetch('/api/timing/latest', { cache: 'no-store' })
        if (!response.ok) {
          throw new Error(await response.text())
        }
        const data: TimingRecord[] = await response.json()
        setTimingData(data)
        setFilteredData(data)
      } catch (error) {
        console.error('Error loading timing data:', error)
      } finally {
        setLoading(false)
      }
    }
    loadData()
  }, [])

  useEffect(() => {
    let filtered = [...timingData]

    // Apply search filter
    if (searchTerm) {
      filtered = filtered.filter((item) =>
        item.symbol.toLowerCase().includes(searchTerm.toLowerCase())
      )
    }

    // Apply signal filter
    if (filterSignal !== 'all') {
      filtered = filtered.filter((item) => item.trading_signal === filterSignal)
    }

    // Apply sorting
    filtered.sort((a, b) => {
      let aVal: any = a[sortField]
      let bVal: any = b[sortField]

      if (sortField === 'tts_score') {
        aVal = parseFloat(aVal)
        bVal = parseFloat(bVal)
      }

      if (aVal < bVal) return sortDirection === 'asc' ? -1 : 1
      if (aVal > bVal) return sortDirection === 'asc' ? 1 : -1
      return 0
    })

    setFilteredData(filtered)
  }, [searchTerm, filterSignal, sortField, sortDirection, timingData])

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc')
    } else {
      setSortField(field)
      setSortDirection('asc')
    }
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Timing Analysis</h1>
        <p className="text-gray-600">
          Technical Trading Signals for {timingData.length} stocks
        </p>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-lg shadow p-6 mb-6">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label htmlFor="search" className="block text-sm font-medium text-gray-700 mb-2">
              Search Symbol
            </label>
            <input
              type="text"
              id="search"
              placeholder="e.g. AAPL, MSFT..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
          <div>
            <label htmlFor="filter" className="block text-sm font-medium text-gray-700 mb-2">
              Filter by Signal
            </label>
            <select
              id="filter"
              value={filterSignal}
              onChange={(e) => setFilterSignal(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="all">All Signals</option>
              <option value="StrongBuy">🚀 Strong Buy</option>
              <option value="Buy">📈 Buy</option>
              <option value="Neutral">➡️ Neutral</option>
              <option value="Sell">📉 Sell</option>
              <option value="StrongSell">💥 Strong Sell</option>
            </select>
          </div>
        </div>
        <div className="mt-4 text-sm text-gray-600">
          Showing {filteredData.length} of {timingData.length} stocks
        </div>
      </div>

      {/* Table */}
      {loading ? (
        <div className="bg-white rounded-lg shadow p-12 text-center">
          <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
          <p className="text-gray-600 mt-4">Loading timing data...</p>
        </div>
      ) : (
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th
                    scope="col"
                    className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
                    onClick={() => handleSort('symbol')}
                  >
                    Symbol {sortField === 'symbol' && (sortDirection === 'asc' ? '↑' : '↓')}
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
                    onClick={() => handleSort('trading_signal')}
                  >
                    Signal {sortField === 'trading_signal' && (sortDirection === 'asc' ? '↑' : '↓')}
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
                    onClick={() => handleSort('tts_score')}
                  >
                    TTS Score {sortField === 'tts_score' && (sortDirection === 'asc' ? '↑' : '↓')}
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
                    onClick={() => handleSort('risk_level')}
                  >
                    Risk {sortField === 'risk_level' && (sortDirection === 'asc' ? '↑' : '↓')}
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                  >
                    Updated
                  </th>
                  <th scope="col" className="relative px-6 py-3">
                    <span className="sr-only">View</span>
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {filteredData.map((item) => (
                  <tr key={item.symbol} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm font-medium text-gray-900">{item.symbol}</div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <SignalBadge signal={item.trading_signal} />
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">{formatNumber(item.tts_score, 2)}</div>
                      <div className="text-xs text-gray-500">
                        Confidence: {formatNumber(item.confidence_score * 100, 0)}%
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <RiskBadge riskLevel={item.risk_level} />
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {formatDate(item.created_at)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      <Link
                        href={`/timing/${item.symbol}`}
                        className="text-blue-600 hover:text-blue-900"
                      >
                        Details →
                      </Link>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>

          {filteredData.length === 0 && (
            <div className="text-center py-12 text-gray-500">
              No stocks match your search criteria
            </div>
          )}
        </div>
      )}
    </div>
  )
}

