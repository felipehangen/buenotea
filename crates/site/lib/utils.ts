import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function getSignalEmoji(signal: string): string {
  switch (signal) {
    case 'StrongBuy':
      return '🚀'
    case 'Buy':
      return '📈'
    case 'Neutral':
      return '➡️'
    case 'Sell':
      return '📉'
    case 'StrongSell':
      return '💥'
    default:
      return '❓'
  }
}

export function getSignalColor(signal: string): string {
  switch (signal) {
    case 'StrongBuy':
    case 'Buy':
      return 'text-blue-600 bg-blue-50 border-blue-200'
    case 'Neutral':
      return 'text-gray-600 bg-gray-50 border-gray-200'
    case 'Sell':
    case 'StrongSell':
      return 'text-red-600 bg-red-50 border-red-200'
    default:
      return 'text-gray-600 bg-gray-50 border-gray-200'
  }
}

export function getRiskColor(riskLevel: string): string {
  switch (riskLevel) {
    case 'Low':
      return 'text-green-600 bg-green-50 border-green-200'
    case 'Medium':
      return 'text-yellow-600 bg-yellow-50 border-yellow-200'
    case 'High':
      return 'text-orange-600 bg-orange-50 border-orange-200'
    case 'VeryHigh':
      return 'text-red-600 bg-red-50 border-red-200'
    default:
      return 'text-gray-600 bg-gray-50 border-gray-200'
  }
}

export function getTrendEmoji(trend: string): string {
  switch (trend) {
    case 'StrongBullish':
      return '📈📈'
    case 'Bullish':
      return '📈'
    case 'Neutral':
      return '➡️'
    case 'Bearish':
      return '📉'
    case 'StrongBearish':
      return '📉📉'
    default:
      return '❓'
  }
}

export function formatDate(dateString: string): string {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

export function formatNumber(num: number, decimals: number = 2): string {
  return num.toFixed(decimals)
}

export function formatLargeNumber(num: number): string {
  if (num >= 1_000_000_000) {
    return `${(num / 1_000_000_000).toFixed(2)}B`
  }
  if (num >= 1_000_000) {
    return `${(num / 1_000_000).toFixed(2)}M`
  }
  if (num >= 1_000) {
    return `${(num / 1_000).toFixed(2)}K`
  }
  return num.toString()
}

