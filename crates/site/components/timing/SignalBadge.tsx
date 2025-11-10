import { getSignalEmoji, getSignalColor } from '@/lib/utils'

interface SignalBadgeProps {
  signal: string
  className?: string
}

export default function SignalBadge({ signal, className = '' }: SignalBadgeProps) {
  return (
    <span
      className={`inline-flex items-center px-3 py-1 rounded-full text-sm font-medium border ${getSignalColor(
        signal
      )} ${className}`}
    >
      <span className="mr-2">{getSignalEmoji(signal)}</span>
      {signal}
    </span>
  )
}

