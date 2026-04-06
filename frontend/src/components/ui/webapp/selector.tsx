import { Select as SelectPrimitive } from "radix-ui"
import { cn } from "@/lib/utils"
import { Icon } from "./icon"

import s from "@styles/ui/Nebula/select.module.scss";

function Select({
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Root>) {
  return <SelectPrimitive.Root data-component="select" {...props} />
}
function SelectGroup({
  className,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Group>) {
  return (
    <SelectPrimitive.Group
      data-component="select-group"
      className={cn(s.selectGroup, className)}
      {...props}
    />
  )
}
function SelectValue({
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Value>) {
  return <SelectPrimitive.Value data-component="select-value" {...props} />
}
function SelectTrigger({
  className,
  size = "default",
  children,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Trigger> & {
  size?: "sm" | "default"
}) {
  return (
    <SelectPrimitive.Trigger
      data-component="select-trigger"
      data-size={size}
      className={cn(
        s.selectTrigger,
        className
      )}
      {...props}
    >
      {children}
      <SelectPrimitive.Icon asChild>
        <Icon className={s.chevron} value="" />
      </SelectPrimitive.Icon>
    </SelectPrimitive.Trigger>
  )
}
function SelectContent({
  className,
  children,
  position = "item-aligned",
  align = "center",
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Content>) {
  return (
    <SelectPrimitive.Portal>
      <SelectPrimitive.Content
        data-component="select-content"
        data-align-trigger={position === "item-aligned"}
        className={cn(
            s.selectContent,
            position === "popper" && s.popper,
            className )}
        position={position}
        align={align}
        {...props}
      >
        <SelectScrollUpButton />
            <SelectPrimitive.Viewport
                data-position={position}
                    className={cn(
                        s.viewport,
                        position === "popper" && ""
                    )}
                >
                {children}
            </SelectPrimitive.Viewport>
        <SelectScrollDownButton />
      </SelectPrimitive.Content>
    </SelectPrimitive.Portal>
  )
}
function SelectLabel({
  className,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Label>) {
  return (
    <SelectPrimitive.Label
      data-component="select-label"
      className={cn(s.selectLabel, className)}
      {...props}
    />
  )
}
function SelectItem({
  className,
  children,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Item>) {
  return (
    <SelectPrimitive.Item
      data-component="select-item"
      className={cn(
        s.selectItem,
        className
      )}
      {...props}
    >
      <span className={s.indicator}>
        <SelectPrimitive.ItemIndicator>
          <Icon value="" style={{pointerEvents: "none"}}/>
        </SelectPrimitive.ItemIndicator>
      </span>
      <SelectPrimitive.ItemText>{children}</SelectPrimitive.ItemText>
    </SelectPrimitive.Item>
  )
}
function SelectSeparator({
  className,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Separator>) {
  return (
    <SelectPrimitive.Separator
      data-component="select-separator"
      className={cn(s.selectSeparator, className)}
      {...props}
    />
  )
}
function SelectScrollUpButton({
  className,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.ScrollUpButton>) {
  return (
    <SelectPrimitive.ScrollUpButton
      data-component="select-scroll-up-button"
      className={cn(s.scrollbutton, className)}
      {...props}
    >
      <Icon value=""/>
    </SelectPrimitive.ScrollUpButton>
  )
}
function SelectScrollDownButton({
  className,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.ScrollDownButton>) {
  return (
    <SelectPrimitive.ScrollDownButton
      data-component="select-scroll-down-button"
      className={cn(s.scrollbutton, className)}
      {...props}
    >
      <Icon value=""/>
    </SelectPrimitive.ScrollDownButton>
  )
}
export {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectScrollDownButton,
  SelectScrollUpButton,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
}
