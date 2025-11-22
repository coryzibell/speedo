# Speedo GUI Preview

## Visual Design

The Speedo GUI features a modern, dark-themed interface with smooth animations and an intuitive layout.

### Layout Overview

```
┌──────────────────────────────────────────────────┐
│                                                  │
│                    Speedo                        │
│            Network Speed Test Tool               │
│                                                  │
├──────────────────────────────────────────────────┤
│                                                  │
│  Select Server:                                  │
│  ┌────────────────────────────────────────────┐ │
│  │ Cloudflare (CDN) - Global           [✓]   │ │
│  │ Hetzner - Nuremberg, Germany              │ │
│  │ Hetzner - Ashburn, USA                    │ │
│  │ Vultr - Singapore                         │ │
│  │ ...                                       │ │
│  └────────────────────────────────────────────┘ │
│                                                  │
├──────────────────────────────────────────────────┤
│                                                  │
│         ┌───────────────────────┐                │
│         │  Run Speed Test       │                │
│         └───────────────────────┘                │
│                                                  │
├──────────────────────────────────────────────────┤
│                                                  │
│  Ready to test                                   │
│                                                  │
└──────────────────────────────────────────────────┘
```

### After Running Test

```
┌──────────────────────────────────────────────────┐
│                                                  │
│  Test complete: 245.67 Mbps (30.71 MB/s)        │
│                                                  │
├──────────────────────────────────────────────────┤
│                                                  │
│  Test Results                                    │
│                                                  │
│  Server:         Cloudflare (CDN)                │
│  Status:         200                             │
│  Downloaded:     95.37 MB                        │
│  Speed:          245.67 Mbps (30.71 MB/s)       │
│  Total Time:     3.11s                           │
│  Connect Time:   0.089s                          │
│  TTFB:           0.156s                          │
│                                                  │
└──────────────────────────────────────────────────┘
```

## Color Scheme

- **Background**: Dark gradient (navy to dark blue)
- **Primary**: Bright cyan (#64c8ff)
- **Secondary**: Purple gradient (#667eea to #764ba2)
- **Text**: Light gray (#e0e0e0)
- **Accents**: Various blues and purples
- **Borders**: Subtle rgba borders with transparency

## Interactive Elements

### Server List
- **Default state**: Semi-transparent dark background
- **Hover**: Slight brightening + slide animation
- **Selected**: Cyan border + glow effect
- **Scrollbar**: Custom styled in cyan theme

### Run Button
- **Default**: Purple gradient with shadow
- **Hover**: Lifts up with enhanced shadow
- **Active**: Pressed down effect
- **Disabled**: Reduced opacity, no interactions

### Results
- **Animation**: Fade in from bottom
- **Speed value**: Highlighted in cyan, larger font
- **Borders**: Subtle separators between metrics
- **Layout**: Two-column with labels and values

## Accessibility

- High contrast text on dark background
- Clear visual hierarchy
- Interactive elements have hover states
- Disabled states clearly indicated
- Smooth but purposeful animations

## Responsive Behavior

- Fixed 800x600 window size (optimal for content)
- Server list scrolls if many servers
- Results section expands to show all data
- Clean margins and padding throughout

## Typography

- **Headings**: Large, bold, cyan colored
- **Body**: Medium weight, light gray
- **Values**: Slightly larger for emphasis
- **Speed**: Extra large, cyan, bold

## Future Enhancements

Planned visual improvements:
- [ ] Animated progress bar during download
- [ ] Line chart for historical results
- [ ] Server health indicators (green/yellow/red dots)
- [ ] Dark/light theme toggle
- [ ] Customizable accent colors
- [ ] System tray icon with menubar
- [ ] Window resize with responsive layout
