# Proposal: Implement macOS-style Theme Switching

## Problem Statement
The current UI is fixed to a single dark theme. Users need the ability to switch between light and dark modes to suit their preferences and environment. The current aesthetic also needs refinement to match the provided macOS-inspired glassmorphism designs.

## Proposed Solution
Implement a dual-theme system with a persistent toggle.
- **Visual Redesign**: Adopt a macOS-inspired glassmorphism aesthetic with bokeh background effects, refined rounded corners (`16px`), and color-coded file type icons.
- **Theme Toggle**: Add a dedicated button in the header to switch between light and dark modes.
- **Persistence**: Use a small JavaScript snippet to save the theme preference in `localStorage` and apply it immediately on page load to prevent flashing.
- **Self-Contained Styles**: Ensure all CSS is inlined and uses theme-aware selectors (e.g., `.dark .element`) to eliminate external dependencies.

## User Impact
- **Flexibility**: Users can choose the theme that works best for them.
- **Aesthetic Quality**: A more polished, premium feel inspired by macOS.
- **Usability**: Improved visual hierarchy and localized scrolling for a desktop-app-like experience.
