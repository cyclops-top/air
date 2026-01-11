# Design: macOS-style Theme Switching

## 1. Theme Architecture
The implementation will rely on a single CSS stylesheet where theme-specific styles are controlled by a `.dark` class on the `<html>` element.

### 1.1 CSS Variables & Selectors
- Use CSS variables for common properties if helpful, but primarily rely on scoped selectors:
  ```css
  .glass-container { /* Light mode styles */ }
  .dark .glass-container { /* Dark mode overrides */ }
  ```

## 2. Component Enhancements

### 2.1 Background (Bokeh)
- Fixed "bokeh" elements (`div`s with high `blur`) will provide the characteristic blurred color spots from the design.
- Gradient backgrounds will transition smoothly between themes.

### 2.2 Icons with Backgrounds
- Each file entry icon will be wrapped in a styled container with a background color that matches its type (e.g., blue for docs, green for logs).
- Background opacity will adjust based on the theme.

### 2.3 Localized Scrolling
- The `tbody` will be the only scrollable area, keeping the header and column labels fixed.
- Custom scrollbar styling will be applied to match the aesthetic.

## 3. Persistence Logic (JavaScript)
A minimal `<script>` tag in the `<head>` will:
1. Check `localStorage.getItem('theme')`.
2. Apply the `.dark` class to `document.documentElement` if preferred or if system preference matches (optional).
3. Provide a `toggleTheme()` function for the button.

## 4. Resource Management
- **Icons**: Continue using inlined SVGs for all Material Symbols to ensure offline availability.
- **Fonts**: Rely on system-native font stacks (e.g., `system-ui`) to avoid external font loading while maintaining a clean look.
