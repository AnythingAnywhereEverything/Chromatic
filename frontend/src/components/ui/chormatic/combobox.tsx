import { Combobox as ComboboxPrimitive } from "@base-ui/react"

import { cn } from "@lib/utils"
import { Button } from "@components/ui/chormatic/button"
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
} from "@components/ui/chormatic/input-group"
import React from "react"
import { Icon } from "./icon"

import s from "@styles/ui/Chormatic/combobox.module.scss"

const Combobox = ComboboxPrimitive.Root

function ComboboxValue({ ...props }: ComboboxPrimitive.Value.Props) {
  return <ComboboxPrimitive.Value data-component="combobox-value" {...props} />
}

function ComboboxTrigger({
  className,
  children,
  ...props
}: ComboboxPrimitive.Trigger.Props) {
  return (
    <ComboboxPrimitive.Trigger
      data-component="combobox-trigger"
      className={cn(typeof className === "string" ? className : undefined)}
      {...props}
    >
      {children}
      <Icon className={s.comboboxTriggerIcon} value="" />
    </ComboboxPrimitive.Trigger>
  )
}

function ComboboxClear({ className, ...props }: ComboboxPrimitive.Clear.Props) {
  return (
    <ComboboxPrimitive.Clear data-component="combobox-clear" className={cn(typeof className === "string" ? className : undefined)} 
    {...props} render={
        <InputGroupButton variant="ghost" size="icon-xs" asChild>
            <Icon style={{pointerEvents: "none"}} value="" />
        </InputGroupButton>} />
  )
}

function ComboboxInput({
  className,
  children,
  disabled = false,
  showTrigger = true,
  showClear = false,
  ...props
}: ComboboxPrimitive.Input.Props & {
  showTrigger?: boolean
  showClear?: boolean
}) {
  return (
    <InputGroup className={cn(s.comboInput, typeof className === "string" ? className : undefined)}>
      <ComboboxPrimitive.Input
        render={<InputGroupInput disabled={disabled} />}
        {...props}
      />
      <InputGroupAddon align="inline-end">
        {showTrigger && (
          <InputGroupButton
            size="icon-xs"
            variant="ghost"
            data-component="input-group-button"
            className={s.inputAddon}
            disabled={disabled}
            asChild
          >
            <ComboboxTrigger />
          </InputGroupButton>
        )}
        {showClear && <ComboboxClear disabled={disabled} />}
      </InputGroupAddon>
      {children}
    </InputGroup>
  )
}

function ComboboxContent({
  className,
  side = "bottom",
  sideOffset = 6,
  align = "start",
  alignOffset = 0,
  anchor,
  keepMount,
  ...props
}: ComboboxPrimitive.Popup.Props & {keepMount?:boolean} &
  Pick<
    ComboboxPrimitive.Positioner.Props,
    "side" | "align" | "sideOffset" | "alignOffset" | "anchor"
  >) {
  return (
    <ComboboxPrimitive.Portal keepMounted={keepMount}>
      <ComboboxPrimitive.Positioner
        side={side}
        sideOffset={sideOffset}
        align={align}
        alignOffset={alignOffset}
        anchor={anchor}
        className={s.comboPositioner}
      >
        <ComboboxPrimitive.Popup
          data-component="combobox-content"
          data-chips={!!anchor}
          className={cn(
            s.comboContent,
            typeof className === "string" ? className : undefined )}
          {...props}
        />
      </ComboboxPrimitive.Positioner>
    </ComboboxPrimitive.Portal>
  )
}

function ComboboxList({ className, ...props }: ComboboxPrimitive.List.Props) {
  return (
    <ComboboxPrimitive.List
      data-component="combobox-list"
      className={cn(
        s.comboList,
        typeof className === "string" ? className : undefined
      )}
      {...props}
    />
  )
}

