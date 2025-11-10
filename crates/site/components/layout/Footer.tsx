import { formatDate } from '@/lib/utils'

export default function Footer() {
  const currentYear = new Date().getFullYear()
  
  return (
    <footer className="bg-gray-50 border-t border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div>
            <h3 className="text-sm font-semibold text-gray-900 mb-4">BuenoTea</h3>
            <p className="text-sm text-gray-600">
              Technical Trading Score analysis for S&P 500 stocks
            </p>
          </div>
          <div>
            <h3 className="text-sm font-semibold text-gray-900 mb-4">Quick Links</h3>
            <ul className="space-y-2">
              <li>
                <a href="/" className="text-sm text-gray-600 hover:text-blue-600">
                  Home
                </a>
              </li>
              <li>
                <a href="/timing" className="text-sm text-gray-600 hover:text-blue-600">
                  Timing Analysis
                </a>
              </li>
            </ul>
          </div>
          <div>
            <h3 className="text-sm font-semibold text-gray-900 mb-4">About</h3>
            <p className="text-sm text-gray-600">
              Data updated daily. Historical tracking available for all signals.
            </p>
          </div>
        </div>
        <div className="mt-8 pt-8 border-t border-gray-200">
          <p className="text-sm text-gray-500 text-center">
            © {currentYear} BuenoTea. Built with Next.js and Supabase.
          </p>
        </div>
      </div>
    </footer>
  )
}

