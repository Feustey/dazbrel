#!/bin/bash
# Script pour créer les images de galerie professionnelles pour l'Umbrel App Store
# Format requis: 1440x900px PNG de haute qualité

# Couleurs du thème Dazno
DARK_BG="#0f172a"
CARD_BG="#1e293b"
ACCENT_BLUE="#2563eb"
ACCENT_GOLD="#fbbf24"
TEXT_WHITE="#e2e8f0"
TEXT_GRAY="#94a3b8"
SUCCESS_GREEN="#10b981"
WARNING_ORANGE="#f59e0b"

echo "🎨 Création des images de galerie pour l'Umbrel App Store..."

# Image 1 - Dashboard Principal avec métriques ROI
magick -size 1440x900 xc:"$DARK_BG" \
  -fill "$CARD_BG" -draw 'roundrectangle 60,60 1380,840 20,20' \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 80,80 1360,160 15,15' \
  -fill "$TEXT_WHITE" -pointsize 36 -font Arial-Bold -annotate +100+130 '⚡ Dazno Lightning ROI Optimizer' \
  -fill "$TEXT_GRAY" -pointsize 18 -annotate +100+160 'Maximize your Lightning Network profits with AI-powered optimization' \
  \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 100,200 460,380 15,15' \
  -fill "$TEXT_WHITE" -pointsize 24 -font Arial-Bold -annotate +120+240 'Monthly ROI' \
  -fill "$TEXT_WHITE" -pointsize 48 -font Arial-Bold -annotate +120+290 '+24.7%' \
  -fill "$ACCENT_GOLD" -pointsize 16 -annotate +120+320 '↗ +5.2% vs last month' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +120+350 '147,832 sats earned' \
  \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 480,200 840,380 15,15' \
  -fill "$TEXT_WHITE" -pointsize 24 -font Arial-Bold -annotate +500+240 'Active Channels' \
  -fill "$TEXT_WHITE" -pointsize 48 -font Arial-Bold -annotate +500+290 '23' \
  -fill "$SUCCESS_GREEN" -pointsize 16 -annotate +500+320 '✓ 21 profitable channels' \
  -fill "$WARNING_ORANGE" -pointsize 14 -annotate +500+350 '⚠ 2 channels need attention' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 860,200 1340,380 15,15' \
  -fill "$TEXT_WHITE" -pointsize 24 -font Arial-Bold -annotate +880+240 'Network Position' \
  -fill "$TEXT_WHITE" -pointsize 48 -font Arial-Bold -annotate +880+290 '#127' \
  -fill "$ACCENT_GOLD" -pointsize 16 -annotate +880+320 '🏆 Top 5% profitability' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +880+350 'vs 50,000+ nodes' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,400 1340,580 15,15' \
  -fill "$TEXT_WHITE" -pointsize 20 -font Arial-Bold -annotate +120+440 '📊 Real-time Analytics Dashboard' \
  -fill "$ACCENT_BLUE" -draw 'rectangle 140,460 400,480' \
  -fill "$SUCCESS_GREEN" -draw 'rectangle 140,490 650,510' \
  -fill "$ACCENT_GOLD" -draw 'rectangle 140,520 300,540' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +420+475 'Routing Revenue: 45,892 sats' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +670+505 'Channel Liquidity: 87% optimal' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +320+535 'Fee Efficiency: 94%' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,600 1340,780 15,15' \
  -fill "$TEXT_WHITE" -pointsize 20 -font Arial-Bold -annotate +120+640 '🤖 AI Recommendations (Last 24h)' \
  -fill "$SUCCESS_GREEN" -pointsize 14 -annotate +140+670 '✓ Increased fee on channel CH_789: +12% revenue' \
  -fill "$ACCENT_BLUE" -pointsize 14 -annotate +140+695 '📈 Open channel to ACINQ: Potential +890 sats/day' \
  -fill "$WARNING_ORANGE" -pointsize 14 -annotate +140+720 '⚡ Rebalance channel CH_456: Low outbound liquidity' \
  -fill "$ACCENT_GOLD" -pointsize 14 -annotate +140+745 '🎯 Optimize routing: 3 high-profit paths identified' \
  \
  gallery/dashboard-main.png

