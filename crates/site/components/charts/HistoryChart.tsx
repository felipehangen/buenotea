'use client'

import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  ReferenceLine,
  Scatter,
  Legend,
} from 'recharts'
import { formatDate } from '@/lib/utils'

interface HistoryChartProps {
  data: Array<{
    created_at: string
    tts_score: number
    trading_signal: string
  }>
}

export default function HistoryChart({ data }: HistoryChartProps) {
  // Transform data for chart
  const chartData = data.map((item) => ({
    date: new Date(item.created_at).getTime(),
    dateLabel: formatDate(item.created_at),
    score: parseFloat(item.tts_score.toFixed(2)),
    signal: item.trading_signal,
  }))

  // Identify signal change points
  const signalChanges = chartData.filter((item, index) => {
    if (index === 0) return false
    return item.signal !== chartData[index - 1].signal
  })

  return (
    <div className="w-full h-[400px]">
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={chartData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
          <XAxis
            dataKey="date"
            type="number"
            domain={['dataMin', 'dataMax']}
            tickFormatter={(timestamp) => {
              const date = new Date(timestamp)
              return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
            }}
            stroke="#6b7280"
          />
          <YAxis domain={[-1, 1]} stroke="#6b7280" />
          <Tooltip
            content={({ active, payload }) => {
              if (active && payload && payload.length) {
                const data = payload[0].payload
                return (
                  <div className="bg-white p-3 border border-gray-200 rounded-lg shadow-lg">
                    <p className="text-sm font-medium text-gray-900">{data.dateLabel}</p>
                    <p className="text-sm text-gray-600">
                      Score: <span className="font-semibold">{data.score}</span>
                    </p>
                    <p className="text-sm text-gray-600">
                      Signal: <span className="font-semibold">{data.signal}</span>
                    </p>
                  </div>
                )
              }
              return null
            }}
          />
          <Legend />
          <ReferenceLine y={0} stroke="#9ca3af" strokeDasharray="3 3" />
          <ReferenceLine y={0.2} stroke="#3b82f6" strokeDasharray="2 2" strokeOpacity={0.3} />
          <ReferenceLine y={-0.2} stroke="#ef4444" strokeDasharray="2 2" strokeOpacity={0.3} />
          <Line
            type="monotone"
            dataKey="score"
            stroke="#3b82f6"
            strokeWidth={2}
            dot={{ fill: '#3b82f6', r: 3 }}
            activeDot={{ r: 5 }}
            name="TTS Score"
          />
          {signalChanges.length > 0 && (
            <Scatter
              data={signalChanges}
              fill="#ef4444"
              shape="circle"
              r={6}
              name="Signal Change"
            />
          )}
        </LineChart>
      </ResponsiveContainer>
    </div>
  )
}

