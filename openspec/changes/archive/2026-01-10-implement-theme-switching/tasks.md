# Tasks: Implement macOS-style Theme Switching

- [x] **CSS & Script Foundation**
    - [x] Update `src/view.rs` to include the theme persistence script in the `<head>`.
    - [x] Re-implement the CSS using scoped selectors for `.dark` mode and integrated bokeh effects.
- [x] **Header Redesign**
    - [x] Update the header structure to match the macOS design.
    - [x] Implement the theme toggle button with dual icons (`light_mode`/`dark_mode`).
- [x] **Table & List Redesign**
    - [x] Refine the fixed-header and scrollable-body layout.
    - [x] Update `get_file_icon` and icon rendering logic to return color-coded capsules.
- [x] **Refinement**
    - [x] Ensure all colors and opacities match the provided dark/light designs.
    - [x] Verify that `localStorage` correctly persists the theme across reloads.
- [x] **Verification**
    - [x] [x] Verify theme toggle button works and icon switches correctly.
    - [x] [x] Verify background transitions smoothly.
    - [x] [x] Verify theme survives page refresh.
    - [x] [x] Verify scrollable tbody behavior.
