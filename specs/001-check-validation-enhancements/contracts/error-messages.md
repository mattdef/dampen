# Error Message Formats

This document defines the standard error message formats for the enhanced `gravity check` command.

## Unknown Attribute

**Format**:
```
Error: Unknown attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}
  Did you mean '{suggestion}'? (distance: {n})
```

**Example**:
```
Error: Unknown attribute 'on_clik' for button in ui/main.gravity:10:5
  Did you mean 'on_click'? (distance: 1)
```

**Conditions**:
- Attribute not in widget's valid attribute set
- Suggestion shown if Levenshtein distance <= 3

---

## Missing Required Attribute

**Format**:
```
Error: Missing required attribute '{attr}' for widget '{widget}' in {file}:{line}:{col}
```

**Example**:
```
Error: Missing required attribute 'value' for Text in ui/view.gravity:3:2
```

**Conditions**:
- Required attribute not present on widget
- List of required attributes per widget type

---

## Unknown Handler

**Format**:
```
Error: Unknown handler '{handler}' in {file}:{line}:{col}
  Available handlers: {handler1}, {handler2}, ... ({count} total)
```

**Example**:
```
Error: Unknown handler 'incremnt' in ui/app.gravity:15:8
  Available handlers: increment, decrement, setValue (3 total)
```

**Conditions**:
- Handler not in provided registry
- All available handlers listed (truncated if > 5)

---

## Invalid Binding Field

**Format**:
```
Error: Invalid binding field '{field}' in {file}:{line}:{col}
  Available fields: {field1}, {field2}, ... ({count} total)
```

**Example**:
```
Error: Invalid binding field 'user.nme' in ui/profile.gravity:7:12
  Available fields: user.name, user.email, count, enabled (4 total)
```

**Conditions**:
- Binding path segment not in model
- Top-level or nested field validation

---

## Duplicate Radio Value

**Format**:
```
Error: Duplicate radio value '{value}' in group '{group}' at {file}:{line}:{col}
  First occurrence: {file}:{first_line}:{first_col}
```

**Example**:
```
Error: Duplicate radio value 'option1' in group 'size' at ui/form.gravity:25:9
  First occurrence: ui/form.gravity:20:5
```

**Conditions**:
- Multiple radio buttons in same group have identical value
- Shows both locations

---

## Inconsistent Radio Handlers

**Format**:
```
Error: Radio group '{group}' has inconsistent on_select handlers in {file}:{line}:{col}
  Found handlers: {handler1}, {handler2}
```

**Example**:
```
Error: Radio group 'size' has inconsistent on_select handlers in ui/form.gravity:25:9
  Found handlers: select_small, select_medium
```

**Conditions**:
- Radio buttons in same group have different on_select handlers

---

## Invalid Theme Property

**Format**:
```
Error: Invalid theme property '{property}' in theme '{theme}': {message}
  Valid properties: {prop1}, {prop2}, ...
```

**Example**:
```
Error: Invalid theme property 'font_siz' in theme 'custom': Unknown property
  Valid properties: font_family, font_size, font_weight, ...
```

**Conditions**:
- Property name not in valid theme property set

---

## Theme Circular Dependency

**Format**:
```
Error: Theme '{theme}' has circular dependency: {cycle}
```

**Example**:
```
Error: Theme 'theme_a' has circular dependency: theme_a -> theme_b -> theme_a
```

**Conditions**:
- Theme inheritance chain contains a cycle

---

## CLI Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Validation passed (no errors) |
| 1 | Validation failed (one or more errors) |
| 2 | Invalid arguments or missing files |

---

## Strict Mode

When `--strict` flag is used, all validation issues (including suggestions) cause exit code 1.

**Without --strict**:
```
$ gravity check
Found 3 error(s):
  Error: Unknown attribute 'on_clik' for button in ui/main.gravity:10:5
✓ Validation completed (warnings treated as non-blocking)
```

**With --strict**:
```
$ gravity check --strict
Found 3 error(s):
  Error: Unknown attribute 'on_clik' for button in ui/main.gravity:10:5
✗ Validation failed (strict mode: warnings treated as errors)
```
