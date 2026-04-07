# Design System Document: The Editorial Taskmaster

## 1. Overview & Creative North Star
**Creative North Star: "The Digital Curator"**

This design system transcends the utility of a standard to-do list, reimagining task management as an editorial experience. We reject the "spreadsheet-style" density found in traditional productivity tools. Instead, we embrace **Soft Minimalism**—a philosophy that uses breathing room as a functional tool to reduce cognitive load.

The system breaks the "template" look through:
*   **Intentional Asymmetry:** Aligning large display type against wide-margined, off-center task lists.
*   **Tonal Depth:** Replacing harsh lines with sophisticated, layered surface shifts.
*   **Typographic Gravity:** Using the contrast between the architectural *Manrope* and the functional *Inter* to guide the eye.

---

## 2. Colors: Tonal Sophistication
Our palette is a study in "Deep Blues and Atmosphere." We move away from stark whites and blacks toward a "Ink and Cloud" aesthetic.

### The "No-Line" Rule
**Borders are a failure of hierarchy.** In this system, 1px solid borders are strictly prohibited for sectioning. Boundaries must be defined solely through:
1.  **Background Color Shifts:** A `surface-container-low` task card sitting on a `surface` background.
2.  **Tonal Transitions:** Using `surface-container-highest` for sidebars against a `surface` main stage.

### Surface Hierarchy & Nesting
Treat the UI as a series of stacked, physical layers.
*   **The Foundation:** Use `surface` (#f7f9fc) for the main application background.
*   **The Content Layer:** Use `surface-container-lowest` (#ffffff) for active task cards to make them "pop" against the background without shadows.
*   **The Navigation Layer:** Use `surface-container-high` (#e6e8eb) for sidebars to provide a sense of architectural grounding.

### The "Glass & Gradient" Rule
To inject "soul" into the professional interface:
*   **Floating Elements:** Use `surface` with a 70% opacity and a `20px backdrop-blur` for modals and floating action menus.
*   **Signature Textures:** For primary CTAs and progress bars, apply a subtle linear gradient (135°) from `primary` (#24389c) to `primary_container` (#3f51b5). This prevents the UI from feeling "flat" or "cheap."

---

## 3. Typography: The Editorial Scale
We use two typefaces to create an "Architectural vs. Functional" dialogue.

*   **Display & Headlines (Manrope):** Our "Brand Voice." Use `display-lg` and `headline-md` for page titles and empty-state messaging. Manrope’s geometric clarity provides an authoritative, premium feel.
*   **Body & Labels (Inter):** Our "Workhorse." Used for task descriptions, metadata, and inputs. Inter is chosen for its exceptional legibility at small sizes (`body-sm`) and its neutral, "invisible" quality.

**Key Rule:** Maintain a strict 2:1 ratio for vertical rhythm. If a headline uses `headline-lg` (2rem), ensure the following body text uses `body-md` (0.875rem) with significant `40px` padding-top to create an editorial "header" feel.

---

## 4. Elevation & Depth
Depth in this system is achieved through **Tonal Layering**, not structural scaffolding.

*   **The Layering Principle:** Stack `surface-container` tiers. 
    *   *Example:* Place a `surface-container-lowest` (#ffffff) card inside a `surface-container-low` (#f2f4f7) parent container.
*   **Ambient Shadows:** For elevated elements (like a "Create Task" popover), use a "Cloud Shadow": 
    *   `box-shadow: 0 12px 40px rgba(25, 28, 30, 0.06);` 
    *   The shadow color is a low-opacity tint of `on_surface`, making it feel like natural light rather than a gray smudge.
*   **The "Ghost Border" Fallback:** If accessibility requirements demand a container edge, use `outline_variant` at **15% opacity**. Never use a 100% opaque border.

---

## 5. Components

### Cards & Task Lists
*   **Prohibition:** No divider lines between tasks. 
*   **The Solution:** Use `8px` vertical spacing between task items. Each task should reside on its own `surface-container-lowest` card with an `xl` (0.75rem) corner radius. On hover, transition the background to `surface-container-high`.

### Buttons
*   **Primary:** Indigo gradient (`primary` to `primary_container`). Radius: `full`. No shadow on rest; "Cloud Shadow" on hover.
*   **Secondary:** `secondary_container` background with `on_secondary_container` text. Subtle and sophisticated.
*   **Tertiary:** Transparent background. Use `label-md` uppercase with `primary` color for a "Text-Only" but high-intent action.

### Input Fields
*   **The "Soft Input":** Text inputs should not have a bottom line or a 4-sided border. Use `surface-container-highest` as a solid background fill with a `md` (0.375rem) radius.
*   **Focus State:** Transition the background to `surface-container-lowest` and apply a 1px "Ghost Border" using the `primary` color at 40% opacity.

### Navigation Sidebars
*   **Desktop Layout:** A fixed `surface-container-high` left-hand column. 
*   **Active States:** Do not use a box around the active nav item. Use a `4px` vertical "pill" of `primary` color to the immediate left of the label, and shift the text weight to `Semi-Bold`.

### Checkboxes
*   **Unchecked:** A circular ring using `outline_variant`.
*   **Checked:** A solid `primary` circle with an `on_primary` checkmark. The task text should transition to `on_surface_variant` with a subtle strikethrough.

---

## 6. Do's and Don'ts

### Do:
*   **Do** use white space as a separator. If in doubt, add `16px` more padding.
*   **Do** use `manrope` for any text larger than 24px to maintain the editorial voice.
*   **Do** use `9999px` (full) rounding for buttons to create a friendly, modern touch against the more structured `xl` cards.

### Don't:
*   **Don't** use pure black (#000000) for text. Use `on_surface` (#191c1e) to maintain a refined, low-contrast look.
*   **Don't** use standard "Drop Shadows" from component libraries. Always use the diffused "Cloud Shadow" formula.
*   **Don't** use dividers to separate task metadata (e.g., date vs. priority). Use `body-sm` typography and `12px` horizontal gaps instead.