# Image 2 - Analytics ROI détaillées
magick -size 1440x900 xc:"$DARK_BG" \
  -fill "$CARD_BG" -draw 'roundrectangle 60,60 1380,840 20,20' \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 80,80 1360,140 15,15' \
  -fill "$TEXT_WHITE" -pointsize 32 -font Arial-Bold -annotate +100+120 '📈 ROI Analytics & Performance Tracking' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,160 700,450 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+190 'Revenue Trend (30 days)' \
  -fill "$SUCCESS_GREEN" -draw 'polyline 140,400 180,350 220,320 260,340 300,310 340,280 380,270 420,240 460,220 500,200 540,190 580,170 620,160 660,150' \
  -fill "$ACCENT_GOLD" -pointsize 12 -annotate +580+180 '↗ +247% growth' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +140+430 'Jan 1' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +620+430 'Jan 30' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 720,160 1340,450 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +740+190 'Channel Performance Matrix' \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 750,210 820,250 8,8' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +760+235 'CH_123' \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 830,210 900,250 8,8' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +840+235 'CH_456' \
  -fill "$WARNING_ORANGE" -draw 'roundrectangle 910,210 980,250 8,8' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +920+235 'CH_789' \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 990,210 1060,250 8,8' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +1000+235 'CH_012' \
  \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +750+280 'High Performance: 18 channels' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +750+300 'Medium Performance: 4 channels' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +750+320 'Need Optimization: 1 channel' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,470 1340,780 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+500 '🎯 Competitive Analysis vs Top Nodes' \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 140,520 440,680 10,10' \
  -fill "$TEXT_WHITE" -pointsize 16 -font Arial-Bold -annotate +160+545 'Your Node' \
  -fill "$SUCCESS_GREEN" -pointsize 14 -annotate +160+570 'ROI: 24.7%/month' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +160+590 'Channels: 23' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +160+610 'Capacity: 2.5 BTC' \
  -fill "$ACCENT_GOLD" -pointsize 12 -annotate +160+630 'Ranking: #127' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +160+650 '✓ Above average' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 460,520 760,680 10,10' \
  -fill "$TEXT_WHITE" -pointsize 16 -font Arial-Bold -annotate +480+545 'Amboss Magma' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +480+570 'ROI: 18.3%/month' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +480+590 'Channels: 47' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +480+610 'Capacity: 8.2 BTC' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +480+630 'Ranking: #23' \
  -fill "$ACCENT_GOLD" -pointsize 12 -annotate +480+650 '🏆 You beat them!' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 780,520 1080,680 10,10' \
  -fill "$TEXT_WHITE" -pointsize 16 -font Arial-Bold -annotate +800+545 'Network Average' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +800+570 'ROI: 8.1%/month' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +800+590 'Channels: 12' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +800+610 'Capacity: 0.8 BTC' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +800+630 'Ranking: ~25,000' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +800+650 '↗ +205% better' \
  \
  gallery/roi-analytics.png

