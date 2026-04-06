import { cn } from "@/lib/utils"
import s from "@styles/ui/Nebula/textarea.module.scss"

function Textarea({ className, ...props }: React.ComponentProps<"textarea">) {
  return (
    <textarea
      data-component="textarea"
      className={cn(
        s.inputField,
        className
      )}
      {...props}
    />
  )
}
export { Textarea }
