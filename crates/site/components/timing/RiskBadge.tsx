import { getRiskColor } from '@/lib/utils'

interface RiskBadgeProps {
  riskLevel: string
  className?: string
}

export default function RiskBadge({ riskLevel, className = '' }: RiskBadgeProps) {
  return (
    <span
      className={`inline-flex items-center px-3 py-1 rounded-full text-sm font-medium border ${getRiskColor(
        riskLevel
      )} ${className}`}
    >
      {riskLevel}
    </span>
  )
}

