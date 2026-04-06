import { Switch as SwitchPrimitive } from "radix-ui"
import { cn } from "@/lib/utils"

import s from "@styles/ui/Nebula/switch.module.scss"

function Switch({
  className,
  size = "default",
  ...props
}: React.ComponentProps<typeof SwitchPrimitive.Root> & {
  size?: "sm" | "default" | "lg" | "xl"
}) {
  return (
    <SwitchPrimitive.Root
      data-component="switch"
      data-size={size}
      className={cn(
        s.switch,
        className
      )}
      {...props}
    >
      <SwitchPrimitive.Thumb
        data-component="switch-thumb"
        className={s.switchThumb}
      />
    </SwitchPrimitive.Root>
  )
}
export { Switch }