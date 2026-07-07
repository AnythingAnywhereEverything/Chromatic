import { Label as LabelPrimitive } from "radix-ui"

import { cn } from "@/lib/utils"

import s from "@styles/ui/chromatic/label.module.scss"

function Label({
  className,
  ...props
}: React.ComponentProps<typeof LabelPrimitive.Root>) {
  return (
    <LabelPrimitive.Root
      data-component="label"
      className={cn(
        s.label,
        className
      )}
      {...props}
    />
  )
}

export { Label }
