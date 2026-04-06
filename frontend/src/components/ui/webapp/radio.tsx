import { RadioGroup as RadioGroupPrimitive } from "radix-ui"

import { cn } from "@/lib/utils"
import { Icon } from "./icon"

import s from "@styles/ui/Nebula/radio.module.scss"

function RadioGroup({
  className,
  ...props
}: React.ComponentProps<typeof RadioGroupPrimitive.Root>) {
  return (
        <RadioGroupPrimitive.Root
            data-component="radio-group"
            className={cn(s.radioGroup, className)}
            {...props}
        />
  )
}

function RadioGroupItem({
  className,
  ...props
}: React.ComponentProps<typeof RadioGroupPrimitive.Item>) {
  return (
    <RadioGroupPrimitive.Item
      data-component="radio-group-item"
      className={cn(
        s.radioItem,
        className
      )}
      {...props}
    >
      <RadioGroupPrimitive.Indicator
        data-component="radio-group-indicator"
        className={s.radioIndicator}
      >
        <Icon className={s.icon} value="" />
      </RadioGroupPrimitive.Indicator>
    </RadioGroupPrimitive.Item>
  )
}

export { RadioGroup, RadioGroupItem }
