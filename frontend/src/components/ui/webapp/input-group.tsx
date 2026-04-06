import s from "@styles/ui/Nebula/inputgroup.module.scss"
import { cva, type VariantProps } from "class-variance-authority"
import { Button, Input, Textarea } from "../webappUI"
import { cn } from "@/lib/utils"

function InputGroup({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-component="input-group"
      role="group"
      className={cn(
        s.inputGroup,
        className
      )}
      {...props}
    />
  )
}

interface inputGroupAddonVariants {
    align: "inline-start" | "inline-end" | "block-start" | "block-end"
}

function InputGroupAddon({
  className,
  align = "inline-start",
  ...props
}: React.ComponentProps<"div"> & inputGroupAddonVariants) {
  return (
    <div
      role="group"
      data-component="input-group-addon"
      data-align={align}
      className={cn(s.inputGroupAddon, className)}
      onClick={(e) => {
        if ((e.target as HTMLElement).closest("button")) {
          return
        }
        e.currentTarget.parentElement?.querySelector("input")?.focus()
      }}
      {...props}
    />
  )
}

const inputGroupButtonVariants = cva(
  s.inputGroupButton,
  {
    variants: {
      size: {
        xs: s.sizeXs,
        sm: "",
        "icon-xs": s.sizeIconXs,
        "icon-sm": s.sizeIconSm,
      },
    },
    defaultVariants: {
      size: "xs",
    },
  }
)

function InputGroupButton({
  className,
  type = "button",
  variant = "ghost",
  size = "xs",
  ...props
}: Omit<React.ComponentProps<typeof Button>, "size"> &
  VariantProps<typeof inputGroupButtonVariants>) {
  return (
    <Button
      type={type}
      data-size={size}
      variant={variant}
      size={size}
      className={cn(inputGroupButtonVariants({ size }), className)}
      {...props}
    />
  )
}

function InputGroupText({ className, ...props }: React.ComponentProps<"span">) {
  return (
    <span
      className={cn(
        s.inputGroupText,
        className
      )}
      {...props}
    />
  )
}

function InputGroupInput({
  className,
  ...props
}: React.ComponentProps<"input">) {
  return (
    <Input
      data-component="input-group-control"
      className={cn(s.inputGroupInput, className)}
      {...props}
    />
  )
}

function InputGroupTextarea({
  className,
  ...props
}: React.ComponentProps<"textarea">) {
  return (
    <Textarea
      data-component="input-group-control"
      className={cn(s.inputGroupTextarea, className)}
      {...props}
    />
  )
}

export {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupText,
  InputGroupInput,
  InputGroupTextarea,
}
