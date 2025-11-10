# BuenoTea Trading Website

A Next.js website displaying timing analysis results with historical signal tracking and charts.

## Getting Started

### Prerequisites

- Node.js 18+ 
- npm or yarn
- Supabase account with timing_history table

### Installation

1. Install dependencies:
```bash
cd crates/site
npm install
```

2. Configure environment variables:
```bash
cp .env.example .env.local
```

Edit `.env.local` with your Supabase credentials:
- `NEXT_PUBLIC_SUPABASE_URL` - Your Supabase project URL
- `NEXT_PUBLIC_SUPABASE_ANON_KEY` - Your Supabase anon/public key (exposed to the browser)
- `SUPABASE_SERVICE_KEY` - **Required.** Supabase service role key used only by server-side API routes. This key must remain private and should never be embedded in client bundles.

### Development

Run the development server:
```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the site.

### Build

Create a production build:
```bash
npm run build
```

This generates a static export in the `out/` directory, ready for deployment to S3/CloudFront.

## Features

- **Main Page**: Landing page with quick stats
- **Timing List** (`/timing`): View all 501 stocks with trading signals
- **Ticker Details** (`/timing/[symbol]`): Detailed analysis with historical charts

## Architecture

- **Frontend**: Next.js 14+ with App Router
- **Styling**: Tailwind CSS
- **Charts**: Recharts
- **Database**: Supabase (PostgreSQL)
- **Deployment**: Static export to S3 + CloudFront

## API Routes

- `GET /api/timing/history/[symbol]?days=90` - Historical TTS scores
- `GET /api/timing/changes?days=7` - Recent signal changes
- `GET /api/stats` - Aggregate statistics

## Project Structure

```
crates/site/
├── app/
│   ├── page.tsx              # Main landing page
│   ├── layout.tsx            # Root layout
│   ├── timing/
│   │   ├── page.tsx          # Timing list page
│   │   └── [symbol]/
│   │       └── page.tsx      # Ticker detail page
│   └── api/                  # API routes
├── components/               # Reusable components
├── lib/                      # Utilities & Supabase client
└── public/                   # Static assets
```

## Deployment

See `terraform/website/` for AWS infrastructure setup.

```bash
# Build
npm run build

# Deploy to S3
aws s3 sync out/ s3://your-bucket-name/

# Invalidate CloudFront cache
aws cloudfront create-invalidation --distribution-id YOUR_ID --paths "/*"
```
