# BuenoTea Website Setup Guide

## Prerequisites

- Node.js 18+ installed
- npm or yarn
- Supabase project with timing_history table and timing view
- FMP API key (already configured in root project)

## Quick Start

### 1. Configure Environment Variables

Copy the `.env.local` file that was created or create it manually:

```bash
cd crates/site
cp .env.example .env.local
```

Edit `.env.local` with your Supabase credentials (already copied from root `.env`):

```env
NEXT_PUBLIC_SUPABASE_URL=https://your-project.supabase.co
NEXT_PUBLIC_SUPABASE_ANON_KEY=your_anon_key_here
SUPABASE_SERVICE_KEY=your_service_role_key_here   # required for server-side API routes
```

### 2. Install Dependencies (Already Done)

```bash
npm install
```

### 3. Run Development Server

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the site.

### 4. Build for Production

```bash
npm run build
```

This creates a static export in the `out/` directory.

## Features Implemented

### Pages

- ✅ **Main Landing Page** (`/`)
  - Hero section with BuenoTea branding
  - Real-time statistics (total stocks, buy/sell/neutral counts, avg TTS score)
  - Features showcase
  - Responsive design

- ✅ **Timing List Page** (`/timing`)
  - Table showing all 500+ stocks with timing signals
  - Search by symbol
  - Filter by trading signal
  - Sort by symbol, TTS score, trading signal, or risk level
  - Click-through to detailed ticker pages

- ✅ **Ticker Detail Page** (`/timing/[symbol]`)
  - Current trading signal and TTS score (large display)
  - Risk assessment card
  - Interactive historical chart (7d, 30d, 90d toggles)
  - Technical indicators (RSI, MACD, Bollinger, MA, Stochastic, Williams, ATR, Volume)
  - Trend analysis (short/medium/long term)
  - Support & resistance levels
  - Volume analysis

### Components

- ✅ Layout components (Header, Footer)
- ✅ SignalBadge - Color-coded signal display with emoji
- ✅ RiskBadge - Color-coded risk level display
- ✅ HistoryChart - Interactive Recharts line chart with signal change markers

### API Routes

- ✅ `GET /api/timing/latest` - Latest timing snapshot for all tickers (server-side with service key)
- ✅ `GET /api/timing/latest/[symbol]` - Latest timing record for a specific symbol
- ✅ `GET /api/timing/history/[symbol]?days=90` - Historical TTS scores
- ✅ `GET /api/timing/changes?days=7` - Recent signal changes
- ✅ `GET /api/stats` - Aggregate statistics (cached for 5 minutes)

### Database Functions Used

The site uses these Supabase functions (already created in migrations):

- `timing` view - Latest timing data for all stocks
- `timing_history` table - Historical time-series data
- `get_timing_history(symbol, days)` - Get historical data for a symbol
- `get_timing_signal_changes(days)` - Get recent signal changes

## File Structure

```
crates/site/
├── app/
│   ├── page.tsx                    # Main landing page
│   ├── layout.tsx                  # Root layout with Header/Footer
│   ├── globals.css                 # Global Tailwind styles
│   ├── timing/
│   │   ├── page.tsx                # Timing list page
│   │   └── [symbol]/
│   │       └── page.tsx            # Ticker detail page
│   └── api/
│       ├── timing/
│       │   └── history/
│       │       └── [symbol]/route.ts  # Historical data API
│       ├── changes/route.ts        # Signal changes API
│       └── stats/route.ts          # Statistics API
├── components/
│   ├── layout/
│   │   ├── Header.tsx              # Navigation bar
│   │   └── Footer.tsx              # Footer
│   ├── timing/
│   │   ├── SignalBadge.tsx         # Signal display
│   │   └── RiskBadge.tsx           # Risk level display
│   └── charts/
│       └── HistoryChart.tsx        # TTS history chart
├── lib/
│   ├── supabase.ts                 # Supabase client & helper functions
│   └── utils.ts                    # Utility functions (formatting, colors)
├── public/                         # Static assets
├── package.json                    # Dependencies
├── next.config.ts                  # Next.js config (static export)
├── tailwind.config.ts              # Tailwind CSS config
├── tsconfig.json                   # TypeScript config
├── .env.local                      # Environment variables (gitignored)
├── .env.example                    # Environment template
├── README.md                       # Project documentation
└── SETUP.md                        # This file
```

## Development Workflow

### Running Locally

1. Start the development server:
   ```bash
   npm run dev
   ```

2. The site will be available at:
   - Local: http://localhost:3000
   - Network: http://192.168.x.x:3000

3. Hot Module Replacement (HMR) is enabled - changes auto-reload

### Testing Different Pages

- Main page: http://localhost:3000/
- Timing list: http://localhost:3000/timing/
- Ticker detail: http://localhost:3000/timing/AAPL/

### Checking Data

The site expects:
- `timing` view with latest signals (queried via server API routes)
- `timing_history` table with historical data (queried via API routes)
- PostgreSQL functions (`get_timing_history`, `get_timing_signal_changes`)

## Building for Production

### Static Export

```bash
npm run build
```

Outputs to `out/` directory, ready for:
- AWS S3 + CloudFront
- Netlify
- Vercel
- Any static hosting

### Deployment to AWS

See `terraform/website/` (to be created) for infrastructure setup.

Basic deployment steps:
1. Build: `npm run build`
2. Upload to S3: `aws s3 sync out/ s3://your-bucket/`
3. Invalidate CloudFront: `aws cloudfront create-invalidation --distribution-id XXX --paths "/*"`

## Troubleshooting

### "No timing data found" / 401 errors

- Ensure you've run timing batch analysis: `cargo run --example timing_batch_to_supabase`
- Check that `timing` view exists in Supabase
- Verify environment variables are correct
- Confirm that `SUPABASE_SERVICE_KEY` is set in `.env.local`. Without it, server-side API routes return 500 and the browser will surface 401 unauthorised responses.

### API routes returning 500

- Check that Supabase service key is set in `.env.local`
- Verify PostgreSQL functions exist (run migrations)
- Check Supabase logs for detailed errors

### Charts not loading

- Ensure historical data exists for the symbol
- Check browser console for API errors
- Verify `get_timing_history` function works in Supabase SQL editor

## Next Steps

### Immediate Enhancements

- [ ] Add loading skeletons instead of spinners
- [ ] Add error boundaries for better error handling
- [ ] Implement pagination for timing list (if > 1000 stocks)
- [ ] Add "Recent Changes" widget on homepage

### Future Features

- [ ] Authentication for advanced features
- [ ] Real-time updates via Supabase subscriptions
- [ ] Email alerts for signal changes
- [ ] Export data to CSV/PDF
- [ ] Compare multiple tickers side-by-side
- [ ] Add tabs for fundamentals/regime/sentiment studies
- [ ] Dark mode support
- [ ] Mobile app (React Native)

## Performance Notes

- API routes cache stats for 5 minutes
- Static pages are pre-rendered at build time
- Client-side data fetching for real-time updates
- Recharts used for performant interactive charts
- Tailwind CSS for optimized styling

## Support

For issues or questions:
1. Check Supabase dashboard for data integrity
2. Review browser console for client-side errors
3. Check Next.js development server logs
4. Verify all environment variables are set correctly

