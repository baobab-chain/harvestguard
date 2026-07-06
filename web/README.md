# HarvestGuard — Landing Page

A static landing page explaining the protocol. No build step required.

## Run locally

```bash
cd web
python3 -m http.server 8084
```

Or open `index.html` directly in a browser.

## Design notes

- The hero visual is a rainfall gauge whose level drops and crosses a
  dashed threshold line, firing a "PAYOUT" ring — depicting the actual
  parametric trigger mechanism, distinct from the animations used across
  the other four Baobab Labs repos (rotating ring, stamp draw, code
  reveal, scan sweep) so all five read as their own products.
- Terracotta is the primary action color here, matching diaspora-circle
  — the fifth repo cycling back to reuse a prior accent, since by now
  each repo's hero visual itself is different enough to stay
  distinguishable without needing a sixth unique color.
- No frameworks or build tooling — deploy directly via GitHub Pages, Netlify, or any static host.