# Image 3 - Channel Management Interface
magick -size 1440x900 xc:"$DARK_BG" \
  -fill "$CARD_BG" -draw 'roundrectangle 60,60 1380,840 20,20' \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 80,80 1360,140 15,15' \
  -fill "$TEXT_WHITE" -pointsize 32 -font Arial-Bold -annotate +100+120 '⚡ Smart Channel Management' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,160 1340,320 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+190 '🤖 AI-Powered Recommendations' \
  \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 140,210 1300,250 8,8' \
  -fill "$TEXT_WHITE" -pointsize 14 -annotate +160+235 '✓ HIGH PRIORITY: Open channel to 03a1b2c3... (ACINQ) - Estimated +890 sats/day revenue' \
  \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 140,260 1300,300 8,8' \
  -fill "$TEXT_WHITE" -pointsize 14 -annotate +160+285 '📊 MEDIUM: Increase fee rate on CH_789abc to 500ppm - Expected +340 sats/day' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,340 1340,600 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+370 '📋 Channel Overview (Active: 23 channels)' \
  \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 140,390 1300,430 5,5' \
  -fill "$TEXT_WHITE" -pointsize 12 -font Arial-Bold -annotate +160+410 'CH_123abc → 02d1e2f3...BLOCKSTREAM  |  2.1M sats  |  Fee: 1000ppm  |  Status: ✓ OPTIMAL' \
  -fill "$ACCENT_GOLD" -pointsize 11 -annotate +160+425 'Revenue (24h): 2,340 sats | Liquidity: 78% outbound | Performance: 97%' \
  \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 140,440 1300,480 5,5' \
  -fill "$TEXT_WHITE" -pointsize 12 -font Arial-Bold -annotate +160+460 'CH_456def → 03x4y5z6...KRAKEN       |  1.8M sats  |  Fee: 750ppm   |  Status: ✓ GOOD' \
  -fill "$TEXT_GRAY" -pointsize 11 -annotate +160+475 'Revenue (24h): 1,890 sats | Liquidity: 65% outbound | Performance: 89%' \
  \
  -fill "$WARNING_ORANGE" -draw 'roundrectangle 140,490 1300,530 5,5' \
  -fill "$TEXT_WHITE" -pointsize 12 -font Arial-Bold -annotate +160+510 'CH_789ghi → 04m9n8o7...BITFINEX     |  3.2M sats  |  Fee: 200ppm   |  Status: ⚠ NEEDS ATTENTION' \
  -fill "$WARNING_ORANGE" -pointsize 11 -annotate +160+525 'Revenue (24h): 234 sats | Liquidity: 12% outbound | Performance: 34% | 🔧 Action required' \
  \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 140,540 1300,580 5,5' \
  -fill "$TEXT_WHITE" -pointsize 12 -font Arial-Bold -annotate +160+560 'CH_012jkl → 05r6s7t8...LNMARKETS    |  950K sats   |  Fee: 1200ppm  |  Status: ✓ EXCELLENT' \
  -fill "$SUCCESS_GREEN" -pointsize 11 -annotate +160+575 'Revenue (24h): 1,567 sats | Liquidity: 85% balanced | Performance: 98%' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,620 1340,780 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+650 '🎯 Quick Actions' \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 140,670 280,710 8,8' \
  -fill "$TEXT_WHITE" -pointsize 14 -font Arial-Bold -annotate +160+695 'Open Channel' \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 300,670 440,710 8,8' \
  -fill "$TEXT_WHITE" -pointsize 14 -font Arial-Bold -annotate +320+695 'Rebalance' \
  -fill "$WARNING_ORANGE" -draw 'roundrectangle 460,670 600,710 8,8' \
  -fill "$TEXT_WHITE" -pointsize 14 -font Arial-Bold -annotate +480+695 'Close Channel' \
  -fill "$ACCENT_GOLD" -draw 'roundrectangle 620,670 760,710 8,8' \
  -fill "$TEXT_WHITE" -pointsize 14 -font Arial-Bold -annotate +640+695 'Update Fees' \
  \
  gallery/channel-management.png

