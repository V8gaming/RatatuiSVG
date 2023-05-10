Port for tui to ratatui

# SVG Checklist
__NEEDS RATIO FIXING & RELATIVE PATHS__
- [x] Line
- [x] Rect
- [ ] Circle
- [ ] Ellipse
- [ ] __Path__
  - [x] M
  - [x] L
  - [X] H
  - [X] V
  - [ ] A, Sweep flag not working, large arc flag need testing
  - [ ] Q, needs more testing
  - [ ] C, needs more testing
  - [ ] S, needs complete testing
  - [ ] T, needs complete testing
- [ ] Image (use jpeg compression (DCT) to render 16 colors)
- [ ] Iframe (will be attemped last)
- [ ] Text
- [ ] SVG
- [ ] Polygon
- [ ] Polyline
- [ ] textPath
- [ ] __Transformations, needs testing__
  - [ ] Translate
  - [ ] Scale
  - [ ] Rotate
  - [ ] Skew
  - [ ] Matrix
- [ ] __Style__
  - [ ] Fill(bg) Color
  - [ ] Stroke
    - [x] (fg) Color
    - [ ] (fg) size
    - [ ] linecap
    - [ ] linejoin