function ComboboxItem({
  className,
  children,
  ...props
}: ComboboxPrimitive.Item.Props) {
  return (
    <ComboboxPrimitive.Item
      data-component="combobox-item"
      className={cn(
        s.comboItem,
        typeof className === "string" ? className : undefined
      )}
      {...props}
    >
      {children}
        <ComboboxPrimitive.ItemIndicator render={
        <span className={s.comboItemIndicator}>
            <Icon style={{pointerEvents: "none"}} value=""/>
        </span>} />
    </ComboboxPrimitive.Item>
  )
}

function ComboboxGroup({ className, ...props }: ComboboxPrimitive.Group.Props) {
  return (
    <ComboboxPrimitive.Group
      data-component="combobox-group"
      className={cn(typeof className === "string" ? className : undefined)}
      {...props}
    />
  )
}

function ComboboxLabel({
  className,
  ...props
}: ComboboxPrimitive.GroupLabel.Props) {
  return (
    <ComboboxPrimitive.GroupLabel
      data-component="combobox-label"
      className={cn(s.comboLabel, typeof className === "string" ? className : undefined)}
      {...props}
    />
  )
}

function ComboboxCollection({ ...props }: ComboboxPrimitive.Collection.Props) {
  return (
    <ComboboxPrimitive.Collection data-component="combobox-collection" {...props} />
  )
}

function ComboboxEmpty({ className, ...props }: ComboboxPrimitive.Empty.Props) {
  return (
    <ComboboxPrimitive.Empty
      data-component="combobox-empty"
      className={cn(s.comboEmpty, typeof className === "string" ? className : undefined)}
      {...props}
    />
  )
}

function ComboboxSeparator({
  className,
  ...props
}: ComboboxPrimitive.Separator.Props) {
  return (
    <ComboboxPrimitive.Separator
      data-component="combobox-separator"
      className={cn(s.comboSeparator, typeof className === "string" ? className : undefined)}
      {...props}
    />
  )
}

function ComboboxChips({
  className,
  ...props
}: React.ComponentPropsWithRef<typeof ComboboxPrimitive.Chips> &
  ComboboxPrimitive.Chips.Props) {
  return (
    <ComboboxPrimitive.Chips
      data-component="combobox-chips"
      className={cn(
        s.comboChips,
        typeof className === "string" ? className : undefined)}
      {...props}
    />
  )
}

function ComboboxChip({
  className,
  children,
  showRemove = true,
  ...props
}: ComboboxPrimitive.Chip.Props & {
  showRemove?: boolean
}) {
  return (
    <ComboboxPrimitive.Chip
      data-component="combobox-chip"
      className={cn(
        s.comboChip,
        typeof className === "string" ? className : undefined
      )}
      {...props}
    >
      {children}
      {showRemove && (
        <ComboboxPrimitive.ChipRemove 
            className={s.comboChipRemove}
            data-component="combobox-chip-remove" 
            render={<Button variant="ghost" size="icon-xs">
                <Icon  style={{pointerEvents: "none"}} value="" />
            </Button>} />
      )}
    </ComboboxPrimitive.Chip>
  )
}

function ComboboxChipsInput({
  className,
  ...props
}: ComboboxPrimitive.Input.Props) {
  return (
    <ComboboxPrimitive.Input
      data-component="combobox-chip-input"
      className={cn(
        s.comboInput,
        typeof className === "string" ? className : undefined
      )}
      {...props}
    />
  )
}

function useComboboxAnchor() {
  return React.useRef<HTMLDivElement | null>(null)
}

export {
  Combobox,
  ComboboxInput,
  ComboboxContent,
  ComboboxList,
  ComboboxItem,
  ComboboxGroup,
  ComboboxLabel,
  ComboboxCollection,
  ComboboxEmpty,
  ComboboxSeparator,
  ComboboxChips,
  ComboboxChip,
  ComboboxChipsInput,
  ComboboxTrigger,
  ComboboxValue,
  useComboboxAnchor,
}
