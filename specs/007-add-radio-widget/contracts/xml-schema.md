# Radio Widget XML Schema

**Feature**: 007-add-radio-widget
**Date**: 2026-01-08

## Overview

This document defines the XML schema for the Radio widget, including element structure, attributes, and validation rules.

## Element Definition

### `<radio>` Element

The radio element represents a single radio button option within a radio group.

**Parent Elements**: Any container element (`<column>`, `<row>`, `<stack>`, `<container>`, etc.)

**Child Elements**: None (radio is an atomic widget)

## Attributes

### Required Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `label` | String | Text displayed next to the radio button |
| `value` | String | Internal value for this option |

### Optional Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `selected` | Binding/Option<String> | `None` | Currently selected value |
| `on_select` | Handler | `""` | Handler called on selection change |
| `disabled` | Binding/Bool | `false` | Disable user interaction |
| `id` | String | Auto-generated | Unique identifier for the widget |
| `class` | String | `""` | CSS classes for styling |

### Event Attributes

| Event | Handler | Description |
|-------|---------|-------------|
| `on_select` | `handler(value: String)` | Fired when user selects this radio option |

## XML Schema (XSD-style)

```xml
<xsd:element name="radio">
  <xsd:complexType>
    <xsd:sequence>
      <!-- No child elements -->
    </xsd:sequence>
    
    <xsd:attribute name="label" type="xsd:string" use="required"/>
    <xsd:attribute name="value" type="xsd:string" use="required"/>
    <xsd:attribute name="selected" type="binding" use="optional"/>
    <xsd:attribute name="on_select" type="handler" use="optional"/>
    <xsd:attribute name="disabled" type="binding" use="optional" default="false"/>
    <xsd:attribute name="id" type="xsd:ID" use="optional"/>
    <xsd:attribute name="class" type="xsd:string" use="optional"/>
  </xsd:complexType>
</xsd:element>
```

## Binding Expressions

### `selected` Attribute

Supports binding expressions for the selected value:

- **Static binding**: `selected="small"` - always selects this option
- **Dynamic binding**: `selected={current_size}` - binds to model field
- **Option type**: `selected={size_option}` - binds to `Option<String>` field

### `disabled` Attribute

Supports binding expressions for disabled state:

- **Static**: `disabled="true"` - always disabled
- **Dynamic**: `disabled={!is_premium}` - conditionally disabled

## Complete Examples

### Simple Radio Group

```xml
<column spacing="10">
  <text value="Select your size:"/>
  <radio label="Small" value="small" selected={size} on_select="setSize"/>
  <radio label="Medium" value="medium" selected={size} on_select="setSize"/>
  <radio label="Large" value="large" selected={size} on_select="setSize"/>
</column>
```

### Radio Group with Disabled Option

```xml
<column spacing="10">
  <text value="Choose a shipping method:"/>
  <radio label="Standard (5-7 days)" 
         value="standard" 
         selected={shipping} 
         on_select="setShipping"/>
  <radio label="Express (2-3 days)" 
         value="express" 
         selected={shipping} 
         on_select="setShipping"/>
  <radio label="Overnight" 
         value="overnight" 
         selected={shipping} 
         on_select="setShipping"
         disabled={!is_premium_member}/>
</column>
```

### Styled Radio Group

```xml
<column spacing="8" classes="radio-group">
  <text value="Payment Method:" classes="section-header"/>
  <radio label="Credit Card" 
         value="credit_card" 
         selected={payment_method} 
         on_select="setPayment"
         class="payment-option"/>
  <radio label="PayPal" 
         value="paypal" 
         selected={payment_method} 
         on_select="setPayment"
         class="payment-option"/>
  <radio label="Bank Transfer" 
         value="bank" 
         selected={payment_method} 
         on_select="setPayment"
         class="payment-option"/>
</column>
```

## Validation Rules

### At Parse Time

1. Required attributes (`label`, `value`) must be present
2. Attribute values must be valid UTF-8 strings
3. Handler names must match identifier pattern `[a-zA-Z_][a-zA-Z0-9_]*`

### At Runtime

1. If `selected` is bound to a value, it should match one of the radio `value`s in the group
2. Disabled radios don't respond to clicks
3. Changing `selected` binding programmatically updates the UI

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| Missing required attribute | `label` or `value` not specified | Add the missing attribute |
| Invalid handler name | Handler contains invalid characters | Use alphanumeric and underscores only |
| Type mismatch | `selected` binding type doesn't match `Option<String>` | Ensure model field is `Option<String>` |