# Image 4 - Competitive Analysis
magick -size 1440x900 xc:"$DARK_BG" \
  -fill "$CARD_BG" -draw 'roundrectangle 60,60 1380,840 20,20' \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 80,80 1360,140 15,15' \
  -fill "$TEXT_WHITE" -pointsize 32 -font Arial-Bold -annotate +100+120 '🏆 Competitive Analysis & Benchmarking' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,160 450,400 15,15' \
  -fill "$ACCENT_GOLD" -draw 'roundrectangle 120,180 430,220 10,10' \
  -fill "$TEXT_WHITE" -pointsize 16 -font Arial-Bold -annotate +140+205 '👑 YOUR NODE PERFORMANCE' \
  -fill "$SUCCESS_GREEN" -pointsize 24 -font Arial-Bold -annotate +140+250 'ROI: 24.7%' \
  -fill "$TEXT_WHITE" -pointsize 14 -annotate +140+275 'Monthly Revenue: 147,832 sats' \
  -fill "$TEXT_WHITE" -pointsize 14 -annotate +140+295 'Channels: 23 active' \
  -fill "$TEXT_WHITE" -pointsize 14 -annotate +140+315 'Capacity: 2.5 BTC' \
  -fill "$ACCENT_GOLD" -pointsize 16 -font Arial-Bold -annotate +140+345 '🎯 Network Rank: #127' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +140+365 '📊 Top 5% profitability' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +140+380 '⚡ Above 87% of nodes' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 470,160 820,400 15,15' \
  -fill "$TEXT_WHITE" -pointsize 16 -font Arial-Bold -annotate +490+185 'VS AMBOSS MAGMA' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +490+210 'ROI: 18.3% (-6.4% vs you)' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +490+235 'Revenue: ~89,200 sats' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +490+260 'Channels: 47 active' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +490+285 'Capacity: 8.2 BTC' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +490+310 'Network Rank: #23' \
  -fill "$SUCCESS_GREEN" -pointsize 14 -font Arial-Bold -annotate +490+340 '🏆 YOU WIN ON ROI!' \
  -fill "$ACCENT_GOLD" -pointsize 12 -annotate +490+360 '↗ +35% better efficiency' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +490+380 'Despite lower capacity' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 840,160 1320,400 15,15' \
  -fill "$TEXT_WHITE" -pointsize 16 -font Arial-Bold -annotate +860+185 'VS NETWORK AVERAGE' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +860+210 'ROI: 8.1% (-16.6% vs you)' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +860+235 'Revenue: ~23,400 sats' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +860+260 'Channels: 12 average' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +860+285 'Capacity: 0.8 BTC' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +860+310 'Rank: ~25,000' \
  -fill "$SUCCESS_GREEN" -pointsize 14 -font Arial-Bold -annotate +860+340 '🚀 +205% BETTER!' \
  -fill "$ACCENT_GOLD" -pointsize 12 -annotate +860+360 'Elite performance tier' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +860+380 'Top 1% efficiency' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,420 1320,600 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+450 '📈 Performance Metrics Comparison' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +140+480 'Metric' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +400+480 'Your Node' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +600+480 'Amboss Magma' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +800+480 'Top 10 Average' \
  -fill "$TEXT_GRAY" -pointsize 14 -annotate +1000+480 'Network Avg' \
  \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +140+510 'Fee Efficiency' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -font Arial-Bold -annotate +400+510 '94%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +600+510 '76%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +800+510 '82%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +1000+510 '34%' \
  \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +140+535 'Routing Success' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -font Arial-Bold -annotate +400+535 '97.8%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +600+535 '94.2%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +800+535 '89.5%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +1000+535 '67.3%' \
  \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +140+560 'Liquidity Balance' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -font Arial-Bold -annotate +400+560 '78%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +600+560 '82%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +800+560 '71%' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +1000+560 '45%' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,620 1320,780 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+650 '🎯 Optimization Opportunities' \
  -fill "$ACCENT_GOLD" -pointsize 14 -annotate +140+680 '🔸 Increase capacity by 40% → Potential rank #45 (Est. +67% revenue)' \
  -fill "$ACCENT_BLUE" -pointsize 14 -annotate +140+705 '🔸 Open 5 strategic channels → Match Amboss liquidity distribution' \
  -fill "$SUCCESS_GREEN" -pointsize 14 -annotate +140+730 '🔸 Maintain current efficiency → Stay in top 1% performer tier' \
  -fill "$WARNING_ORANGE" -pointsize 14 -annotate +140+755 '🔸 Monitor market conditions → Adapt fee strategy for max profit' \
  \
  gallery/competitive-analysis.png

