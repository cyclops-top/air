# Design: Cyber-Space Glassmorphism UI

## 1. Aesthetic Foundations
- **Theme**: Dark Mode (Background: `#030305`).
- **Color Palette**:
    - Primary: `#2563eb` (Blue)
    - Accent: `#00f7ff` (Cyan)
    - Success: `#10ff70` (Green)
- **Effects**:
    - `backdrop-blur` for cards (glassmorphism).
    - `glow` effects for text and active elements.
    - Subtle grid background pattern.

## 2. Technical Stack
- **CSS Framework**: Tailwind CSS (CDN-loaded for simplicity in this prototype, or integrated via build).
- **Fonts**:
    - `Space Grotesk`: Headings and labels.
    - `Space Mono`: File names, sizes, and timestamps.
- **Icons**: Material Symbols Outlined.

## 3. Component Updates

### 3.1 Header
- Logo with 'Air' icon and cyber-text styling.
- "System Online" status indicator.
- LAN IP display with 'sensors' icon and cyan glow.

### 3.2 Navigation (Breadcrumbs)
- Cyber-themed breadcrumbs with '/' separators.
- Home icon ('hub') for the root level.
- Hover effects and active state highlighting.

### 3.3 File Listing (Table)
- Table-based layout with clear columns: Filename, Size, Last Sync, Access.
- Rows with hover transitions and cyan glows for filenames.
- Type-specific icons (Folder, Description, Analytics, etc.).
- Right-aligned download buttons with hover scaling.

### 3.4 Layout Structure
- Centered content with maximum width (`7xl`).
- Glassmorphism container for the main table.
- Subtle background decorative elements (blurry circles).

## 4. Implementation Strategy
- Modify `src/view.rs` to generate the new HTML/CSS structure.
- Update `render_html` to include external resources (Tailwind, Fonts, Icons).
- Update `render_breadcrumbs` to use the new icon-based structure.
- Inject dynamic data (IP, Path, File List) into the template.
