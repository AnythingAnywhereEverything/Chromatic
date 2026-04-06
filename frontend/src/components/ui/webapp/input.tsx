import { cn } from "@/lib/utils"
import s from "@styles/ui/Nebula/input.module.scss"

function Input({ className, type, ...props }: React.ComponentProps<"input">) {
  return (
    <input
      type={type}
      data-component="input"
      className={cn(
        s.input,
        className
      )}
      {...props}
    />
  )
}

export { Input }
