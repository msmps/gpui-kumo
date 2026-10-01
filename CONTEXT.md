# GPUI design system

Shared language for a design system built on GPUI. This glossary names concepts; implementation guidance lives in `docs/`.

## Language

### Foundations

**Primitive**:
A building block supplied by GPUI for composition, layout, painting, or interaction.
_Avoid_: Widget (when referring to a framework building block)

**Component**:
A reusable design-system unit with a named purpose, appearance, and behavior contract.
_Avoid_: Primitive (when referring to a design-system unit)

**View**:
A renderable owner of durable UI state.
_Avoid_: Component (when the distinction between state ownership and presentation matters)

**Design token**:
A named design value used consistently across components.
_Avoid_: Constant (when referring to the design meaning of a value)

**Semantic token**:
A design token named for its purpose, such as surface, muted text, or focus indicator.
_Avoid_: Palette value (when referring to a role rather than a raw color)

**Theme**:
A coherent assignment of design-token values for an appearance.
_Avoid_: Stylesheet

**Density**:
A coordinated level of spacing and control dimensions for an interface.
_Avoid_: Size (when referring to interface-wide compactness)

### Component contracts

**Variant**:
A component's intended visual treatment, such as primary or quiet.
_Avoid_: State (for a persistent presentation choice)

**Size**:
A component's named dimension choice, including its associated spacing and typography.
_Avoid_: Density (for a single component's dimension choice)

**Interaction state**:
A transient condition arising from input, such as hover, press, or focus.
_Avoid_: Variant

**Value state**:
A condition representing a component's current value, such as checked, selected, or expanded.
_Avoid_: Active (when referring to selection or a value)

**Availability**:
Whether a control currently accepts activation.
_Avoid_: Disabled appearance (when referring to behavior)

**Activation**:
The user's invocation of a control's primary operation through pointer, keyboard, or assistive technology.
_Avoid_: Click (when referring to an operation independent of input device)

**Slot**:
A named position for caller-supplied component content, such as a leading icon or trailing accessory.
_Avoid_: Child (when the position has a specific contract)

### Identity and behavior

**Element identity**:
The identity of a logical rendered element across frames and within its ancestor scope.
_Avoid_: Label (when referring to identity rather than visible content)

**Focus target**:
The control or composite that receives keyboard input.
_Avoid_: Selection (when referring to keyboard focus)

**Overlay**:
Content presented above surrounding content, such as a popover, menu, or dialog.
_Avoid_: Modal (unless the overlay restricts interaction with surrounding content)

**Accessible name**:
The name assistive technology uses to identify a control.
_Avoid_: Tooltip (when referring to the control's name)