# Image 5 - Automation Settings
magick -size 1440x900 xc:"$DARK_BG" \
  -fill "$CARD_BG" -draw 'roundrectangle 60,60 1380,840 20,20' \
  -fill "$ACCENT_BLUE" -draw 'roundrectangle 80,80 1360,140 15,15' \
  -fill "$TEXT_WHITE" -pointsize 32 -font Arial-Bold -annotate +100+120 '🤖 AI Automation & Smart Controls' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,160 680,380 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+190 '⚙️ Automation Settings' \
  \
  -fill "$SUCCESS_GREEN" -draw 'circle 140,220 150,220' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +170+225 '✓ Auto Fee Optimization (ENABLED)' \
  -fill "$TEXT_GRAY" -pointsize 11 -annotate +170+240 'Adjusts fees every 6 hours based on demand' \
  \
  -fill "$SUCCESS_GREEN" -draw 'circle 140,260 150,260' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +170+265 '✓ Smart Rebalancing (ENABLED)' \
  -fill "$TEXT_GRAY" -pointsize 11 -annotate +170+280 'Auto-rebalance channels when liquidity < 20%' \
  \
  -fill "$ACCENT_BLUE" -draw 'circle 140,300 150,300' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +170+305 '○ Channel Auto-Opening (SELECTIVE)' \
  -fill "$TEXT_GRAY" -pointsize 11 -annotate +170+320 'Only for opportunities > 500 sats/day ROI' \
  \
  -fill "$TEXT_GRAY" -draw 'circle 140,340 150,340' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +170+345 '○ Auto Channel Closing (DISABLED)' \
  -fill "$TEXT_GRAY" -pointsize 11 -annotate +170+360 'Manual approval required for safety' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 700,160 1320,380 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +720+190 '🧠 ML Engine Status' \
  \
  -fill "$SUCCESS_GREEN" -draw 'roundrectangle 730,210 1300,250 8,8' \
  -fill "$TEXT_WHITE" -pointsize 14 -font Arial-Bold -annotate +750+235 '🟢 ONLINE - Processing 1,247 network events/min' \
  \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +730+270 'Model Version: v2.4.1 (Updated: 2 days ago)' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +730+290 'Accuracy Score: 94.7% prediction success' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +730+310 'Training Data: 847,392 routing events' \
  -fill "$ACCENT_GOLD" -pointsize 12 -annotate +730+330 '🎯 Next training: In 12 hours' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +730+350 '✓ All systems optimal' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,400 1320,580 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+430 '📊 Recent Automation Results (Last 7 days)' \
  \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +140+460 '✅ 47 fee adjustments executed → +12,394 sats additional revenue' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +140+480 '✅ 8 channels rebalanced → Restored optimal liquidity' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +140+500 '✅ 2 new channels opened → ACINQ, LNMARKETS (+1,890 sats/day)' \
  -fill "$ACCENT_BLUE" -pointsize 12 -annotate +140+520 '📋 3 manual approvals pending → High-value channel opportunities' \
  -fill "$WARNING_ORANGE" -pointsize 12 -annotate +140+540 '⚠️ 1 rebalancing failed → Insufficient liquidity on peer side' \
  -fill "$ACCENT_GOLD" -pointsize 12 -annotate +140+560 '💡 15 optimization suggestions generated → Review in dashboard' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 100,600 680,780 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +120+630 '🔧 Risk Controls' \
  \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +140+660 'Max Daily Spend: 50,000 sats' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +140+680 'Max Channel Size: 5,000,000 sats' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +140+700 'Min ROI Threshold: 300 sats/day' \
  -fill "$TEXT_WHITE" -pointsize 12 -annotate +140+720 'Auto-approval Limit: 1,000,000 sats' \
  -fill "$SUCCESS_GREEN" -pointsize 12 -annotate +140+740 '✓ Emergency stop: Available' \
  -fill "$ACCENT_GOLD" -pointsize 12 -annotate +140+760 '🔒 All limits respected' \
  \
  -fill "$CARD_BG" -draw 'roundrectangle 700,600 1320,780 15,15' \
  -fill "$TEXT_WHITE" -pointsize 18 -font Arial-Bold -annotate +720+630 '📈 Performance Impact' \
  \
  -fill "$SUCCESS_GREEN" -pointsize 16 -font Arial-Bold -annotate +740+665 '+31.2% Revenue Increase' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +740+685 'Since automation enabled (30 days ago)' \
  \
  -fill "$ACCENT_BLUE" -pointsize 14 -font Arial-Bold -annotate +740+710 '89% Time Saved' \
  -fill "$TEXT_GRAY" -pointsize 12 -annotate +740+730 'vs manual channel management' \
  \
  -fill "$ACCENT_GOLD" -pointsize 14 -font Arial-Bold -annotate +740+755 '97.8% Uptime' \
  -fill "$TEXT_GRAY" -pointsize 11 -annotate +740+770 'Automation system reliability' \
  \
  gallery/automation-settings.png

echo "✅ 5 images de galerie créées avec succès!"
echo "📁 Fichiers générés:"
echo "   - gallery/dashboard-main.png (1440x900)"
echo "   - gallery/roi-analytics.png (1440x900)"
echo "   - gallery/channel-management.png (1440x900)"
echo "   - gallery/competitive-analysis.png (1440x900)"
echo "   - gallery/automation-settings.png (1440x900)"
echo ""
echo "🎨 Images prêtes pour l'Umbrel App Store!"
echo "📋 Format: PNG haute qualité 1440x900px"
echo "🚀 Optimisées pour présentation professionnelle"