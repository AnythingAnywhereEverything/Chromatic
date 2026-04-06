import { Checkbox as CheckboxPrimitive } from "radix-ui"
import { cn } from "@/lib/utils"
import { Icon } from "./icon"

import s from "@styles/ui/Nebula/checkbox.module.scss"

function Checkbox({
  className,
  ...props
}: React.ComponentProps<typeof CheckboxPrimitive.Root>) {
  return (
    <CheckboxPrimitive.Root
      data-component="checkbox"
      className={cn(
        s.checkbox,
        className
      )}
      {...props}
    >
      <CheckboxPrimitive.Indicator
        data-component="checkbox-indicator"
        className={s.indicator}
      >
        <Icon value="" style={{fontSize: "var(--text-extra-small)"}} />
      </CheckboxPrimitive.Indicator>
    </CheckboxPrimitive.Root>
  )
}
export { Checkbox }